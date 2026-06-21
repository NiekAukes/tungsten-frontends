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
    #[arg(short, long)]
    mod_folder: Option<String>,

    #[arg(short, long, default_value = "density")]
    output: PathBuf,

    #[arg(long, default_value_t = false)]
    cuda: bool,

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

    // base version is always at crate_root/vanilla_worldgen
    let macro_root = env!("CARGO_MANIFEST_DIR");
    let mut folder_path = PathBuf::from(macro_root);
    folder_path.push("vanilla_worldgen");

    // 1. Load configs based on CLI args
    config_load::load_all_configs(&mut data, &folder_path.to_string_lossy(), None);
    if let Some(mod_folder) = &args.mod_folder {
        config_load::load_all_configs(&mut data, mod_folder, None);
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
    let config = CompilerConfig::new()
        .with_rcl(true)
        .with_cuda(args.cuda) // Now respects the CLI flag
        .rcl_module_names("density_function", "orchestration");

    // 5. Run the Compiler Library
    println!("Compiling SPMT program across target backends...");
    let compiled_output =
        compile(&program, &config).expect("Fatal error during codegen compilation");

    // 6. Write Outputs to Disk

    // Fix: Use `with_extension` instead of `join` to properly append `.rs` to the filename
    let real_path = args.output.with_extension("rs");
    println!("Preparing to write RCL output to '{}'", real_path.display());
    if let Some(folder) = real_path.parent() {
        if !folder.as_os_str().is_empty() {
            std::fs::create_dir_all(folder).expect("Unable to create output directory");
        }
    }

    // --- Write RCL ---
    // Safely write the newly combined RCL output block
    if let Some(rcl_code) = compiled_output.rcl {
        std::fs::write(&real_path, rcl_code).unwrap();
        println!("Generated inline RCL at '{}'", real_path.display());
    }

    // --- Write CUDA ---
    if args.cuda {
        let cuda_base = args.output;
        std::fs::create_dir_all(&cuda_base).expect("Unable to create CUDA output directory");
        println!(
            "CUDA output enabled. Preparing to write CUDA files to '{}'",
            cuda_base.display()
        );
        if let Some(cuda_density) = compiled_output.cuda_density_function {
            std::fs::write(
                format!("{}_density_function.cu", cuda_base.display()),
                cuda_density,
            )
            .unwrap();
            println!("Generated CUDA density function.");
        }

        if let Some(cuda_orch) = compiled_output.cuda_orchestration {
            std::fs::write(
                format!("{}_orchestration.cu", cuda_base.display()),
                cuda_orch,
            )
            .unwrap();
            println!("Generated CUDA orchestration.");
        }
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
