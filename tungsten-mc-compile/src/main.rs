use std::{path::PathBuf, thread::Builder};

use clap::Parser;

use tungsten_wg::spmt::model::SPMT;
use tungsten_wg::spmt::pretty::Printer;

use tungsten_wg::spmt::{dag::DensityDAG, model::Addr, pretty::PrettyPrint};

use tungsten_mc_compile::{MinecraftCompilerConfig, do_compile, parse, run_parse, transform_spmt};

#[derive(Parser, Debug)]
#[command(author, version, about = "Minecraft Worldgen SPMT Transformer & Codegen", long_about = None)]
struct Args {
    #[arg(short, long)]
    mod_folder: Option<String>,

    #[arg(short, long, default_value = ".")]
    output: PathBuf,

    #[arg(long, default_value_t = false)]
    cuda: bool,

    #[arg(long, default_value_t = false)]
    rust: bool,

    #[arg(long, default_value_t = 16)]
    chunk_size: usize,

    /// The noise settings to compile
    #[arg(long, default_value = "minecraft:overworld")]
    root: String,

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
    let mut config = MinecraftCompilerConfig::new()
        .with_chunk_size(args.chunk_size)
        .with_root(args.root.clone());
    if let Some(mod_folder) = &args.mod_folder {
        config = config.with_mod_path(mod_folder.clone());
    }

    // 1. Load and parse the bundled vanilla worldgen plus the mod folder
    let arena = bumpalo::Bump::with_capacity(1 * 1024 * 1024);
    let mcdata = match run_parse(&arena, &config) {
        Ok(mcdata) => mcdata,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if args.verbose {
        println!(
            "Final arena usage after parsing: {:.2} MB",
            arena.allocated_bytes() as f64 / (1024.0 * 1024.0)
        );
    }

    // 2. Core Logic (Density Functions & SPMT Transformations)
    let noise_generator = mcdata.root().expect("run_parse parses the root");

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

    // 4. Run the Compiler
    println!("Compiling SPMT program across target backends...");
    let compiled_output = do_compile(&program);

    // 5. Write Outputs to Disk

    // Fix: Use `with_extension` instead of `join` to properly append `.rs` to the filename
    let real_path = args.output.join("density").with_extension("rs");
    println!(
        "Preparing to write Rust output to '{}'",
        real_path.display()
    );
    if let Some(folder) = real_path.parent() {
        if !folder.as_os_str().is_empty() {
            std::fs::create_dir_all(folder).expect("Unable to create output directory");
        }
    }

    if let Some(wave_graph) = compiled_output.wave_orchestration_graph {
        let wave_graph_path =
            std::path::Path::new("wave_orchestration_graph").with_extension("dot");
        std::fs::write(&wave_graph_path, wave_graph)
            .expect("Unable to write wave orchestration graph");
        println!(
            "Generated wave orchestration graph at '{}'",
            wave_graph_path.display()
        );
    }

    if args.rust || (!args.cuda && !args.rust) {
        let Some(rust_code) = compiled_output.rust_code else {
            eprintln!("No Rust output was generated.");
            return;
        };
        std::fs::write(&real_path, rust_code).unwrap();
        println!("Generated Rust output at '{}'", real_path.display());
    }
    if let Some(java_code) = &compiled_output.java_code {
        let java_path = args.output.join("GeneratedWorldgen.java");
        std::fs::write(&java_path, java_code).unwrap();
        println!("Generated Java output at '{}'", java_path.display());
    }
    if args.cuda {
        // --- Write CUDA ---
        let cuda_base = args.output;
        std::fs::create_dir_all(&cuda_base).expect("Unable to create CUDA output directory");
        println!(
            "CUDA output enabled. Preparing to write CUDA files to '{}'",
            cuda_base.display()
        );
        for (file_name, contents) in &compiled_output.cuda_files {
            let path = cuda_base.join(file_name);
            std::fs::write(&path, contents).unwrap();
            println!("Generated CUDA file '{}'.", path.display());
        }
    }
}

/// Extracted helper function to keep `run_with_args` clean
fn emit_intermediate_files(mcdata: &parse::MinecraftData, program: &SPMT) {
    let density_function = mcdata
        .root()
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
    for (i, mdf) in program.main_density_functions.iter().enumerate() {
        let density_function = mdf.density_function;
        let ddag_root = density_function;
        let ddag = DensityDAG { root: ddag_root };
        let mut printer = Printer::new_with_name_cache(name_cache_bor.take().unwrap());
        ddag.pretty(&mut printer);
        let (dot_output, name_cache) = printer.finish_with_name_cache();

        let fname = density_function.canonical_name.clone().unwrap_or_else(|| {
            name_cache
                .get(&(density_function).addr())
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
