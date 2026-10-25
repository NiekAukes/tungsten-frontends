mod cuda_build;
mod ffi;
mod generate;
mod java_build;
mod oracle;
mod paths;
mod reexport;
mod rust_build;

use clap::Parser;
use paths::{Cli, HarnessConfig};

fn main() {
    let cli = Cli::parse();
    let config = HarnessConfig::from_cli(&cli);
    let target_names: Vec<_> = config.targets.iter().map(|t| t.name()).collect();
    println!("Targets: {}", target_names.join(", "));
    println!("Phase 0: compiling generated sources...");
    let path = if let Some(source_dir) = &cli.source_dir {
        Some(source_dir.as_path())
    } else {
        None
    };
    generate::generate_from_source(&config, path, cli.chunk_size, cli.single_run);
    println!("Building the selected pipelines...");
    ffi::build_targets(&config);

    println!("Executing the pipelines...");
    let outputs = ffi::run_pipelines(&config);
    println!("Rust pipeline produced {} samples.", ffi::DIMS);

    println!("Comparing {} samples against the Rust reference...", ffi::DIMS);
    let compared = oracle::assert_pipelines_agree(&outputs, &config);
    if compared.is_empty() {
        println!("No target to compare: only the Rust reference was selected.");
        return;
    }
    let names: Vec<_> = compared.iter().map(|t| t.name()).collect();
    println!("OK: Rust agrees with {}.", names.join(", "));
}
