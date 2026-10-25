use libloading::{Library, Symbol};

use crate::paths::{HarnessConfig, Target};
use crate::{cuda_build, java_build, rust_build};

pub const DIMS: usize = 16 * 384 * 16;

type RustPipelineFn = unsafe extern "C" fn(f64, f64, f64, i64, *mut f64, usize) -> i32;
type CudaPipelineFn = unsafe extern "C" fn(f64, f64, f64, u64, *mut f64, usize) -> i32;

/// The final-density samples one target produced.
pub struct PipelineOutput {
    pub target: Target,
    pub values: Box<[f64]>,
}

/// Builds every selected target from the sources in `generated_dir`.
pub fn build_targets(config: &HarnessConfig) {
    rust_build::build_rust_cdylib(config);
    if config.has_target(Target::Cuda) {
        cuda_build::build_cuda_shared_lib(config);
    }
    if config.has_target(Target::Java) {
        java_build::build_java(config);
    }
}

/// Runs the Rust reference plus every other selected target with the same
/// seed and origin. The Rust output is always first.
pub fn run_pipelines(config: &HarnessConfig) -> Vec<PipelineOutput> {
    let mut outputs = vec![PipelineOutput {
        target: Target::Rust,
        values: run_rust_pipeline(config),
    }];
    for &target in &config.targets {
        let values = match target {
            Target::Rust => continue,
            Target::Cuda => run_cuda_pipeline(config),
            Target::Java => java_build::run_java_pipeline(config),
        };
        outputs.push(PipelineOutput { target, values });
    }
    outputs
}

fn run_rust_pipeline(config: &HarnessConfig) -> Box<[f64]> {
    // SAFETY: the library was just built by this process from rust_wrapper,
    // which exports the extern "C" symbol named below.
    unsafe {
        let rust_lib = Library::new(config.rust_cdylib_path()).expect("failed to load Rust cdylib");
        let run_rust_pipeline: Symbol<RustPipelineFn> = rust_lib
            .get(b"run_rust_pipeline\0")
            .expect("missing symbol run_rust_pipeline");

        let (ox, oy, oz) = config.origin;
        let mut output = vec![0.0f64; DIMS].into_boxed_slice();
        let rc = run_rust_pipeline(ox, oy, oz, config.seed, output.as_mut_ptr(), DIMS);
        assert_eq!(rc, 0, "run_rust_pipeline returned error code {rc}");
        output
    }
}

fn run_cuda_pipeline(config: &HarnessConfig) -> Box<[f64]> {
    // SAFETY: as above, for cuda_wrapper.
    unsafe {
        let cuda_lib =
            Library::new(config.cuda_shared_lib_path()).expect("failed to load CUDA shared lib");
        let run_cuda_pipeline: Symbol<CudaPipelineFn> = cuda_lib
            .get(b"run_cuda_pipeline\0")
            .expect("missing symbol run_cuda_pipeline");

        let (ox, oy, oz) = config.origin;
        let mut output = vec![0.0f64; DIMS].into_boxed_slice();
        let rc = run_cuda_pipeline(ox, oy, oz, config.seed as u64, output.as_mut_ptr(), DIMS);
        assert_eq!(rc, 0, "run_cuda_pipeline returned error code {rc}");
        output
    }
}
