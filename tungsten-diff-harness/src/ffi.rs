use libloading::{Library, Symbol};

use crate::paths::HarnessConfig;

pub const DIMS: usize = 16 * 384 * 16;

type RustPipelineFn =
    unsafe extern "C" fn(f64, f64, f64, i64, *mut f64, usize) -> i32;
type CudaPipelineFn =
    unsafe extern "C" fn(f64, f64, f64, u64, *mut f64, usize) -> i32;

/// Result of phase 2 (dynamic load) + phase 3 (execution): both pipelines
/// run with the same origin/seed inputs.
pub struct RunResult {
    pub rust_output: Box<[f64; DIMS]>,
    pub cuda_output: Box<[f64; DIMS]>,
}

pub fn run_rust_pipeline(config: &HarnessConfig) -> Box<[f64; DIMS]> {
    unsafe {
        let rust_lib =
            Library::new(config.rust_cdylib_path()).expect("failed to load Rust cdylib");
        let run_rust_pipeline: Symbol<RustPipelineFn> = rust_lib
            .get(b"run_rust_pipeline\0")
            .expect("missing symbol run_rust_pipeline");

        let (ox, oy, oz) = config.origin;
        let mut rust_output: Box<[f64; DIMS]> = Box::new([0.0; DIMS]);
        let rc = run_rust_pipeline(
            ox,
            oy,
            oz,
            config.seed,
            rust_output.as_mut_ptr(),
            DIMS,
        );
        assert_eq!(rc, 0, "run_rust_pipeline returned error code {rc}");
        rust_output
    }
}

pub fn run_both_pipelines(config: &HarnessConfig) -> RunResult {
    // SAFETY: both libraries were just built by this process from sources
    // we control (rust_wrapper / cuda_wrapper), and export the extern "C"
    // symbols named below.
    unsafe {
        let rust_lib =
            Library::new(config.rust_cdylib_path()).expect("failed to load Rust cdylib");
        let cuda_lib =
            Library::new(config.cuda_shared_lib_path()).expect("failed to load CUDA shared lib");

        let run_rust_pipeline: Symbol<RustPipelineFn> = rust_lib
            .get(b"run_rust_pipeline\0")
            .expect("missing symbol run_rust_pipeline");
        let run_cuda_pipeline: Symbol<CudaPipelineFn> = cuda_lib
            .get(b"run_cuda_pipeline\0")
            .expect("missing symbol run_cuda_pipeline");

        let (ox, oy, oz) = config.origin;

        let mut rust_output: Box<[f64; DIMS]> = Box::new([0.0; DIMS]);
        let rc = run_rust_pipeline(
            ox,
            oy,
            oz,
            config.seed,
            rust_output.as_mut_ptr(),
            DIMS,
        );
        assert_eq!(rc, 0, "run_rust_pipeline returned error code {rc}");

        let mut cuda_output: Box<[f64; DIMS]> = Box::new([0.0; DIMS]);
        let rc = run_cuda_pipeline(
            ox,
            oy,
            oz,
            config.seed as u64,
            cuda_output.as_mut_ptr(),
            DIMS,
        );
        assert_eq!(rc, 0, "run_cuda_pipeline returned error code {rc}");

        RunResult {
            rust_output,
            cuda_output,
        }
    }
}
