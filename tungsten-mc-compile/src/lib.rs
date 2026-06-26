//! # tungsten-mc-compile
//!
//! A Minecraft worldgen compiler that parses Minecraft data packs and compiles them
//! to optimized Rust and CUDA code.
//!
//! This crate provides a frontend for the [`tungsten-wg`](https://github.com/NiekAukes/tungsten-wg)
//! compiler that understands Minecraft's worldgen format (density functions, noise settings, etc.).
//!
//! ## Quick Start
//!
//! ```no_run
//! use tungsten_mc_compile::{MinecraftCompilerConfig, run_generation};
//!
//! let config = MinecraftCompilerConfig::new()
//!     .with_target_dimension("minecraft:overworld")
//!     .with_chunk_size(16);
//!
//! let output = run_generation(&config);
//! println!("Generated code: {}", output.rust_code);
//! ```
//!
//! ## Compilation Pipeline
//!
//! 1. **Loading**: Reads vanilla worldgen configs (bundled) and optional mod/datapack configs
//! 2. **Parsing**: Parses JSON files into internal data structures
//! 3. **Transformation**: Converts density function graph into SPMT representation
//! 4. **Compilation**: Uses tungsten-wg backend to generate optimized code

use std::path::PathBuf;

use bumpalo::Bump;
pub use tungsten_wg::CompilerConfig;
use tungsten_wg::{compile, CompiledOutput};

pub mod config_load;
pub mod parse;
pub mod transform_spmt;

/// Configuration for the Minecraft worldgen compilation process.
///
/// This struct provides a builder-style API for configuring how Minecraft
/// worldgen data should be parsed and compiled.
///
/// # Examples
///
/// ```
/// use tungsten_mc_compile::MinecraftCompilerConfig;
///
/// let config = MinecraftCompilerConfig::new()
///     .with_mod_path("path/to/datapack")
///     .with_chunk_size(16);
/// ```
pub struct MinecraftCompilerConfig {
    /// Optional path to a mod or datapack folder to load additional worldgen data
    pub mod_path: Option<String>,
    /// Chunk size for world generation (typically 16 for Minecraft)
    pub chunk_size: usize,
    /// Backend compiler configuration (controls target language, optimizations, etc.)
    pub backend_config: CompilerConfig,
}

impl Default for MinecraftCompilerConfig {
    fn default() -> Self {
        Self {
            mod_path: None,
            chunk_size: 16,
            backend_config: CompilerConfig::default().with_rcl(true),
        }
    }
}

impl MinecraftCompilerConfig {
    /// Creates a new configuration with default values.
    ///
    /// Defaults:
    /// - No mod path (vanilla only)
    /// - Chunk size: 16
    /// - Backend config: Default with RCL enabled
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the path to a mod or datapack folder.
    ///
    /// This allows you to load additional worldgen data on top of vanilla.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = MinecraftCompilerConfig::new()
    ///     .with_mod_path("path/to/my_datapack");
    /// ```
    pub fn with_mod_path(mut self, mod_path: impl Into<String>) -> Self {
        self.mod_path = Some(mod_path.into());
        self
    }

    /// Sets the chunk size for world generation.
    ///
    /// In vanilla Minecraft, chunks are 16x16 blocks.
    ///
    /// # Examples
    ///
    /// ```
    /// use tungsten_mc_compile::MinecraftCompilerConfig;
    ///
    /// let config = MinecraftCompilerConfig::new()
    ///     .with_chunk_size(32); // Use larger chunks
    /// ```
    pub fn with_chunk_size(mut self, size: usize) -> Self {
        self.chunk_size = size;
        self
    }

    /// Sets the backend compiler configuration.
    ///
    /// This controls the target language (Rust/CUDA), optimizations, and other
    /// backend-specific settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use tungsten_mc_compile::MinecraftCompilerConfig;
    /// use tungsten_wg::CompilerConfig;
    ///
    /// let backend = CompilerConfig::default()
    ///     .with_rcl(true)
    ///     .with_cuda(true);
    ///
    /// let config = MinecraftCompilerConfig::new()
    ///     .with_backend_config(backend);
    /// ```
    pub fn with_backend_config(mut self, config: CompilerConfig) -> Self {
        self.backend_config = config;
        self
    }
}

/// Compiles Minecraft worldgen configuration into optimized code.
///
/// This is the main entry point for the compilation pipeline. It:
/// 1. Loads vanilla worldgen configs (bundled with the crate)
/// 2. Optionally loads mod/datapack configs
/// 3. Parses JSON into internal data structures
/// 4. Transforms density functions into SPMT representation
/// 5. Compiles to target code using the tungsten-wg backend
///
/// # Arguments
///
/// * `config` - Configuration specifying which dimension to compile, chunk size, etc.
///
/// # Returns
///
/// A [`CompiledOutput`] containing the generated Rust and/or CUDA code.
///
/// # Panics
///
/// Panics if:
/// - The target dimension is not found in the loaded worldgen data
/// - The SPMT compilation fails
///
/// # Examples
///
/// ```no_run
/// use tungsten_mc_compile::{MinecraftCompilerConfig, run_generation};
///
/// let config = MinecraftCompilerConfig::new()
///     .with_target_dimension("minecraft:overworld");
///
/// let output = run_generation(&config);
///
/// // Save the generated Rust code
/// std::fs::write("worldgen.rs", &output.rust_code).unwrap();
/// ```
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
        .get("minecraft:overworld")
        .unwrap_or_else(|| panic!("Could not find {} settings", "minecraft:overworld"));

    // 3. Transform to SPMT Program
    let transform_arena = Bump::with_capacity(1 * 1024 * 1024);
    let transformer = transform_spmt::Transformer::new(&transform_arena);
    let program = transformer.transform(noise_generator);

    // 4. Delegate to the new compiler library
    compile(&program, &config.backend_config)
        .expect("Failed to compile SPMT program into target backends")
}
