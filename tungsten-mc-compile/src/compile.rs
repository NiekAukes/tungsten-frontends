// --- Output Structure ---

use tungsten_wg::{
    InspectBackend, InspectConfig, JavaConfig, JavaOutput, RustConfig, compile,
    ir::model::OptimizedProgram, optimizer::perform_inline,
};

use crate::contract;
use crate::error::CompileError;

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
    /// Every generated CUDA file as `(file name, contents)`: `orchestration.h`, `interface.cuh`,
    /// `density.cu` and `orchestration.cu`.
    pub cuda_files: Vec<(String, String)>,

    /// The generated Java source (`GeneratedWorldgen.java`).
    pub java_code: Option<String>,

    /// The generated wave orchestration graph.
    pub wave_orchestration_graph: Option<String>,
}

/// Converts the SPMT program and inlines it; the input of every backend.
fn optimize(program: &tungsten_wg::spmt::SPMT) -> Result<OptimizedProgram, CompileError> {
    let mut optimized_program = tungsten_wg::spmt::conversion::convert(program)
        .map_err(|e| CompileError::Internal(format!("SPMT conversion failed: {:?}", e)))?;
    perform_inline(&mut optimized_program);
    Ok(optimized_program)
}

/// Runs only the Java backend, with the class names of [`crate::contract`]. Writes nothing.
pub fn compile_java(program: &tungsten_wg::spmt::SPMT) -> Result<JavaOutput, CompileError> {
    let optimized_program = optimize(program)?;
    let config = JavaConfig {
        package: contract::PACKAGE.to_string(),
        class_name: contract::CLASS_NAME.to_string(),
        ..JavaConfig::default()
    };
    let mut java_backend = config.to_backend();
    compile(&optimized_program, &mut [&mut java_backend])
        .map_err(|e| CompileError::Internal(format!("Java backend failed: {:?}", e)))?;
    java_backend
        .output()
        .cloned()
        .ok_or_else(|| CompileError::Internal("Java backend produced no output".into()))
}

pub fn do_compile(program: &tungsten_wg::spmt::SPMT) -> CompiledOutput {
    let optimized_program = optimize(program).unwrap();

    let mut rust_backend = RustConfig::default().to_backend();
    let mut java_backend = JavaConfig::default().to_backend();
    let mut inspect = InspectBackend::new(InspectConfig::default());
    let mut cuda_config = tungsten_wg::CudaConfig::default();
    cuda_config.profile = tungsten_wg::cuda::Profile::Tolerance;
    let mut cuda_backend = cuda_config.to_backend();
    // let mut cuda_backend = tungsten_wg::CudaConfig::default().to_backend();
    compile(
        &optimized_program,
        &mut [
            &mut rust_backend,
            &mut java_backend,
            &mut inspect,
            &mut cuda_backend,
        ],
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

    let cuda_output = cuda_backend.output().unwrap();
    if cuda_output.warnings.len() > 0 {
        eprintln!("CUDA compilation warnings:");
        for warning in &cuda_output.warnings {
            eprintln!("{}", warning);
        }
    }

    let cuda_density_function = cuda_output.density_source.clone();
    let cuda_orchestration = cuda_output.orchestration_source.clone();

    let rust_code = rust_backend.output();
    CompiledOutput {
        rust_code: rust_code.map(|s| s.full_source.clone()),
        cuda_density_function: Some(cuda_density_function),
        cuda_orchestration: Some(cuda_orchestration),
        cuda_files: cuda_output.files(),
        java_code: Some(java_code.source.clone()),
        wave_orchestration_graph: None,
    }
}
