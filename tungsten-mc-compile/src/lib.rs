#![allow(warnings)]

use tungsten_wg::{CompilerConfig, compile};

pub mod config_load;
pub mod parse;
pub mod transform_spmt;

pub fn run_inline_generation(path: &str, base: &str) -> String {
    // 1. Load Raw Minecraft Data
    let mut data = config_load::MinecraftDataRaw::new();
    config_load::load_all_configs(&mut data, base, None);
    config_load::load_all_configs(&mut data, path, None);

    // 2. Parse Data
    let arena = bumpalo::Bump::with_capacity(1 * 1024 * 1024);
    let mut mcdata = parse::MinecraftData::new(&arena, &data, 16);
    mcdata.parse_from_raw();

    let noise_generator = mcdata
        .noise_settings
        .get("minecraft:overworld")
        .expect("Could not find minecraft:overworld settings");

    // 3. Transform to SPMT Program
    let transform_arena = bumpalo::Bump::with_capacity(1 * 1024 * 1024);
    let transformer = transform_spmt::Transformer::new(&transform_arena);
    let program = transformer.transform(noise_generator);

    // 4. Delegate to the new compiler library
    let config = CompilerConfig::new()
        .with_rcl(true)
        .rcl_module_names("density_function", "orchestration");

    let compiled_output =
        compile(&program, &config).expect("Failed to compile SPMT program into target backends");

    compiled_output.rcl.expect("RCL output was not generated")
}
