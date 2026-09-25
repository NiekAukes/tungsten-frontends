#![allow(warnings)]

include!(concat!(env!("OUT_DIR"), "/generated_worldgen.rs"));

use std::{
    io::Write,
    sync::Arc,
    thread::{self, Builder},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use crate::{mathf64::Vec3, utilsf64::set_perlin_seed};
use rayon::prelude::*;

//mod density_function;
//mod gpu_orchestrator;
pub mod math;
pub mod mathf64;
//mod orchestration;
pub mod perlin;
pub mod random;
mod test_server;
pub mod utils;
pub mod utilsf64;
pub mod xoroshiro;

fn main() {
    let h = Builder::new()
        .name("MainThread".to_string())
        .stack_size(1024 * 1024 * 10)
        .spawn(run)
        .unwrap();
    h.join().unwrap();
}

fn run() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(pos) = args.iter().position(|a| a == "--test-server") {
        let addr = args
            .get(pos + 1)
            .map(|s| s.as_str())
            .unwrap_or("127.0.0.1:9876");
        test_server::run(addr);
        return;
    }

    if args.iter().any(|a| a == "--benchmark") {
        let output = args
            .iter()
            .position(|a| a == "--output")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.clone())
            .unwrap_or_else(|| "chunk_benchmark_rcl.csv".to_string());

        let builder = thread::Builder::new()
            .name("BenchmarkThread".to_string())
            .stack_size(1024 * 1024 * 10);

        let handle = builder
            .spawn(move || {
                //gdt_cpus::pin_thread_to_core(0).unwrap();
                run_benchmark_mp(&output);
            })
            .unwrap();

        handle.join().unwrap();
        return;
    }

    if args.iter().any(|a| a == "--gpu") {
        //run_gpu();
        return;
    }

    if args.iter().any(|a| a == "--gpu-cpu-benchmark") {
        let output = args
            .iter()
            .position(|a| a == "--output")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.clone())
            .unwrap_or_else(|| "gpu_cpu_benchmark.csv".to_string());

        //run_gpu_cpu_benchmark(&output);
        return;
    }

    // Default: compute one chunk
    let builder = thread::Builder::new()
        .name("DensityThread".to_string())
        .stack_size(1024 * 1024 * 10);

    let handle = builder
        .spawn(|| {
            let mut rnd = xoroshiro::Xoroshiro128PlusPlusRandom::new(214140, 12411);
            let x = orchestration_seeded(
                0, //rnd.next_long(),
                Vec3 {
                    x: rnd.next_int_bound(1000) as f64,
                    y: -64.0,
                    z: rnd.next_int_bound(1000) as f64,
                },
            );
            let a = x.final_density;

            // print the mean, min and max of the density values for a sanity check
            let mean = a.iter().sum::<f64>() / a.len() as f64;
            let min = a.iter().cloned().fold(f64::INFINITY, f64::min);
            let max = a.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            println!(
                "Density chunk computed. \nMean: {:.6}\nMin: {:.6}\nMax: {:.6}",
                mean, min, max
            );
        })
        .unwrap();

    handle.join().unwrap();
}

