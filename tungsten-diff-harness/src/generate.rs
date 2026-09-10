use std::fs;
use std::path::Path;

use tungsten_mc_compile::{run_generation, CompilerConfig, MinecraftCompilerConfig};

/// Phase 0: compiles a Minecraft datapack source folder into a fresh
/// `density.rs` / `density_function.cu` / `orchestration.cu` triple inside
/// `generated_dir`, using tungsten-mc-compile directly as a library (no
/// subprocess, no CLI file-naming quirks to work around).
///
/// `helpers.cu` is not produced by this step; it must be supplied separately
/// alongside the generated CUDA sources.
pub fn generate_from_source(generated_dir: &Path, source_dir: Option<&Path>, chunk_size: usize) {
    fs::create_dir_all(generated_dir).expect("failed to create generated_dir");

    let backend_config = CompilerConfig::new()
        .with_rcl(true)
        .with_cuda(true);


    let config = MinecraftCompilerConfig::new()
        .with_chunk_size(chunk_size)
        .with_backend_config(backend_config);
    let config = if let Some(source_dir) = source_dir {
        config.with_mod_path(source_dir.to_string_lossy())
    } else {
        config
    };

    println!(
        "Compiling worldgen source into {}...",
        generated_dir.display()
    );
    // let output = run_generation(&config);
    let arena = bumpalo::Bump::new();
    let ast = tungsten_mc_compile::run_parse(&arena, &config);

    // run the shrinker until it cannot shrink anymore
    let initial = ast.noise_settings.get("minecraft:overworld").unwrap()
        .noise_router.final_density;
    let mut shrinker = tungsten_mc_compile::shrink::Shrinker::new(&arena, initial);

    let mut iters = 0;
    while let Some(shrunk) = shrinker.shrink() {
        // take the shrink success randomly
        let random: f32 = rand::random();
        if random < 0.0 {
            // accept this shrunk version
            shrinker.mark_last_shrink_buggy();
        } else {
            shrinker.mark_last_shrink_successful();
        }
        iters += 1;
    }
    println!();
    println!("Total shrink iterations: {}", iters);
    let bytes = arena.allocated_bytes();
    println!("Total size of memory used: {}mb", bytes / 1024 / 1024);

    let final_density = shrinker.finish();
    // println!("Final density after shrinking: {:?}", final_density);
    println!();
    let output = run_generation(&config);

    let rcl = output
        .rcl
        .expect("compiler did not produce RCL (Rust) output");
    fs::write(generated_dir.join("density.rs"), rcl).expect("failed to write density.rs");

    let cuda_density_function = output
        .cuda_density_function
        .expect("compiler did not produce CUDA density_function output");
    fs::write(
        generated_dir.join("density_function.cu"),
        cuda_density_function,
    )
    .expect("failed to write density_function.cu");

    let cuda_orchestration = output
        .cuda_orchestration
        .expect("compiler did not produce CUDA orchestration output");
    fs::write(generated_dir.join("orchestration.cu"), cuda_orchestration)
        .expect("failed to write orchestration.cu");

    println!("Wrote density.rs, density_function.cu, orchestration.cu.");
}
