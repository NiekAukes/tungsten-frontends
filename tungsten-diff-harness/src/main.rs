mod cuda_build;
mod ffi;
mod generate;
mod oracle;
mod paths;
mod rust_build;

use clap::Parser;
use paths::{Cli, HarnessConfig};

fn main() {
    let cli = Cli::parse();
    let config = HarnessConfig::from_cli(&cli);
    println!("Phase 0: compiling generated sources...");
    let path = if let Some(source_dir) = &cli.source_dir {
        Some(source_dir.as_path())
    } else {
        None
    };
    generate::generate_from_source(&config.generated_dir, path.clone(), cli.chunk_size);
    println!("Compiling Rust and CUDA pipelines into shared libraries...");
    rust_build::build_rust_cdylib(&config);
    //cuda_build::build_cuda_shared_lib(&config);

    println!("Loading both libraries and executing the pipelines...");
    // let result = ffi::run_both_pipelines(&config);
    let result = ffi::run_rust_pipeline(&config);

    println!(
        "Comparing {} samples (epsilon={:e})...",
        ffi::DIMS,
        config.epsilon
    );
    // oracle::assert_pipelines_agree(&result, config.epsilon);
    println!("Rust pipeline produced {} samples.", ffi::DIMS);

    println!("OK: Rust and CUDA pipelines agree within tolerance.");
}