fn run_benchmark(output: &str) {
    // Fixed world seed — same source as the default run
    let mut rnd = xoroshiro::Xoroshiro128PlusPlusRandom::new(214140, 12411);
    let world_seed = rnd.next_long();

    // Initialise permutation tables once for the world seed
    let perm_tables = set_perlin_seed(world_seed);

    let field = (-16, 16);

    println!(
        "Benchmarking {}x{} chunks...",
        field.1 - field.0,
        field.1 - field.0
    );

    struct Record {
        chunk_x: i32,
        chunk_z: i32,
        duration_ms: f64,
        timestamp_ms: u128,
    }

    let mut records: Vec<Record> = Vec::with_capacity(64 * 64);

    for cx in field.0..field.1 {
        for cz in field.0..field.1 {
            let origin = Vec3 {
                x: (cx * 16) as f64,
                y: 0.0,
                z: (cz * 16) as f64,
            };

            let timestamp_ms = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();

            let t0 = Instant::now();
            let _result = orchestration::orchestration(origin, perm_tables);
            let duration_ms = t0.elapsed().as_secs_f64() * 1000.0;

            records.push(Record {
                chunk_x: cx,
                chunk_z: cz,
                duration_ms,
                timestamp_ms,
            });
        }
    }

    // Write CSV
    let file = std::fs::File::create(output).expect("Failed to create output file");
    let mut writer = std::io::BufWriter::new(file);
    writeln!(writer, "chunk_x,chunk_z,duration_ms,timestamp_ms").unwrap();
    for r in &records {
        writeln!(
            writer,
            "{},{},{:.3},{}",
            r.chunk_x, r.chunk_z, r.duration_ms, r.timestamp_ms
        )
        .unwrap();
    }

    let durations: Vec<f64> = records.iter().map(|r| r.duration_ms).collect();
    let n = durations.len() as f64;
    let mean = durations.iter().sum::<f64>() / n;
    let mut sorted = durations.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = sorted[sorted.len() / 2];
    let p95 = sorted[(sorted.len() as f64 * 0.95) as usize];

    println!("Done. {} chunks", records.len());
    println!("  mean   : {:.2} ms", mean);
    println!("  median : {:.2} ms", median);
    println!("  min    : {:.2} ms", sorted[0]);
    println!("  max    : {:.2} ms", sorted[sorted.len() - 1]);
    println!("  p95    : {:.2} ms", p95);
    println!("Saved → {}", output);
}

fn run_benchmark_mp(output: &str) {
    // Fixed world seed — same source as the default run
    let mut rnd = xoroshiro::Xoroshiro128PlusPlusRandom::new(214140, 12411);
    let world_seed = rnd.next_long();

    // Initialise permutation tables once for the world seed
    let perm_tables = set_perlin_seed(world_seed);

    let field = (-16, 16);

    let width = field.1 - field.0;
    let total_chunks = (width * width) as usize;

    println!(
        "Benchmarking {}x{} chunks using {} Rayon threads...",
        width,
        width,
        rayon::current_num_threads()
    );

    struct Record {
        chunk_x: i32,
        chunk_z: i32,
        duration_ms: f64,
        timestamp_ms: u128,
    }

    // Build the list of chunks first.
    let chunks: Vec<(i32, i32)> = (field.0..field.1)
        .flat_map(|cx| (field.0..field.1).map(move |cz| (cx, cz)))
        .collect();

    let start_time = Instant::now();

    // Run each chunk independently in Rayon.
    let records: Vec<Record> = chunks
        .par_iter()
        .map(|&(cx, cz)| {
            let origin = Vec3 {
                x: (cx * 16) as f64,
                y: 0.0,
                z: (cz * 16) as f64,
            };

            let timestamp_ms = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();

            let t0 = Instant::now();

            let _result = orchestration::orchestration(origin, perm_tables);

            let duration_ms = t0.elapsed().as_secs_f64() * 1000.0;

            Record {
                chunk_x: cx,
                chunk_z: cz,
                duration_ms,
                timestamp_ms,
            }
        })
        .collect();

    let elapsed_time = start_time.elapsed().as_secs_f64() * 1000.0;

    // Write CSV
    let file = std::fs::File::create(output)
        .expect("Failed to create output file");

    let mut writer = std::io::BufWriter::new(file);

    writeln!(
        writer,
        "chunk_x,chunk_z,duration_ms,timestamp_ms"
    )
    .unwrap();

    for r in &records {
        writeln!(
            writer,
            "{},{},{:.3},{}",
            r.chunk_x,
            r.chunk_z,
            r.duration_ms,
            r.timestamp_ms
        )
        .unwrap();
    }

    // Statistics
    let durations: Vec<f64> = records
        .iter()
        .map(|r| r.duration_ms)
        .collect();

    let n = durations.len() as f64;

    let mean = durations.iter().sum::<f64>() / n;

    let mut sorted = durations.clone();

    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let median = sorted[sorted.len() / 2];

    let p95 = sorted[(sorted.len() as f64 * 0.95) as usize];

    println!("Done. {} chunks", total_chunks);
    println!("  Rayon threads : {}", rayon::current_num_threads());
    println!("  mean          : {:.2} ms", mean);
    println!("  median        : {:.2} ms", median);
    println!("  min           : {:.2} ms", sorted[0]);
    println!("  max           : {:.2} ms", sorted[sorted.len() - 1]);
    println!("  p95           : {:.2} ms", p95);
    println!("  Total time    : {:.2} ms", elapsed_time);
    println!("Saved → {}", output);
}

