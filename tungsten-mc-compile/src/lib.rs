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
//!     .with_root("minecraft:overworld")
//!     .with_chunk_size(16);
//!
//! let output = run_generation(&config);
//! println!("Generated code: {:?}", output.rust_code);
//! ```
//!
//! [`CompilerSession`] is the in-memory alternative: no bundled vanilla, no files, Java only.
//!
//! ## Compilation Pipeline
//!
//! 1. **Loading**: Reads vanilla worldgen configs (bundled) and optional mod/datapack configs
//! 2. **Parsing**: Parses JSON files into internal data structures
//! 3. **Transformation**: Converts density function graph into SPMT representation
//! 4. **Compilation**: Uses tungsten-wg backend to generate optimized code
#![recursion_limit = "512"]

use std::path::PathBuf;

use bumpalo::Bump;

use crate::parse::MinecraftData;

pub use crate::compile::{CompiledOutput, do_compile};
pub use crate::error::{CompileError, ResourceKind};
pub use crate::session::{
    CompilerSession, JavaOutput, JavaUnit, Layout, ReachableId, compiler_version,
};

mod compile;
pub mod config_load;
pub mod contract;
mod error;
pub mod parse;
mod session;
pub mod shrink;
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
    /// The noise settings to compile
    pub root: String,
}

impl Default for MinecraftCompilerConfig {
    fn default() -> Self {
        Self {
            mod_path: None,
            chunk_size: 16,
            root: "minecraft:overworld".to_string(),
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
    /// use tungsten_mc_compile::MinecraftCompilerConfig;
    ///
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

    /// Sets the noise settings to compile (default `minecraft:overworld`).
    pub fn with_root(mut self, root: impl Into<String>) -> Self {
        self.root = root.into();
        self
    }
}

/// Compiles Minecraft worldgen configuration into optimized code.
///
/// This is the main entry point for the compilation pipeline. It:
/// 1. Loads vanilla worldgen configs (bundled with the crate)
/// 2. Optionally loads mod/datapack configs
/// 3. Parses the root noise settings and everything it references
/// 4. Transforms density functions into SPMT representation
/// 5. Compiles to target code using the tungsten-wg backend
///
/// # Panics
///
/// Panics if the worldgen can't be loaded or parsed, or if the SPMT compilation fails.
pub fn run_generation(config: &MinecraftCompilerConfig) -> CompiledOutput {
    let arena = Bump::with_capacity(1 * 1024 * 1024);
    let mcdata = run_parse(&arena, config).unwrap_or_else(|e| panic!("{}", e));
    run_generation_from_ast(&mcdata)
}

/// Loads the bundled vanilla worldgen, then the mod folder on top of it.
pub fn load_folders(config: &MinecraftCompilerConfig) -> std::io::Result<config_load::MinecraftDataRaw> {
    let mut data = config_load::MinecraftDataRaw::new();

    // base version is always at crate_root/vanilla_worldgen
    let mut folder_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    folder_path.push("vanilla_worldgen");
    config_load::load_datapack_folder(&mut data, &folder_path)?;

    if let Some(mod_path) = &config.mod_path {
        config_load::load_datapack_folder(&mut data, mod_path.as_ref())?;
    }
    Ok(data)
}

/// Loads (see [`load_folders`]) and parses `config.root`.
///
/// # Panics
///
/// Panics if a folder can't be read.
pub fn run_parse<'m>(
    arena: &'m Bump,
    config: &MinecraftCompilerConfig,
) -> Result<MinecraftData<'m>, CompileError> {
    let data = load_folders(config).unwrap_or_else(|e| panic!("Could not read worldgen folders: {}", e));
    let mut mcdata = parse::MinecraftData::new(arena, data, config.chunk_size);
    mcdata.parse_root(&config.root)?;
    Ok(mcdata)
}

/// Transforms and compiles the root parsed by [`run_parse`] with every backend.
pub fn run_generation_from_ast(ast: &MinecraftData) -> CompiledOutput {
    let noise_generator = ast.root().expect("run_parse must parse a root first");

    // 3. Transform to SPMT Program
    let transform_arena = Bump::with_capacity(1 * 1024 * 1024);
    let transformer = transform_spmt::Transformer::new(&transform_arena);
    let program = transformer.transform(noise_generator);

    // 4. Delegate to the new compiler library
    do_compile(&program)
}
