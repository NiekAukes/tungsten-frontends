use std::{path::PathBuf, thread::Builder};

use clap::Parser;

use tungsten_wg::spmt::model::SPMT;
use tungsten_wg::spmt::pretty::Printer;
// Import the new library API
use tungsten_wg::{CompilerConfig, compile};

use tungsten_wg::spmt::{dag::DensityDAG, model::Addr, pretty::PrettyPrint};

pub mod config_load;
pub mod parse;
pub mod transform_spmt;

#[derive(Parser, Debug)]
#[command(author, version, about = "Minecraft Worldgen SPMT Transformer & Codegen", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "vanilla_worldgen_1.21.1")]
    mod_folder: String,

    #[arg(short, long, default_value = "vanilla_worldgen_1.21.1")]
    base_version: String,

    #[arg(short, long, default_value = "../rcl_density")]
    output_dir: PathBuf,

    #[arg(long, default_value_t = false)]
    skip_shaders: bool,

    #[arg(long, default_value_t = 16)]
    chunk_size: usize,

    #[arg(long, default_value_t = false)]
    emit_intermediates: bool,

    #[arg(short, long)]
    verbose: bool,
}

pub fn main() {
    let args = Args::parse();

    let h = Builder::new()
        .name("Main Thread".into())
        .stack_size(16 * 1024 * 1024) // 16 MB stack size
        .spawn(move || {
            run_with_args(args);
        })
        .expect("Failed to spawn main thread");
    h.join().expect("Main thread panicked");
}

fn run_with_args(args: Args) {
    let mut data = config_load::MinecraftDataRaw::new();

    // 1. Load configs based on CLI args
    config_load::load_all_configs(&mut data, &args.base_version, None);
    if args.mod_folder != args.base_version {
        config_load::load_all_configs(&mut data, &args.mod_folder, None);
    }

    let arena = bumpalo::Bump::with_capacity(1 * 1024 * 1024);
    let mut mcdata = parse::MinecraftData::new(&arena, &data, args.chunk_size);
    mcdata.parse_from_raw();

    if args.verbose {
        println!(
            "Final arena usage after parsing: {:.2} MB",
            arena.allocated_bytes() as f64 / (1024.0 * 1024.0)
        );
    }

    // 2. Core Logic (Density Functions & SPMT Transformations)
    let noise_generator = mcdata
        .noise_settings
        .get("minecraft:overworld")
        .expect("Could not find minecraft:overworld settings");

    let transform_arena = bumpalo::Bump::with_capacity(1 * 1024 * 1024);
    let transformer = transform_spmt::Transformer::new(&transform_arena);
    let program = transformer.transform(noise_generator);

    if args.verbose {
        let bytes = transform_arena.allocated_bytes();
        println!(
            "Final arena usage after SPMT transformation: {:.2} MB",
            bytes as f64 / (1024.0 * 1024.0)
        );
    }

    // 3. Emit Intermediates (SPMT DAGs, DOT files)
    if args.emit_intermediates {
        emit_intermediate_files(&mcdata, &program);
    }

    // 4. Configure the Compiler
    // Assuming `CompilerConfig` was expanded to include Naga/CUDA toggles
    let config = CompilerConfig::new()
        .with_rcl(true)
        .with_cuda(true)
        .rcl_module_names("density_function", "orchestration");

    // 5. Run the Compiler Library
    println!("Compiling SPMT program across target backends...");
    let compiled_output =
        compile(&program, &config).expect("Fatal error during codegen compilation");

    // 6. Write Outputs to Disk
    let folder = args.output_dir.as_path().to_str().unwrap();
    std::fs::create_dir_all(folder).expect("Unable to create output directory");

    // --- Write RCL ---
    if let Some(rcl_code) = compiled_output.rcl_density_function {
        let path = format!("{}/src/density_function.rs", folder);
        std::fs::write(&path, &rcl_code).unwrap();
        println!("Generated '{}' ({} bytes)", path, rcl_code.len());
    }

    if let Some(orch_code) = compiled_output.rcl_orchestration {
        let path = format!("{}/src/orchestration.rs", folder);
        std::fs::write(&path, &orch_code).unwrap();
        println!("Generated '{}' ({} bytes)", path, orch_code.len());
    }

    // // --- Write Naga / WGSL Shaders ---
    // currently not supported anymore (unsuccesful attempt)
    // if !args.skip_shaders {
    //     if let Some(gpu_orch) = compiled_output.gpu_orchestrator {
    //         let path = format!("{}/src/gpu_orchestrator.rs", folder);
    //         std::fs::write(&path, &gpu_orch).unwrap();
    //         println!("Generated '{}' ({} bytes)", path, gpu_orch.len());
    //     }

    //     if let Some(naga_shaders) = compiled_output.naga_shaders {
    //         let shaders_dir = format!("{}/shaders", folder);
    //         if std::path::Path::new(&shaders_dir).exists() {
    //             std::fs::remove_dir_all(&shaders_dir).unwrap();
    //         }
    //         std::fs::create_dir_all(&shaders_dir).unwrap();

    //         for (name, wgsl_code) in naga_shaders {
    //             let file_path = format!("{}/{}.wgsl", shaders_dir, name);
    //             std::fs::write(&file_path, &wgsl_code).unwrap();
    //             println!(
    //                 "Generated WGSL shader '{}' ({} bytes)",
    //                 file_path,
    //                 wgsl_code.len()
    //             );
    //         }
    //     }
    // }

    // --- Write CUDA ---
    // (Assuming CUDA still writes to the hardcoded ../cuda_density path)
    let cuda_folder = "../cuda_density";
    if let Some(cuda_density) = compiled_output.cuda_density_function {
        std::fs::create_dir_all(format!("{}/src", cuda_folder)).unwrap_or_default();
        std::fs::write(
            format!("{}/src/density_function.cu", cuda_folder),
            cuda_density,
        )
        .unwrap();
        println!("Generated CUDA density function.");
    }
    if let Some(cuda_orch) = compiled_output.cuda_orchestration {
        std::fs::create_dir_all(format!("{}/src", cuda_folder)).unwrap_or_default();
        std::fs::write(format!("{}/src/orchestration.cu", cuda_folder), cuda_orch).unwrap();
        println!("Generated CUDA orchestration.");
    }
}