// fn run_gpu_cpu_benchmark(output: &str) {
//     println!("=== GPU/CPU Benchmark Tool ===\n");

//     // Fixed world seed
//     let mut rnd = xoroshiro::Xoroshiro128PlusPlusRandom::new(214140, 12411);
//     let world_seed = rnd.next_long();

//     // Prepare permutation tables
//     let perm_tables = set_perlin_seed(world_seed);

//     // Test origin - same for all tests
//     let test_origin = Vec3 {
//         x: (rnd.next_int_bound(100) * 16) as f64,
//         y: 0.0,
//         z: (rnd.next_int_bound(100) * 16) as f64,
//     };

//     struct BenchmarkRecord {
//         test_type: String,
//         processor: String,
//         iteration: Option<u32>,
//         chunk_x: Option<i32>,
//         chunk_z: Option<i32>,
//         duration_ms: f64,
//         timestamp_ms: u128,
//     }

//     let mut records: Vec<BenchmarkRecord> = Vec::new();

//     // === SPEED TEST: Single Chunk (10 iterations each) ===
//     println!("Running speed tests (single chunk, 10 iterations)...");

//     // CPU Speed Test
//     println!("  CPU speed test...");
//     for i in 0..10 {
//         let timestamp_ms = SystemTime::now()
//             .duration_since(UNIX_EPOCH)
//             .unwrap()
//             .as_millis();

//         let t0 = Instant::now();
//         let _result = orchestration::orchestration(test_origin, perm_tables);
//         let duration_ms = t0.elapsed().as_secs_f64() * 1000.0;

//         records.push(BenchmarkRecord {
//             test_type: "speed_single".to_string(),
//             processor: "cpu".to_string(),
//             iteration: Some(i + 1),
//             chunk_x: None,
//             chunk_z: None,
//             duration_ms,
//             timestamp_ms,
//         });
//     }

//     // GPU Speed Test
//     println!("  GPU speed test (initializing GPU)...");
//     let gpu = Arc::new(gpu_orchestrator::GpuOrchestrator_final_density::new());
//     let perm_tables_arc = Arc::new(perm_tables);

//     for i in 0..10 {
//         let timestamp_ms = SystemTime::now()
//             .duration_since(UNIX_EPOCH)
//             .unwrap()
//             .as_millis();

//         let t0 = Instant::now();
//         let handle = gpu.orchestrate(test_origin, &perm_tables_arc);
//         let _result = handle;
//         let duration_ms = t0.elapsed().as_secs_f64() * 1000.0;

//         records.push(BenchmarkRecord {
//             test_type: "speed_single".to_string(),
//             processor: "gpu".to_string(),
//             iteration: Some(i + 1),
//             chunk_x: None,
//             chunk_z: None,
//             duration_ms,
//             timestamp_ms,
//         });
//     }

//     // === THROUGHPUT TEST: Whole Field ===
//     // let field = (-8, 8);
//     // let total_chunks = (field.1 - field.0) * (field.1 - field.0);

//     // println!(
//     //     "\nRunning throughput tests ({}x{} = {} chunks)...",
//     //     field.1 - field.0,
//     //     field.1 - field.0,
//     //     total_chunks
//     // );

//     // // CPU Throughput Test
//     // println!("  CPU throughput test...");
//     // let timestamp_ms = SystemTime::now()
//     //     .duration_since(UNIX_EPOCH)
//     //     .unwrap()
//     //     .as_millis();

//     // let t0 = Instant::now();
//     // for cx in field.0..field.1 {
//     //     for cz in field.0..field.1 {
//     //         let origin = Vec3 {
//     //             x: (cx * 16) as f64,
//     //             y: 0.0,
//     //             z: (cz * 16) as f64,
//     //         };
//     //         let _result = orchestration::orchestration(origin, perm_tables);
//     //     }
//     // }
//     // let cpu_throughput_duration_ms = t0.elapsed().as_secs_f64() * 1000.0;

