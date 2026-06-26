// build.rs
use std::env;
use std::fs;
use std::path::PathBuf;

// Adjust import paths to match your actual crate names
use tungsten_mc_compile::{CompilerConfig, MinecraftCompilerConfig, run_generation};

fn debug_print(_msg: &str) {
    //println!("cargo:warning={}", msg);
}

fn main() {
    // 3. Configure the underlying backend (RCL + CUDA)
    let backend_config = CompilerConfig::new()
        .with_rcl(true)
        .with_cuda(false)
        .rcl_module_names("density_function", "orchestration");

    // 4. Configure the Minecraft pipeline
    let config = MinecraftCompilerConfig::new()
        .with_chunk_size(16)
        .with_backend_config(backend_config);

    // 5. Generate the code
    let output = run_generation(&config);

    debug_print("Code generation completed. Preparing to write outputs...");

    // 6. Write outputs to source files
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    if let Some(mut rcl_code) = output.rcl {
        debug_print("Writing RCL output to OUT_DIR...");
        let rs_path = out_dir.join("generated_worldgen.rs");
        debug_print(&format!("RCL output path: {}", rs_path.display()));
        rcl_code = "use crate::mathf64::*;\nuse crate::utilsf64::*;\n\n".to_string() + &rcl_code;
        fs::write(&rs_path, rcl_code).expect("Failed to write generated Rust code");
    }

    if let Some(cuda_code) = output.cuda_density_function {
        let cu_path = out_dir.join("generated_worldgen.cu");
        fs::write(&cu_path, cuda_code).expect("Failed to write generated CUDA code");
    }
}
