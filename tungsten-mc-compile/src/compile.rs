// --- Output Structure ---

use tungsten_wg::{RustConfig, compile};

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

    /// The generated wave orchestration graph.
    pub wave_orchestration_graph: Option<String>,
}

pub fn do_compile(program: &tungsten_wg::spmt::SPMT) -> CompiledOutput {
    let optimized_program = tungsten_wg::spmt::conversion::convert(&program).unwrap();
    let mut rust_backend = RustConfig::default().to_backend();
    compile(&optimized_program, &mut [&mut rust_backend])
        .expect("Failed to compile SPMT program into Rust");

    let rust_code = rust_backend.output();
    CompiledOutput {
        rust_code: rust_code.map(|s| s.full_source.clone()),
        cuda_density_function: None,
        cuda_orchestration: None,
        wave_orchestration_graph: None,
    }
}