//     // records.push(BenchmarkRecord {
//     //     test_type: "throughput_field".to_string(),
//     //     processor: "cpu".to_string(),
//     //     iteration: None,
//     //     chunk_x: None,
//     //     chunk_z: None,
//     //     duration_ms: cpu_throughput_duration_ms,
//     //     timestamp_ms,
//     // });

//     // // GPU Throughput Test
//     // println!("  GPU throughput test...");
//     // let timestamp_ms = SystemTime::now()
//     //     .duration_since(UNIX_EPOCH)
//     //     .unwrap()
//     //     .as_millis();

//     // let t0 = Instant::now();
//     // for cx in field.0..field.1 {
//     //     for cz in field.0..field.1 {
//     //         let origin = Vec3 {
//     //             x: (cx * 16) as f64,
//     //             y: 0.0,
//     //             z: (cz * 16) as f64,
//     //         };
//     //         let handle = gpu.orchestrate(origin, &perm_tables_arc);
//     //         let _result = handle.wait();
//     //     }
//     // }
//     // let gpu_throughput_duration_ms = t0.elapsed().as_secs_f64() * 1000.0;

//     // records.push(BenchmarkRecord {
//     //     test_type: "throughput_field".to_string(),
//     //     processor: "gpu".to_string(),
//     //     iteration: None,
//     //     chunk_x: None,
//     //     chunk_z: None,
//     //     duration_ms: gpu_throughput_duration_ms,
//     //     timestamp_ms,
//     // });

//     // === Write CSV ===
//     let file = std::fs::File::create(output).expect("Failed to create output file");
//     let mut writer = std::io::BufWriter::new(file);
//     writeln!(
//         writer,
//         "test_type,processor,iteration,chunk_x,chunk_z,duration_ms,timestamp_ms"
//     )
//     .unwrap();

//     for r in &records {
//         writeln!(
//             writer,
//             "{},{},{},{},{},{:.3},{}",
//             r.test_type,
//             r.processor,
//             r.iteration.map(|i| i.to_string()).unwrap_or_default(),
//             r.chunk_x.map(|i| i.to_string()).unwrap_or_default(),
//             r.chunk_z.map(|i| i.to_string()).unwrap_or_default(),
//             r.duration_ms,
//             r.timestamp_ms
//         )
//         .unwrap();
//     }

//     // === Print Summary ===
//     println!("\n=== Benchmark Results ===");

//     // Speed test summary
//     let cpu_speed: Vec<f64> = records
//         .iter()
//         .filter(|r| r.test_type == "speed_single" && r.processor == "cpu")
//         .map(|r| r.duration_ms)
//         .collect();
//     let gpu_speed: Vec<f64> = records
//         .iter()
//         .filter(|r| r.test_type == "speed_single" && r.processor == "gpu")
//         .map(|r| r.duration_ms)
//         .collect();

//     fn stats(data: &[f64]) -> (f64, f64, f64, f64) {
//         let mean = data.iter().sum::<f64>() / data.len() as f64;
//         let mut sorted = data.to_vec();
//         sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
//         let median = sorted[sorted.len() / 2];
//         let min = sorted[0];
//         let max = sorted[sorted.len() - 1];
//         (mean, median, min, max)
//     }

//     let (cpu_mean, cpu_median, cpu_min, cpu_max) = stats(&cpu_speed);
//     let (gpu_mean, gpu_median, gpu_min, gpu_max) = stats(&gpu_speed);

//     println!("\nSpeed Test (Single Chunk):");
//     println!("  CPU:");
//     println!("    mean   : {:.2} ms", cpu_mean);
//     println!("    median : {:.2} ms", cpu_median);
//     println!("    min    : {:.2} ms", cpu_min);
//     println!("    max    : {:.2} ms", cpu_max);
//     println!("  GPU:");
//     println!("    mean   : {:.2} ms", gpu_mean);
//     println!("    median : {:.2} ms", gpu_median);
//     println!("    min    : {:.2} ms", gpu_min);
//     println!("    max    : {:.2} ms", gpu_max);
//     println!("  Speedup: {:.2}x", cpu_mean / gpu_mean);

