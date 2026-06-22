#![allow(warnings)]

use std::path::PathBuf;

use bumpalo::Bump;
use tungsten_wg::{CompiledOutput, CompilerConfig, compile};

pub mod config_load;
pub mod parse;
pub mod transform_spmt;

/// Configuration for the Minecraft-specific parsing and compilation step.
pub struct MinecraftCompilerConfig {
    pub mod_path: Option<String>,
    pub target_dimension: String,
    pub chunk_size: usize,
    pub backend_config: CompilerConfig,
}

impl Default for MinecraftCompilerConfig {
    fn default() -> Self {
        Self {
            mod_path: None,
            target_dimension: "minecraft:overworld".to_string(),
            chunk_size: 16,
            backend_config: CompilerConfig::default().with_rcl(true),
        }
    }
}

impl MinecraftCompilerConfig {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_mod_path(mut self, mod_path: impl Into<String>) -> Self {
        self.mod_path = Some(mod_path.into());
        self
    }

    pub fn with_target_dimension(mut self, dimension: impl Into<String>) -> Self {
        self.target_dimension = dimension.into();
        self
    }

    pub fn with_chunk_size(mut self, size: usize) -> Self {
        self.chunk_size = size;
        self
    }

    pub fn with_backend_config(mut self, config: CompilerConfig) -> Self {
        self.backend_config = config;
        self
    }
}

/// Generates the compiled output based on the provided configuration.
pub fn run_generation(config: &MinecraftCompilerConfig) -> CompiledOutput {
    // 1. Load Raw Minecraft Data
    let mut data = config_load::MinecraftDataRaw::new();

    // base version is always at crate_root/vanilla_worldgen
    let macro_root = env!("CARGO_MANIFEST_DIR");
    let mut folder_path = PathBuf::from(macro_root);
    folder_path.push("vanilla_worldgen");

    // 1. Load configs based on CLI args
    config_load::load_all_configs(&mut data, &folder_path.to_string_lossy(), None);

    if let Some(mod_path) = &config.mod_path {
        config_load::load_all_configs(&mut data, mod_path, None);
    }

    // 2. Parse Data
    let arena = Bump::with_capacity(1 * 1024 * 1024);
    let mut mcdata = parse::MinecraftData::new(&arena, &data, config.chunk_size);
    mcdata.parse_from_raw();

    let noise_generator = mcdata
        .noise_settings
        .get(&config.target_dimension)
        .unwrap_or_else(|| panic!("Could not find {} settings", config.target_dimension));

    // 3. Transform to SPMT Program
    let transform_arena = Bump::with_capacity(1 * 1024 * 1024);
    let transformer = transform_spmt::Transformer::new(&transform_arena);
    let program = transformer.transform(noise_generator);

    // 4. Delegate to the new compiler library
    compile(&program, &config.backend_config)
        .expect("Failed to compile SPMT program into target backends")
}
