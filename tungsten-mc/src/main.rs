#![allow(warnings)]

use crate::mathf64::*;
use crate::utilsf64::*;

// include!(concat!(env!("OUT_DIR"), "/generated_worldgen.rs"));
include!("density.rs");

use std::{
    io::Write,
    sync::Arc,
    thread::{self, Builder},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use crate::mathf64::Vec3;
use crate::orchestration::HostConsts;
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

impl Default for HostConsts {
    fn default() -> Self {
        HostConsts {
            base_box: BlockBox::default(),
            jigsaw_array: [Jigsaw::default(); 500],
            piece_array: [Piece::default(); 500],
        }
    }
}

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
                run_benchmark(&output);
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
            // let x = (
            //     0, //rnd.next_long(),
            //     Vec3 {
            //         x: rnd.next_int_bound(1000) as f64,
            //         y: -64.0,
            //         z: rnd.next_int_bound(1000) as f64,
            //     },
            // );
            let mut orchestrator = orchestration::Orchestration::new(0);
            let a = orchestrator.orchestrate_final_density_with_beardify(
                Vec3 {
                    x: rnd.next_int_bound(1000) as f64,
                    y: -64.0,
                    z: rnd.next_int_bound(1000) as f64,
                },
                &HostConsts::default(),
            );

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
    let mut orchestrator = orchestration::Orchestration::new(world_seed);

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

            let host_consts = HostConsts::default();

            let t0 = Instant::now();
            let _result =
                orchestrator.orchestrate_final_density_with_beardify(origin, &host_consts);
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
    let mut orchestrator = orchestration::Orchestration::new(world_seed);

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
            let mut orchestrator_cloned = orchestrator.clone();
            let origin = Vec3 {
                x: (cx * 16) as f64,
                y: 0.0,
                z: (cz * 16) as f64,
            };

            let timestamp_ms = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();

            let host_consts = HostConsts::default();

            let t0 = Instant::now();

            let _result =
                orchestrator_cloned.orchestrate_final_density_with_beardify(origin, &host_consts);

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

    // Statistics
    let durations: Vec<f64> = records.iter().map(|r| r.duration_ms).collect();

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
