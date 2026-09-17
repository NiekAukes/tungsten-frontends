use std::fs;
use std::process::Command;

use crate::paths::HarnessConfig;

/// Phase 1 (CUDA side): compiles `cuda_wrapper.cu` (which `#include`s the
/// generated `density_function.cu` / `orchestration.cu`, which in turn are
/// expected to `#include "helpers.cu"`) into a shared library via `nvcc`.
pub fn build_cuda_shared_lib(config: &HarnessConfig) {
    for required in ["density_function.cu", "orchestration.cu"] {
        let path = config.generated_dir.join(required);
        assert!(
            path.exists(),
            "expected generated CUDA source at {}",
            path.display()
        );
    }

    // let helpers_path = config.generated_dir.join("helpers.cu");
    let helpers_path = config.cuda_wrapper_src.join("helpers.cu");
    assert!(
        helpers_path.exists(),
        "expected CUDA helpers at {} (not yet generated for this pipeline)",
        helpers_path.display()
    );

    let out_dir = config.build_dir.join("cuda");
    fs::create_dir_all(&out_dir).expect("failed to create CUDA build directory");

    let out_path = config.cuda_shared_lib_path();

    let status = Command::new("nvcc")
        .arg("-arch=compute_75")
        .arg("--shared")
        .arg("-Xcompiler")
        .arg("-fPIC")
        .arg("-O0")
        .arg("-t 0")
        .arg("-o")
        .arg(&out_path)
        .arg(&config.cuda_wrapper_src.join("main.cu"))
        .arg("-I")
        .arg(&config.generated_dir)
        .arg("-I")
        .arg(&config.cuda_wrapper_src)
        .env("CMAKE_CUDA_COMPILER_LAUNCHER", "ccache")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .status()
        .expect("failed to spawn `nvcc` (is the CUDA toolkit installed?)");

    assert!(status.success(), "nvcc build of cuda_wrapper.cu failed");
    assert!(
        out_path.exists(),
        "expected CUDA shared library at {}, but it was not produced",
        out_path.display()
    );
}
