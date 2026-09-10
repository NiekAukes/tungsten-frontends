use std::path::{Path, PathBuf};

use clap::Parser;

/// Differential harness for the generated Rust and CUDA worldgen pipelines.
///
/// Every option can also be set via the matching environment variable; CLI
/// flags take precedence.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Datapack source folder to compile into `generated-dir` before
    /// running the diff (phase 0). Skipped if not set, in which case
    /// `generated-dir` must already contain density.rs / density_function.cu
    /// / orchestration.cu / helpers.cu.
    #[arg(long, env = "TUNGSTEN_SOURCE_DIR")]
    pub source_dir: Option<PathBuf>,

    /// Directory containing (or receiving, if `--source-dir` is used)
    /// `density.rs`, `density_function.cu`, `orchestration.cu`, `helpers.cu`.
    #[arg(long, env = "TUNGSTEN_GENERATED_DIR")]
    pub generated_dir: Option<PathBuf>,

    /// Directory the harness writes build artifacts (the two .so files) to.
    #[arg(long, env = "TUNGSTEN_DIFF_BUILD_DIR")]
    pub build_dir: Option<PathBuf>,

    /// Chunk size used when compiling from `--source-dir`.
    #[arg(long, env = "TUNGSTEN_DIFF_CHUNK_SIZE", default_value_t = 16)]
    pub chunk_size: usize,

    /// World seed fed to both pipelines.
    #[arg(long, env = "TUNGSTEN_DIFF_SEED", default_value_t = 0)]
    pub seed: i64,

    /// Chunk-local origin X fed to both pipelines.
    #[arg(long, env = "TUNGSTEN_DIFF_ORIGIN_X", default_value_t = 0.0)]
    pub origin_x: f64,

    /// Chunk-local origin Y fed to both pipelines.
    #[arg(long, env = "TUNGSTEN_DIFF_ORIGIN_Y", default_value_t = -64.0)]
    pub origin_y: f64,

    /// Chunk-local origin Z fed to both pipelines.
    #[arg(long, env = "TUNGSTEN_DIFF_ORIGIN_Z", default_value_t = 0.0)]
    pub origin_z: f64,

    /// Absolute/relative tolerance used by the differential oracle.
    #[arg(long, env = "TUNGSTEN_DIFF_EPSILON", default_value_t = 1e-4)]
    pub epsilon: f64,
}

/// All paths and knobs the harness needs, resolved from the CLI (which in
/// turn falls back to environment variables and defaults).
pub struct HarnessConfig {
    /// Directory containing `density.rs`, `density_function.cu`,
    /// `orchestration.cu`, and `helpers.cu` for a single compiled pipeline.
    pub generated_dir: PathBuf,
    /// Directory the harness writes build artifacts (the two .so files) to.
    pub build_dir: PathBuf,
    /// Path to the tungsten-diff-rust-wrapper crate.
    pub rust_wrapper_manifest: PathBuf,
    /// Path to the CUDA wrapper source.
    pub cuda_wrapper_src: PathBuf,
    /// World seed fed to both pipelines.
    pub seed: i64,
    /// Chunk-local origin fed to both pipelines.
    pub origin: (f64, f64, f64),
    /// Absolute tolerance used by the differential oracle.
    pub epsilon: f64,
}

impl HarnessConfig {
    pub fn from_cli(cli: &Cli) -> Self {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let generated_dir = cli
            .generated_dir
            .clone()
            .unwrap_or_else(|| manifest_dir.join("generated"));

        let build_dir = cli
            .build_dir
            .clone()
            .unwrap_or_else(|| manifest_dir.join("target").join("diff-harness"));

        Self {
            generated_dir,
            build_dir,
            rust_wrapper_manifest: manifest_dir.join("rust_wrapper").join("Cargo.toml"),
            cuda_wrapper_src: manifest_dir.join("cuda_wrapper").join("cuda_wrapper.cu"),
            seed: cli.seed,
            origin: (cli.origin_x, cli.origin_y, cli.origin_z),
            epsilon: cli.epsilon,
        }
    }

    pub fn density_rs(&self) -> PathBuf {
        self.generated_dir.join("density.rs")
    }

    pub fn rust_cdylib_path(&self) -> PathBuf {
        // cdylib naming matches the Cargo package name declared in
        // rust_wrapper/Cargo.toml.
        cdylib_path(&self.build_dir.join("rust"), "tungsten_diff_rust_wrapper")
    }

    pub fn cuda_shared_lib_path(&self) -> PathBuf {
        self.build_dir.join("cuda").join("libcuda_pipeline.so")
    }
}

/// Resolves the platform-specific cdylib filename cargo would produce for
/// `crate_name` inside `target_dir` (e.g. `target_dir/release/libfoo.so`).
fn cdylib_path(target_dir: &Path, crate_name: &str) -> PathBuf {
    let file_name = if cfg!(target_os = "windows") {
        format!("{crate_name}.dll")
    } else if cfg!(target_os = "macos") {
        format!("lib{crate_name}.dylib")
    } else {
        format!("lib{crate_name}.so")
    };
    target_dir.join("release").join(file_name)
}
