//! `CompilerSession`: the in-memory path the mod drives over JNI.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde_json::{Value, json};
use tungsten_mc_compile::config_load::walk_datapack_folder;
use tungsten_mc_compile::{
    CompileError, CompilerSession, JavaOutput, Layout, MinecraftCompilerConfig, ResourceKind,
    contract, run_generation,
};

fn vanilla_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("vanilla_worldgen")
}

fn jjthunder_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../JJThunderToTheMax")
}

fn feed_folder(session: &mut CompilerSession, root: &Path) {
    walk_datapack_folder(root, &mut |id, bytes| {
        session.load_resource(id, bytes);
    })
    .unwrap();
}

fn session(packs: &[PathBuf], root: &str) -> CompilerSession {
    let mut session = CompilerSession::new();
    for pack in packs {
        feed_folder(&mut session, pack);
    }
    session.set_root(root);
    session
}

fn vanilla_output() -> &'static JavaOutput {
    static OUTPUT: OnceLock<JavaOutput> = OnceLock::new();
    OUTPUT.get_or_init(|| session(&[vanilla_dir()], "minecraft:overworld").compile().unwrap())
}

fn jjthunder_output() -> &'static JavaOutput {
    static OUTPUT: OnceLock<JavaOutput> = OnceLock::new();
    OUTPUT.get_or_init(|| {
        session(&[vanilla_dir(), jjthunder_dir()], "minecraft:overworld").compile().unwrap()
    })
}

fn source(output: &JavaOutput) -> &str {
    assert_eq!(output.units.len(), 1);
    &output.units[0].source
}

/// Vanilla with one density function file replaced.
fn vanilla_with_density_function(id: &str, json: &str) -> Result<JavaOutput, CompileError> {
    let mut session = session(&[vanilla_dir()], "minecraft:overworld");
    session.load_resource(id, json.as_bytes());
    session.compile()
}

#[test]
fn vanilla_matches_cli_path() {
    // The CLI path runs every backend and needs the same deep stack the session gives itself.
    let cli = std::thread::Builder::new()
        .stack_size(16 * 1024 * 1024)
        .spawn(|| run_generation(&MinecraftCompilerConfig::new()).java_code.unwrap())
        .unwrap()
        .join()
        .unwrap();
    let output = vanilla_output();
    assert!(source(output) == cli, "session Java differs from the CLI path");
    assert_eq!(output.units[0].name, contract::qualified_class_name());
}

/// The vanilla overworld noise settings with `edit` applied to the parsed JSON.
fn overworld_settings_with(edit: impl FnOnce(&mut Value)) -> String {
    let path = vanilla_dir().join("data/minecraft/worldgen/noise_settings/overworld.json");
    let mut settings: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    edit(&mut settings);
    settings.to_string()
}

fn vanilla_with_settings(json: &str) -> Result<JavaOutput, CompileError> {
    let mut session = session(&[vanilla_dir()], "minecraft:overworld");
    session.load_resource("minecraft:worldgen/noise_settings/overworld.json", json.as_bytes());
    session.compile()
}

#[test]
fn vanilla_keeps_the_contract() {
    // compile() already checks this; the test pins it to the published constants
    let output = vanilla_output();
    assert_eq!(contract::check(source(output)), Ok(()));
    assert_eq!(output.layout, Layout { min_y: -64, height: 384 });
}

#[test]
fn constant_routes_keep_the_contract() {
    // every route a constant, aquifers and ore veins off: nothing to sample, everything still exists
    let json = overworld_settings_with(|s| {
        for (_, route) in s["noise_router"].as_object_mut().unwrap() {
            *route = json!(0.0);
        }
        s["aquifers_enabled"] = json!(false);
        s["ore_veins_enabled"] = json!(false);
    });
    let output = vanilla_with_settings(&json).unwrap();
    assert_eq!(contract::check(source(&output)), Ok(()));
}

