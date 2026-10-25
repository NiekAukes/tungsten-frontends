use std::collections::HashMap;
use std::fs;
use std::path::Path;

use tungsten_mc_compile::parse::model::Density;
use tungsten_mc_compile::{run_generation_from_ast, CompiledOutput, MinecraftCompilerConfig};

use crate::paths::{HarnessConfig, Target};
use crate::{ffi, oracle, reexport};

/// Writes the generated sources of every selected target into
/// `generated_dir`. A source the compiler didn't produce is only an error
/// when a target that needs it is selected.
fn write_generated(config: &HarnessConfig, output: CompiledOutput) {
    let generated_dir = &config.generated_dir;

    let rust_code = output
        .rust_code
        .expect("compiler did not produce Rust output");
    fs::write(generated_dir.join("density.rs"), rust_code).expect("failed to write density.rs");

    if config.has_target(Target::Java) {
        let java_code = output
            .java_code
            .expect("Java target selected but the compiler produced no Java output");
        fs::write(config.java_source(), java_code).expect("failed to write GeneratedWorldgen.java");
    }

    if config.has_target(Target::Cuda) {
        let cuda_density_function = output
            .cuda_density_function
            .expect("CUDA target selected but the compiler produced no CUDA density_function output");
        fs::write(
            generated_dir.join("density_function.cu"),
            cuda_density_function,
        )
        .expect("failed to write density_function.cu");

        let cuda_orchestration = output
            .cuda_orchestration
            .expect("CUDA target selected but the compiler produced no CUDA orchestration output");
        fs::write(generated_dir.join("orchestration.cu"), cuda_orchestration)
            .expect("failed to write orchestration.cu");
    }
}

/// Phase 0: compiles a Minecraft datapack source folder (or just vanilla) into
/// fresh `density.rs` / `GeneratedWorldgen.java` (and, for the CUDA target,
/// `density_function.cu` / `orchestration.cu`) inside `generated_dir`, using tungsten-mc-compile directly as a library (no
/// subprocess, no CLI file-naming quirks to work around).
///
/// If the resulting pipelines disagree, this also runs a
/// shrink-and-test loop that repeatedly regenerates, rebuilds, and reruns
/// the selected pipelines against smaller and smaller candidate `final_density`
/// trees, keeping only the reductions that still reproduce the mismatch.
/// Passing `single_run = true` skips this shrink loop entirely: the source
/// is compiled once and control returns to the caller to build and run the
/// pipelines a single time.
///
/// `helpers.cu` is not produced by this step; it must be supplied separately
/// alongside the generated CUDA sources.
pub fn generate_from_source(
    config: &HarnessConfig,
    source_dir: Option<&Path>,
    chunk_size: usize,
    single_run: bool,
) {
    let generated_dir = &config.generated_dir;
    fs::create_dir_all(generated_dir).expect("failed to create generated_dir");

    let mc_config = MinecraftCompilerConfig::new().with_chunk_size(chunk_size);
    let mc_config = if let Some(source_dir) = source_dir {
        mc_config.with_mod_path(source_dir.to_string_lossy())
    } else {
        mc_config
    };

    println!(
        "Compiling worldgen source into {}...",
        generated_dir.display()
    );
    let arena = bumpalo::Bump::new();
    let ast = tungsten_mc_compile::run_parse(&arena, &mc_config);

    if single_run {
        let output = run_generation_from_ast(&ast);
        write_generated(config, output);
        return;
    }

    let mut ast = ast;
    let initial_density = ast
        .noise_settings
        .get("minecraft:overworld")
        .unwrap()
        .noise_router
        .final_density;

    // Substitutes `final_density` into the AST, regenerates and rebuilds
    // the selected pipelines from scratch, and reports whether they still disagree
    // (i.e. whether the bug still reproduces with this candidate).
    let mut bug_reproduces = |final_density| {
        ast.noise_settings
            .get_mut("minecraft:overworld")
            .unwrap()
            .noise_router
            .final_density = final_density;

        let output = run_generation_from_ast(&ast);
        write_generated(config, output);

        ffi::build_targets(config);
        let result = ffi::run_pipelines(config);
        !oracle::pipelines_agree(&result, config)
    };

    println!("Confirming the mismatch reproduces before shrinking...");
    if !bug_reproduces(initial_density) {
        println!("The selected pipelines already agree; nothing to shrink.");
        return;
    }

    println!("Mismatch confirmed. Shrinking final_density...");
    let mut shrinker = tungsten_mc_compile::shrink::Shrinker::new(&arena, initial_density);
    shrinker.set_density_function_name("final_density".to_string());

    let shrink_log_dir = config.build_dir.join("shrink_log");

    let mut result_cache: HashMap<Density, bool> = HashMap::new();
    let mut iters = 0;
    while let Some(shrunk) = shrinker.shrink() {
        iters += 1;
        if let Some(&reproduces) = result_cache.get(shrunk.get_density()) {
            if reproduces {
                println!("  iteration {iters}: cached (still reproduces)");
                shrinker.mark_last_shrink_successful();
                continue;
            } else {
                println!("  iteration {iters}: cached (no longer reproduces)");
                shrinker.mark_last_shrink_successful();
                continue;
            }
        }
        let reproduces = bug_reproduces(shrunk);
        reexport::save_shrink_iteration(&shrink_log_dir, iters, &shrunk, reproduces);
        if reproduces {
            // Mismatch still reproduces with the smaller tree: keep it.
            shrinker.mark_last_shrink_buggy();
            result_cache.insert(shrunk.get_density(), true);
            println!("  iteration {iters}: kept (still reproduces)");
        } else {
            // Mismatch disappeared: revert and try the next shrink method.
            shrinker.mark_last_shrink_successful();
            result_cache.insert(shrunk.get_density(), false);
            println!("  iteration {iters}: reverted (no longer reproduces)");
        }

        // println!("Total shrink iterations: {}", iters);
        // let bytes = arena.allocated_bytes();
        // println!("Total size of memory used: {}mb", bytes / 1024 / 1024);
    }

    let final_density = shrinker.finish();
    println!("Regenerating minimal reproducing pipeline...");
    bug_reproduces(final_density);

    let final_density_node = match final_density {
        tungsten_mc_compile::parse::model::DensitySource::MultiSamplingDensity {
            density, ..
        } => density,
        tungsten_mc_compile::parse::model::DensitySource::SingleSamplingDensity { density } => {
            density
        }
    };
    println!("Final density function:");
    println!("{}", final_density_node);
}