/// Extracted helper function to keep `run_with_args` clean
fn emit_intermediate_files(mcdata: &parse::MinecraftData, program: &SPMT) {
    let density_function = mcdata
        .noise_settings
        .get("minecraft:overworld")
        .unwrap()
        .noise_router
        .final_density;

    let pretty = format!("{}", density_function.get_density());
    std::fs::write("pretty_density_function.txt", pretty).expect("Unable to write file");

    let dot = parse::dot::print_density_dot(density_function.get_density());
    std::fs::write("density_function.dot", dot).expect("Unable to write file");

    let mut printer = Printer::new();
    program.pretty(&mut printer);
    let (_, name_cache) = printer.finish_with_name_cache();

    std::fs::create_dir_all("density_dags").expect("Unable to create directory");

    let mut name_cache_bor = Some(name_cache);
    for (i, (density_function, _)) in program.main_density_functions.iter().enumerate() {
        let ddag_root = *density_function;
        let ddag = DensityDAG { root: ddag_root };
        let mut printer = Printer::new_with_name_cache(name_cache_bor.take().unwrap());
        ddag.pretty(&mut printer);
        let (dot_output, name_cache) = printer.finish_with_name_cache();

        let fname = density_function.canonical_name.clone().unwrap_or_else(|| {
            name_cache
                .get(&(*density_function).addr())
                .cloned()
                .unwrap_or_else(|| "unknown".into())
        });

        name_cache_bor = Some(name_cache);
        let file_name = format!(
            "density_dags/{}.dot",
            fname.replace(|c: char| !c.is_alphanumeric() && c != '_', "_")
        );

        if let Err(e) = std::fs::write(&file_name, dot_output) {
            eprintln!("Failed to write DOT file for density function {}: {}", i, e);
        }
    }

    std::fs::create_dir_all("density_functions").expect("Unable to create directory");
    for (i, density_function) in program.density_functions.iter().enumerate() {
        let mut printer = Printer::new_with_name_cache(name_cache_bor.take().unwrap());
        density_function.pretty_with_deps(&mut printer);
        let (dot_output, name_cache) = printer.finish_with_name_cache();

        let fname = density_function.canonical_name.clone().unwrap_or_else(|| {
            name_cache
                .get(&(*density_function).addr())
                .cloned()
                .unwrap_or_else(|| "unknown".into())
        });
        name_cache_bor = Some(name_cache);
        let file_name = format!(
            "density_functions/{}.spmt",
            fname.replace(|c: char| !c.is_alphanumeric() && c != '_', "_")
        );

        if let Err(e) = std::fs::write(&file_name, dot_output) {
            eprintln!(
                "Failed to write SPMT file for density function {}: {}",
                i, e
            );
        }
    }
}