#[test]
fn shared_and_referenced_routes_keep_the_contract() {
    // routes that are references, several to the same function, must not merge or take its name
    let json = overworld_settings_with(|s| {
        let router = &mut s["noise_router"];
        for route in ["vein_toggle", "vein_ridged", "vein_gap"] {
            router[route] = json!("minecraft:overworld/continents");
        }
        router["final_density"] = json!("minecraft:overworld/base_3d_noise");
    });
    let output = vanilla_with_settings(&json).unwrap();
    assert_eq!(contract::check(source(&output)), Ok(()));
}

#[test]
fn layout_follows_the_noise_settings() {
    let json = overworld_settings_with(|s| {
        s["noise"]["min_y"] = json!(0);
        s["noise"]["height"] = json!(256);
    });
    let output = vanilla_with_settings(&json).unwrap();
    assert_eq!(output.layout, Layout { min_y: 0, height: 256 });
    assert_eq!(contract::check(source(&output)), Ok(()));
}

#[test]
fn jjthunder_compiles_and_reports_its_ids() {
    let mut vanilla_ids = BTreeSet::new();
    walk_datapack_folder(&vanilla_dir(), &mut |id, _| {
        vanilla_ids.insert(id.to_string());
    })
    .unwrap();

    let mut jj_only = BTreeSet::new();
    walk_datapack_folder(&jjthunder_dir(), &mut |id, _| {
        if !vanilla_ids.contains(id) {
            if let Some(name) = id
                .strip_prefix("minecraft:worldgen/density_function/")
                .and_then(|p| p.strip_suffix(".json"))
            {
                jj_only.insert(format!("minecraft:{}", name));
            }
        }
    })
    .unwrap();

    let reachable: BTreeSet<String> = jjthunder_output()
        .reachable
        .iter()
        .filter(|r| r.kind == ResourceKind::DensityFunction)
        .map(|r| r.id.clone())
        .collect();
    assert!(
        !jj_only.is_disjoint(&reachable),
        "no JJThunder-only density function is reachable: {:?}",
        reachable
    );
    assert_ne!(source(jjthunder_output()), source(vanilla_output()));
}

#[test]
fn vanilla_reachable_ids() {
    let reachable = &vanilla_output().reachable;
    let has = |kind, id: &str| reachable.iter().any(|r| r.kind == kind && r.id == id);
    assert!(has(ResourceKind::DensityFunction, "minecraft:overworld/continents"));
    assert!(has(ResourceKind::Noise, "minecraft:continentalness"));
    // nether-only worldgen isn't reached from the overworld
    assert!(!has(ResourceKind::Noise, "minecraft:nether_wart"));
    assert!(reachable.windows(2).all(|w| w[0] < w[1]), "reachable must be sorted and unique");
}

#[test]
fn unknown_type_is_unsupported() {
    let err = vanilla_with_density_function(
        "minecraft:worldgen/density_function/overworld/continents.json",
        r#"{"type": "minecraft:does_not_exist", "argument": 1.0}"#,
    )
    .unwrap_err();
    let CompileError::Unsupported { kind, id, message } = err else {
        panic!("expected Unsupported, got {:?}", err);
    };
    assert_eq!(kind, ResourceKind::DensityFunction);
    assert_eq!(id, "minecraft:overworld/continents");
    assert!(message.contains("minecraft:does_not_exist"), "{}", message);
}

#[test]
fn missing_reference_is_unsupported() {
    let err = vanilla_with_density_function(
        "minecraft:worldgen/density_function/overworld/continents.json",
        r#""minecraft:not/there""#,
    )
    .unwrap_err();
    let CompileError::Unsupported { kind, id, message } = err else {
        panic!("expected Unsupported, got {:?}", err);
    };
    assert_eq!(kind, ResourceKind::DensityFunction);
    assert_eq!(id, "minecraft:not/there");
    assert!(message.contains("minecraft:overworld/continents"), "{}", message);
}

