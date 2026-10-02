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

    let structure_weight_table = compute_structure_weight_table();
    // paste into a file for inclusion
    let swt_path = out_dir.join("structure_weight_table.dat");
    let out = format!("{:?}", structure_weight_table);
    fs::write(&swt_path, out).expect("Failed to write structure weight table");
}

fn compute_structure_weight_table() -> [f64; 13824] {
    let mut array = [0.0; 13824];
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < 24 {
        while j < 24 {
            while k < 24 {
                array[i * 24 * 24 + j * 24 + k] = calculate_structure_weight(j as i32 - 12, k as i32 - 12, i as i32 - 12);
                k += 1;
            }
            j += 1;
            k = 0;
        }
        i += 1;
        j = 0;
    }
    array
}

fn calculate_structure_weight(x: i32, y: i32, z: i32) -> f64 {
    let y = y as f64 - 0.5;
    let x = x as f64;
    let z = z as f64;
    let d = (x * x + y * y + z * z);
    std::f64::consts::E.powf(-d / 16.0)
}

