// --- Output Structure ---

use tungsten_wg::{
    InspectBackend, InspectConfig, JavaConfig, RustConfig, compile, optimizer::perform_inline,
};

/// Holds the generated raw source code strings for whichever backends were enabled
/// in the [`CompilerConfig`].
#[derive(Debug, Default)]
pub struct CompiledOutput {
    /// The concatenated Rust source code.
    /// Contains both the density function mathematical logic and the wave orchestration logic.
    pub rust_code: Option<String>,
    //pub rcl_orchestration: Option<String>,
    //pub gpu_orchestrator: Option<String>,
    /// The generated CUDA device C++ code containing the mathematical density kernels.
    pub cuda_density_function: Option<String>,
    /// The generated CUDA host C++ code handling memory and kernel launch orchestration.
    pub cuda_orchestration: Option<String>,

    /// The generated Java source (`GeneratedWorldgen.java`).
    pub java_code: Option<String>,

    /// The generated wave orchestration graph.
    pub wave_orchestration_graph: Option<String>,
}

pub fn do_compile(program: &tungsten_wg::spmt::SPMT) -> CompiledOutput {
    let mut optimized_program = tungsten_wg::spmt::conversion::convert(&program).unwrap();
    perform_inline(&mut optimized_program);

    let mut rust_backend = RustConfig::default().to_backend();
    let mut java_backend = JavaConfig::default().to_backend();
    let mut inspect = InspectBackend::new(InspectConfig::default());
    compile(
        &optimized_program,
        &mut [&mut rust_backend, &mut java_backend, &mut inspect],
    )
    .expect("Failed to compile SPMT program into Rust");

    let java_code = java_backend.output().unwrap();
    if java_code.warnings.len() > 0 {
        eprintln!("Java compilation warnings:");
        for warning in &java_code.warnings {
            eprintln!("{}", warning);
        }
    }

    let inspect_output = inspect.output().unwrap();
    // write all files to /target/inspect
    for (filename, content) in &inspect_output.files {
        let path = std::path::Path::new("target/inspect").join(filename);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(path, content).unwrap();
    }

    let rust_code = rust_backend.output();
    CompiledOutput {
        rust_code: rust_code.map(|s| s.full_source.clone()),
        cuda_density_function: None,
        cuda_orchestration: None,
        java_code: Some(java_code.source.clone()),
        wave_orchestration_graph: None,
    }
}