#[test]
fn missing_noise_and_bad_json_are_unsupported() {
    let err = vanilla_with_density_function(
        "minecraft:worldgen/density_function/overworld/continents.json",
        r#"{"type": "minecraft:noise", "noise": "minecraft:not_a_noise"}"#,
    )
    .unwrap_err();
    assert!(
        matches!(&err, CompileError::Unsupported { kind: ResourceKind::Noise, id, .. } if id == "minecraft:not_a_noise"),
        "{:?}",
        err
    );

    let err = vanilla_with_density_function(
        "minecraft:worldgen/density_function/overworld/continents.json",
        "{ not json",
    )
    .unwrap_err();
    assert!(
        matches!(&err, CompileError::Unsupported { id, .. } if id == "minecraft:overworld/continents"),
        "{:?}",
        err
    );
}

#[test]
fn invalid_values_are_unsupported() {
    // Each of these used to panic in the transform.
    for (json, expected) in [
        (
            r#"{"type": "minecraft:weird_scaled_sampler", "input": 0.0, "noise": "minecraft:cave_layer", "rarity_value_mapper": "bogus"}"#,
            "bogus",
        ),
        (
            r#"{"type": "minecraft:find_top_surface", "density": 0.0, "upper_bound": 0.0, "lower_bound": 0, "cell_height": 0}"#,
            "cell_height",
        ),
        (r#"{"type": "minecraft:spline", "spline": {"coordinate": 0.0, "points": []}}"#, "at least one point"),
    ] {
        let err = vanilla_with_density_function(
            "minecraft:worldgen/density_function/overworld/continents.json",
            json,
        )
        .unwrap_err();
        assert!(
            matches!(&err, CompileError::Unsupported { id, message, .. }
                if id == "minecraft:overworld/continents" && message.contains(expected)),
            "{:?}",
            err
        );
    }
}

#[test]
fn root_errors() {
    let err = session(&[vanilla_dir()], "minecraft:nope").compile().unwrap_err();
    assert!(
        matches!(&err, CompileError::Unsupported { kind: ResourceKind::NoiseSettings, id, .. } if id == "minecraft:nope"),
        "{:?}",
        err
    );
    assert!(matches!(CompilerSession::new().compile(), Err(CompileError::Internal(_))));
}

#[test]
fn nothing_bundled() {
    let err = session(&[], "minecraft:overworld").compile().unwrap_err();
    assert!(matches!(err, CompileError::Unsupported { kind: ResourceKind::NoiseSettings, .. }));
}

#[test]
fn amplified_differs_from_overworld() {
    let amplified = session(&[vanilla_dir()], "minecraft:amplified").compile().unwrap();
    assert_ne!(source(&amplified), source(vanilla_output()));
}

/// SPEC §7.6.1: the generated code must stay in the subset of Java that Janino accepts.
fn assert_janino_dialect(source: &str) {
    for (number, line) in source.lines().enumerate() {
        let code = line.trim();
        if code.starts_with("//") || code.starts_with("/*") || code.starts_with('*') {
            continue;
        }
        let code = code.split("//").next().unwrap();
        let fail = |what: &str| panic!("{} on line {}: {}", what, number + 1, line);
        for (needle, what) in [
            ("->", "lambda or switch arrow"),
            ("::", "method reference"),
            ("\"\"\"", "text block"),
            ("@", "annotation"),
        ] {
            if code.contains(needle) {
                fail(what);
            }
        }
        let words: Vec<&str> = code
            .split(|c: char| !(c.is_alphanumeric() || c == '_' || c == '$'))
            .collect();
        for word in ["var", "record", "yield", "instanceof"] {
            if words.contains(&word) {
                fail(word);
            }
        }
        // generics: a type name directly followed by `<`
        for (i, _) in code.match_indices('<') {
            let ident: String = code[..i]
                .chars()
                .rev()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            if ident.chars().last().is_some_and(|c| c.is_ascii_uppercase()) {
                fail("generic type");
            }
        }
    }
}

#[test]
fn dialect_vanilla() {
    assert_janino_dialect(source(vanilla_output()));
}

#[test]
fn dialect_jjthunder() {
    assert_janino_dialect(source(jjthunder_output()));
}