//     // println!("\nThroughput Test ({} chunks):", total_chunks);
//     // println!("  CPU:");
//     // println!("    total  : {:.2} ms", cpu_throughput_duration_ms);
//     // println!(
//     //     "    rate   : {:.2} chunks/sec",
//     //     total_chunks as f64 / (cpu_throughput_duration_ms / 1000.0)
//     // );
//     // println!("  GPU:");
//     // println!("    total  : {:.2} ms", gpu_throughput_duration_ms);
//     // println!(
//     //     "    rate   : {:.2} chunks/sec",
//     //     total_chunks as f64 / (gpu_throughput_duration_ms / 1000.0)
//     // );
//     // println!(
//     //     "  Speedup: {:.2}x",
//     //     cpu_throughput_duration_ms / gpu_throughput_duration_ms
//     // );

//     println!("\nSaved → {}", output);
// }

/// Initialize the Perlin sampler with the given seed before computing density
pub fn orchestration_seeded(seed: i64, origin: Vec3) -> orchestration::OrchestrationOutput {
    let perm_tables = set_perlin_seed(seed);
    orchestration::orchestration(origin, perm_tables)
}

// fn run_gpu() {
//     let profile_enabled = std::env::var_os("RCL_PROFILE").is_some();
//     let run_begin = std::time::Instant::now();
//     let mut rnd = xoroshiro::Xoroshiro128PlusPlusRandom::new(214140, 12411);
//     let seed = rnd.next_long();
//     let origin = Vec3 {
//         x: rnd.next_int_bound(1000) as f64,
//         y: rnd.next_int_bound(1000) as f64,
//         z: rnd.next_int_bound(1000) as f64,
//     };
//     let prep_begin = std::time::Instant::now();
//     let perm_tables = orchestration::make_permutation_tables(seed);
//     let prep_duration = prep_begin.elapsed();

//     println!("Compiling GPU shaders...");
//     let gpu_init_begin = std::time::Instant::now();
//     let gpu = gpu_orchestrator::GpuOrchestrator_final_density::new();
//     let gpu_init_duration = gpu_init_begin.elapsed();

//     //let result = gpu.orchestrate(origin, &perm_tables);
//     // run 1000 iterations at once to amortize throughput.
//     let gpu = Arc::new(gpu);
//     let perm_tables = Arc::new(perm_tables);
//     println!("Running GPU orchestration...");
//     gpu.orchestrate(origin, &perm_tables); // warmup
//     let begin = std::time::Instant::now();
//     let result = gpu.orchestrate(origin, &perm_tables);
//     let gpu_duration = begin.elapsed();
//     println!("GPU orchestration completed in {:.2?}", gpu_duration);

//     let mut cpu_direct_duration_ms = None;
//     if profile_enabled {
//         let cpu_direct_begin = std::time::Instant::now();
//         let _cpu_direct_result = orchestration_seeded(seed, origin).final_density;
//         let cpu_direct_duration = cpu_direct_begin.elapsed();
//         cpu_direct_duration_ms = Some(cpu_direct_duration.as_secs_f64() * 1000.0);
//         println!(
//             "[profile][cpu] direct_orchestration={:.3}ms",
//             cpu_direct_duration.as_secs_f64() * 1000.0
//         );
//     }

//     let begin = std::time::Instant::now();
//     //let cpu_result = orchestration_seeded(seed, origin).final_density;
//     let builder = thread::Builder::new()
//         .name("DensityThread".to_string())
//         .stack_size(1024 * 1024 * 100);

//     let handle = builder
//         .spawn(move || orchestration_seeded(seed, origin).final_density)
//         .unwrap();
//     let cpu_result = handle.join().unwrap();

//     let cpu_threaded_duration = begin.elapsed();
//     println!(
//         "CPU orchestration completed in {:.2?}",
//         cpu_threaded_duration
//     );

//     if profile_enabled {
//         println!(
//             "[profile][run] setup_perm_tables={:.3}ms gpu_init={:.3}ms gpu_orchestrate_total={:.3}ms cpu_direct={:.3}ms cpu_threaded_total={:.3}ms run_total={:.3}ms",
//             prep_duration.as_secs_f64() * 1000.0,
//             gpu_init_duration.as_secs_f64() * 1000.0,
//             gpu_duration.as_secs_f64() * 1000.0,
//             cpu_direct_duration_ms.unwrap_or(0.0),
//             cpu_threaded_duration.as_secs_f64() * 1000.0,
//             run_begin.elapsed().as_secs_f64() * 1000.0
//         );
//     }
// }
