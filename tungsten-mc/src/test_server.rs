use std::io::{BufRead, BufReader, Write};
use std::mem::MaybeUninit;
use std::net::TcpListener;
use std::sync::{Mutex, OnceLock};

use crate::orchestration::HostConsts;

use crate::preliminary_surface_cache::PreliminarySurfaceCache;
use crate::utils::make_permutation_table;

use crate::mathf64::{as_index, Pos3, Vec3};
use crate::perlin::{create_perlin_noise_sampler, sample_perlin};
use crate::utilsf64::{abs, clamp, fade, hermite, max, min, PerlinNoiseSampler};
use crate::xoroshiro::{
    create_xoroshiro_seed, create_xoroshiro_seed_str, Xoroshiro128PlusPlusRandom, XoroshiroSeed,
};
use crate::{density_function, orchestration, utils};

/// Start a TCP test server on the given address (e.g. "127.0.0.1:9876").
///
/// # Protocol
///
/// Each request is a single UTF-8 line terminated by `\n`:
///
/// ```text
/// function_name arg0 arg1 ... argN
/// ```
///
/// The response is a single line:
///
/// ```text
/// OK value0 [value1 value2 ...]
/// ```
///
/// or, on error:
///
/// ```text
/// ERR description
/// ```
///
/// ## Supported functions
///
/// ### Utility functions (single values)
/// | Name                | Args                                                | Returns    |
/// |---------------------|-----------------------------------------------------|------------|
/// | `ping`              | (none)                                              | `pong`     |
/// | `hermite`           | `t p0 p1 m0 m1`                                     | 1 float    |
/// | `fade`              | `x y z`                                             | 3 floats   |
/// | `clamp`             | `x min max`                                         | 1 float    |
/// | `abs`               | `x`                                                 | 1 float    |
/// | `min`               | `a b`                                               | 1 float    |
/// | `max`               | `a b`                                               | 1 float    |
/// | `seeded_perlin`     | `seed x y z`  (`seed` is i64)                       | 1 float    |
/// | `old_blended_noise` | `x y z xz_scale y_scale xz_factor y_factor smear`   | 1 float    |
///
/// ### Density computation functions
/// All `seed` arguments are signed 64-bit (`i64`). Origin/position arguments are f32.
///
/// | Name                | Args                                                               | Returns                             |
/// |---------------------|--------------------------------------------------------------------|-------------------------------------|
/// | `density_chunk`     | `seed origin_x origin_y origin_z`                                  | 17 `name:min/max/mean` tokens       |
/// | `density_point`     | `seed origin_x origin_y origin_z patch_x patch_y patch_z output_idx` | 1 float                          |
/// | `density_chunk_all` | `seed origin_x origin_y origin_z output_idx`                       | 65536 floats (full array)           |
///
/// `output_idx` values for density functions:
/// `0`=temperature, `1`=vegetation, `2`=initial_density_without_jaggedness,
/// `3`=vein_toggle, `4`=continents, `5`=vein_gap, `6`=vein_ridged, `7`=depth,
/// `8`=fluid_level_spread, `9`=final_density, `10`=ridges, `11`=lava, `12`=erosion,
/// `13`=barrier, `14`=minecraft_cave_layer, `15`=fluid_level_floodedness,
/// `16`=minecraft_temperature_med
///
/// ### Single-density orchestration (pruned DAG)
/// These commands compute only the named density and its transitive dependencies,
/// rather than the full orchestration. Much faster when you only need one output.
///
/// | Name                      | Args                                                    | Returns                 |
/// |---------------------------|---------------------------------------------------------|-------------------------|
/// | `density_single_chunk`    | `seed origin_x origin_y origin_z density_name`          | `name:min/max/mean`     |
/// | `density_single_point`    | `seed origin_x origin_y origin_z px py pz density_name` | 1 float                |
/// | `density_single_chunk_all`| `seed origin_x origin_y origin_z density_name`          | 65536 floats            |
///
/// Valid `density_name` values: `barrier`, `continents`, `depth`, `erosion`,
/// `final_density`, `fluid_level_floodedness`, `fluid_level_spread`,
/// `initial_density_without_jaggedness`, `lava`, `ridges`, `temperature`,
/// `vegetation`, `vein_gap`, `vein_ridged`, `vein_toggle`
///
/// ### Permutation / Double Perlin functions
/// `seed` is a u64 (world seed). String keys are hashed via MD5 (`create_xoroshiro_seed_str`).
///
/// | Name                     | Args                                              | Returns                                                       |
/// |--------------------------|---------------------------------------------------|---------------------------------------------------------------|
/// | `make_permutation_table` | `seed seed1_name iter_count seed2_name`            | `xo=<f64> yo=<f64> zo=<f64> p0=<i8> p255=<i8> perm[0..255]`  |
/// | `double_perlin`          | `seed seed1_name seed2_name x y z`                 | 1 float                                                       |
///
/// ### Xoroshiro128++ functions
/// All integer arguments and return values are signed 64-bit (`i64`).
///
/// | Name                   | Args                    | Returns                            |
/// |------------------------|-------------------------|------------------------------------|
/// | `xoroshiro_next`       | `seed_lo seed_hi`       | 1 i64 (first `next_raw()` output)  |
/// | `xoroshiro_next_n`     | `seed_lo seed_hi n`     | n space-separated i64 values       |
/// | `xoroshiro_seed_long`  | `seed`                  | `seed_lo seed_hi` after mixing     |
/// | `xoroshiro_seed_str`   | `name`                  | `seed_lo seed_hi` from MD5 of name |
///
/// Send `quit` to gracefully shut down the connection.
pub fn run(addr: &str) {
    let listener = TcpListener::bind(addr).expect("failed to bind test server");
    println!("[test-server] listening on {addr}");

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[test-server] accept error: {e}");
                continue;
            }
        };

        let peer = stream
            .peer_addr()
            .map(|a| a.to_string())
            .unwrap_or_else(|_| "unknown".into());
        println!("[test-server] connection from {peer}");

        let reader = BufReader::new(match stream.try_clone() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[test-server] clone error: {e}");
                continue;
            }
        });
        let mut writer = stream;

        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("[test-server] read error: {e}");
                    break;
                }
            };

            let line = line.trim().to_string();
            if line.is_empty() {
                continue;
            }
            if line == "quit" {
                println!("[test-server] {peer} disconnected (quit)");
                break;
            }

            let response = dispatch(&line);
            if writeln!(writer, "{response}").is_err() {
                break;
            }
        }

        println!("[test-server] {peer} disconnected");
    }
}

use crate::orchestration::{self as orch};

const MIN_Y: i32 = -64;
const HEIGHT: usize = (320 - MIN_Y) as usize;
const DIMS: usize = 16 * HEIGHT * 16;

/// Run a single-density orchestration by name.
/// Returns the pruned output buffer or None if the name is unknown.
fn run_single_density(
    name: &str,
    origin: Vec3,
    orchestrator: &mut orchestration::Orchestration,
) -> Option<Vec<f64>> {
    let host_consts = HostConsts::default();
    match name {
        // "barrier" => Some(orch::orchestrate_barrier(origin, perm_tables)),
        // "continents" => Some(orch::orchestrate_continents(origin, perm_tables)),
        // "depth" => Some(orch::orchestrate_depth(origin, perm_tables)),
        // "erosion" => Some(orch::orchestrate_erosion(origin, perm_tables)),
        "final_density" => Some(
            orchestrator
                .orchestrate_final_density(origin, &host_consts)
                .into(),
        ),
        // "fluid_level_floodedness" => Some(orch::orchestrate_fluid_level_floodedness(
        //     origin,
        //     perm_tables,
        // )),
        // "fluid_level_spread" => Some(orch::orchestrate_fluid_level_spread(origin, perm_tables)),
        // "initial_density_without_jaggedness" => Some(
        //     orch::orchestrate_initial_density_without_jaggedness(origin, perm_tables),
        // ),
        // "lava" => Some(orch::orchestrate_lava(origin, perm_tables)),
        // "ridges" => Some(orch::orchestrate_ridges(origin, perm_tables)),
        // "temperature" => Some(orch::orchestrate_temperature(origin, perm_tables)),
        // "vegetation" => Some(orch::orchestrate_vegetation(origin, perm_tables)),
        // "vein_gap" => Some(orch::orchestrate_vein_gap(origin, perm_tables)),
        // "vein_ridged" => Some(orch::orchestrate_vein_ridged(origin, perm_tables)),
        // "vein_toggle" => Some(orch::orchestrate_vein_toggle(origin, perm_tables)),
        _ => None,
    }
}

static PRELIMINARY_SURFACE: OnceLock<PreliminarySurfaceCache> = OnceLock::new();

fn get_estimated_surface_height(seed: i64, biome_x: i32, biome_z: i32) -> i32 {
    // get the preliminary surface height from the cache
    // if let Some(cache) = &PRELIMINARY_SURFACE {
    //     return cache.get(biome_x, biome_z);
    // } else {
    //     // If the cache is not initialized, initialize it
    //     unsafe {
    //         let orchestration = orchestration::Orchestration::new(seed);
    //         PRELIMINARY_SURFACE = Some(PreliminarySurfaceCache::new(orchestration));
    //     }
    //     if let Some(cache) = &PRELIMINARY_SURFACE {
    //         return cache.get(biome_x, biome_z);
    //     } else {
    //         panic!("Failed to initialize preliminary surface cache");
    //     }
    // }

    // let mut cache = PRELIMINARY_SURFACE.lock().unwrap();
    // if cache.is_none() {
    //     let orchestration = orchestration::Orchestration::new(seed);
    //     *cache = Some(PreliminarySurfaceCache::new(orchestration));
    // }

    let cache = PRELIMINARY_SURFACE
        .get_or_init(|| PreliminarySurfaceCache::new(orchestration::Orchestration::new(seed)));
    cache.get(biome_x, biome_z)
}

const VALID_DENSITY_NAMES: &[&str] = &[
    "barrier",
    "continents",
    "depth",
    "erosion",
    "final_density",
    "fluid_level_floodedness",
    "fluid_level_spread",
    "initial_density_without_jaggedness",
    "lava",
    "ridges",
    "temperature",
    "vegetation",
    "vein_gap",
    "vein_ridged",
    "vein_toggle",
];
// ---------------------------------------------------------------------------
// Dispatcher
// ---------------------------------------------------------------------------

fn dispatch(line: &str) -> String {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return "ERR empty command".into();
    }

    let name = parts[0];
    let args = &parts[1..];

    match name {
        "ping" => "OK pong".into(),

        // "hermite" => with_args(args, 5, |a| {
        //     let v = hermite(a[0], a[1], a[2], a[3], a[4]);
        //     format!("OK {v:.10}")
        // }),
        "fade" => with_args(args, 3, |a| {
            let r = fade(Vec3::new(a[0], a[1], a[2]));
            format!("OK {:.10} {:.10} {:.10}", r.x, r.y, r.z)
        }),

        "clamp" => with_args(args, 3, |a| {
            let v = clamp(a[0], a[1], a[2]);
            format!("OK {v:.10}")
        }),

        "abs" => with_args(args, 1, |a| {
            let v = abs(a[0]);
            format!("OK {v:.10}")
        }),

        "min" => with_args(args, 2, |a| {
            let v = min(a[0], a[1]);
            format!("OK {v:.10}")
        }),

        "max" => with_args(args, 2, |a| {
            let v = max(a[0], a[1]);
            format!("OK {v:.10}")
        }),

        "seeded_perlin" => {
            if args.len() != 4 {
                format!("ERR expected 4 args (seed x y z), got {}", args.len())
            } else {
                let seed = match args[0].parse::<u64>() {
                    Ok(v) => v,
                    Err(e) => return format!("ERR arg 0 parse error: {e}"),
                };
                let x = match args[1].parse::<f64>() {
                    Ok(v) => v,
                    Err(e) => return format!("ERR arg 1 parse error: {e}"),
                };
                let y = match args[2].parse::<f64>() {
                    Ok(v) => v,
                    Err(e) => return format!("ERR arg 2 parse error: {e}"),
                };
                let z = match args[3].parse::<f64>() {
                    Ok(v) => v,
                    Err(e) => return format!("ERR arg 3 parse error: {e}"),
                };
                let xoro_seed = create_xoroshiro_seed(seed as i64);
                let mut rng = Xoroshiro128PlusPlusRandom::from_seed(&xoro_seed);
                let pns = create_perlin_noise_sampler(&mut rng);
                //xo=136.599, yo=154.968, zo=1.418, p0=-61, p255=-39
                println!(
                    "xo={}, yo={}, zo={}, p0={}, p255={}",
                    pns.origin_x,
                    pns.origin_y,
                    pns.origin_z,
                    pns.permutation[0] as i8,
                    pns.permutation[255] as i8
                );
                let v = sample_perlin(&pns, x, y, z) as f32;
                format!("OK {v:.10}")
            }
        }

        // density_chunk <seed> <ox> <oy> <oz>
        "density_chunk" => with_seed_and_f64_args(args, 3, |seed, a| {
            let origin = Vec3::new(a[0], a[1], a[2]);

            let mut orchestrator = orchestration::Orchestration::new(seed);
            let output = orchestrator.orchestrate_final_density(origin, &HostConsts::default());

            // Return min/max/mean for each named output
            let all_outputs: [(&str, Vec<f64>); 1] = [
                // ("temperature", &outputs.temperature),
                // ("vegetation", &outputs.vegetation),
                // (
                //     "initial_density_without_jaggedness",
                //     &outputs.initial_density_without_jaggedness,
                // ),
                // ("vein_toggle", &outputs.vein_toggle),
                // ("continents", &outputs.continents),
                // ("vein_gap", &outputs.vein_gap),
                // ("vein_ridged", &outputs.vein_ridged),
                // ("depth", &outputs.depth),
                // ("fluid_level_spread", &outputs.fluid_level_spread),
                ("final_density", output.into()),
                // ("ridges", &outputs.ridges),
                // ("lava", &outputs.lava),
                // ("erosion", &outputs.erosion),
                // ("barrier", &outputs.barrier),
                // ("fluid_level_floodedness", &outputs.fluid_level_floodedness),
            ];
            let mut stats = Vec::new();
            for (name, output) in &all_outputs {
                {
                    let mut min = f64::INFINITY;
                    let mut max = f64::NEG_INFINITY;
                    let mut sum = 0.0;

                    for &val in output.iter() {
                        if val.is_finite() {
                            min = min.min(val);
                            max = max.max(val);
                            sum += val;
                        }
                    }

                    let mean = sum / 65536.0;
                    stats.push(format!("{name}:{min:.6}/{max:.6}/{mean:.6}"));
                }
            }

            format!("OK {}", stats.join(" "))
        }),

        // density_chunk_all <seed> <ox> <oy> <oz> <output_idx>
        "density_chunk_all" => with_seed_and_f64_args(args, 4, |seed, a| {
            let origin = Vec3::new(a[0], a[1], a[2]);

            let mut orchestrator = orchestration::Orchestration::new(seed);
            let output = orchestrator.orchestrate_final_density(origin, &HostConsts::default());

            // Format all values from the requested output
            let formatted: Vec<String> = output.iter().map(|v| format!("{:.6}", v)).collect();

            format!("OK {}", formatted.join(" "))
        }),

        // ── Permutation / Double Perlin commands ────────────────────────────

        // make_permutation_table <seed> <seed1_name> <iter_count> <seed2_name>
        // seed is a u64 (world seed), seed1_name and seed2_name are string keys
        // (hashed via MD5), iter_count is 0 or 1.
        // Returns: xo=<f64> yo=<f64> zo=<f64> p0=<i8> p255=<i8>
        "make_permutation_table" => {
            if args.len() != 4 {
                format!(
                    "ERR expected 4 args (seed seed1_name iter_count seed2_name), got {}",
                    args.len()
                )
            } else {
                let seed = match args[0].parse::<i64>() {
                    Ok(v) => v,
                    Err(e) => return format!("ERR arg 0 (seed) parse error: {e}"),
                };
                let seed1_name = args[1];
                let iter_count = match args[2].parse::<i64>() {
                    Ok(v) => v,
                    Err(e) => return format!("ERR arg 2 (iter_count) parse error: {e}"),
                };
                let seed2_name = args[3];

                let s1 = create_xoroshiro_seed_str(seed1_name);
                let s2 = create_xoroshiro_seed_str(seed2_name);
                let pns = utils::make_permutation_table(
                    seed, s1.seed_lo, s1.seed_hi, iter_count, s2.seed_lo, s2.seed_hi,
                );

                let perm_str: Vec<String> = pns.permutation.iter().map(|b| b.to_string()).collect();
                format!(
                    "OK xo={:.10} yo={:.10} zo={:.10} p0={} p255={}",
                    pns.origin_x,
                    pns.origin_y,
                    pns.origin_z,
                    pns.permutation[0] as i8,
                    pns.permutation[255] as i8,
                )
            }
        }

        // double_perlin <seed> <seed1_name> x y z
        // seed is a u64 (world seed), seed1_name is string keys.
        // Builds two permutation tables (iteration_count 0 and 1) and samples
        // "double_perlin" => {
        //     if args.len() != 5 {
        //         format!("ERR expected 5 arg (seed), got {}", args.len())
        //     } else {
        //         let seed = match args[0].parse::<i64>() {
        //             Ok(v) => v,
        //             Err(e) => return format!("ERR arg 0 (seed) parse error: {e}"),
        //         };
        //         let seed1_name = match args.get(1) {
        //             Some(s) => *s,
        //             None => return format!("ERR missing arg 1 (seed1_name)"),
        //         };
        //         let x = match args.get(2).and_then(|s| s.parse::<f64>().ok()) {
        //             Some(v) => v,
        //             None => return format!("ERR arg 2 (x) parse error"),
        //         };
        //         let y = match args.get(3).and_then(|s| s.parse::<f64>().ok()) {
        //             Some(v) => v,
        //             None => return format!("ERR arg 3 (y) parse error"),
        //         };
        //         let z = match args.get(4).and_then(|s| s.parse::<f64>().ok()) {
        //             Some(v) => v,
        //             None => return format!("ERR arg 4 (z) parse error"),
        //         };

        //         if seed1_name == "minecraft:ore_gap" {
        //             let seed2_name = "octave_-5";
        //             let seed1 = create_xoroshiro_seed_str(seed1_name);
        //             let seed2 = create_xoroshiro_seed_str(seed2_name);

        //             let mut rng =
        //                 Xoroshiro128PlusPlusRandom::from_seed(&create_xoroshiro_seed(seed));

        //             let mut splitter = rng.next_splitter();
        //             let rng1 = splitter.split(seed1.seed_lo, seed1.seed_hi);

        //             println!(
        //                 "Base seed: seed={}, seed_lo={}, seed_hi={}",
        //                 seed, rng.seed_lo, rng.seed_hi
        //             );
        //             println!(
        //                 "splitter after split: seed_lo={}, seed_hi={}\n",
        //                 rng1.seed_lo, rng1.seed_hi
        //             );

        //             let pt0 = make_permutation_table(
        //                 seed,
        //                 seed1.seed_lo,
        //                 seed1.seed_hi,
        //                 0,
        //                 seed2.seed_lo,
        //                 seed2.seed_hi,
        //             );
        //             let pt1 = make_permutation_table(
        //                 seed,
        //                 seed1.seed_lo,
        //                 seed1.seed_hi,
        //                 1,
        //                 seed2.seed_lo,
        //                 seed2.seed_hi,
        //             );
        //             let pns0 = PerlinNoiseSampler {
        //                 origin_x: pt0.origin_x,
        //                 origin_y: pt0.origin_y,
        //                 origin_z: pt0.origin_z,
        //                 permutation: pt0.permutation,
        //             };
        //             let pns1 = PerlinNoiseSampler {
        //                 origin_x: pt1.origin_x,
        //                 origin_y: pt1.origin_y,
        //                 origin_z: pt1.origin_z,
        //                 permutation: pt1.permutation,
        //             };
        //             // first: [0: xo=5.874, yo=14.358, zo=124.358, p0=-46, p255=42, ]},
        //             // second: [0: xo=181.525, yo=40.360, zo=182.905, p0=-100, p255=15, ]}}
        //             println!(
        //                 "first: [0: xo={:.10}, yo={:.10}, zo={:.10}, p0={}, p255={}]",
        //                 pt0.origin_x,
        //                 pt0.origin_y,
        //                 pt0.origin_z,
        //                 pt0.permutation[0] as i8,
        //                 pt0.permutation[255] as i8
        //             );
        //             println!(
        //                 "second: [0: xo={:.10}, yo={:.10}, zo={:.10}, p0={}, p255={}]",
        //                 pt1.origin_x,
        //                 pt1.origin_y,
        //                 pt1.origin_z,
        //                 pt1.permutation[0] as i8,
        //                 pt1.permutation[255] as i8
        //             );
        //             println!("");

        //             // sample ore_gap noise at 0,0,0 with both tables
        //             let p = Pos3 { x: 0, y: 0, z: 0 };
        //             let origin = Vec3::new(x as f32, y as f32, z as f32);
        //             let v0 = density_function::minecraft_ore_gap_5(p, origin, &*pt0, &*pt1);

        //             return format!("OK {v0:.10}");
        //         } else if seed1_name == "minecraft:cave_layer" {
        //             let perm_tables = orchestration::make_permutation_tables(seed);
        //             let p = Pos3 { x: 0, y: 0, z: 0 };
        //             let origin = Vec3::new(x as f32, y as f32, z as f32);
        //             let v = density_function::minecraft_cave_layer_0(
        //                 p,
        //                 origin,
        //                 &perm_tables.minecraft_cave_layer_0_octave__8,
        //                 &perm_tables.minecraft_cave_layer_1_octave__8,
        //             );
        //             return format!("OK {v:.10}");
        //         } else {
        //             return format!("ERR unknown seed1_name: {seed1_name}");
        //         }
        //     }
        // }

        // "temperature" => with_seed_and_f32_args(args, 3, |seed, a| {
        //     let origin = Vec3::new(a[0], a[1], a[2]);
        //     let outputs = super::orchestration_seeded(seed, origin);
        //     let value = outputs.temperature[0];

        //     format!("OK {value:.10}")
        // }),

        // ── Single-density orchestration commands ─────────────────────────────

        // density_single_chunk <seed> <ox> <oy> <oz> <density_name>
        "density_single_chunk" => {
            if args.len() != 5 {
                return format!(
                    "ERR expected 5 args (seed ox oy oz density_name), got {}",
                    args.len()
                );
            }
            let seed = match args[0].parse::<i64>() {
                Ok(v) => v,
                Err(e) => return format!("ERR arg 0 (seed) parse error: {e}"),
            };
            let ox = match args[1].parse::<f64>() {
                Ok(v) => v,
                Err(e) => return format!("ERR arg 1 (ox) parse error: {e}"),
            };
            let oy = match args[2].parse::<f64>() {
                Ok(v) => v,
                Err(e) => return format!("ERR arg 2 (oy) parse error: {e}"),
            };
            let oz = match args[3].parse::<f64>() {
                Ok(v) => v,
                Err(e) => return format!("ERR arg 3 (oz) parse error: {e}"),
            };
            let density_name = args[4];
            let origin = Vec3::new(ox, oy, oz);
            let mut orchestrator = orchestration::Orchestration::new(seed);
            match run_single_density(density_name, origin, &mut orchestrator) {
                Some(output) => {
                    let mut min = f64::INFINITY;
                    let mut max = f64::NEG_INFINITY;
                    let mut sum = 0.0;
                    for &val in output.iter() {
                        if val.is_finite() {
                            min = min.min(val);
                            max = max.max(val);
                            sum += val;
                        }
                    }
                    let mean = sum / 65536.0;
                    format!("OK {density_name}:{min:.6}/{max:.6}/{mean:.6}")
                }
                None => format!(
                    "ERR unknown density_name: {density_name}. Valid: {}",
                    VALID_DENSITY_NAMES.join(", ")
                ),
            }
        }

        // density_single_point <seed> <ox> <oy> <oz> <px> <py> <pz> <density_name>
        "density_single_point" => {
            if args.len() != 8 {
                return format!(
                    "ERR expected 8 args (seed ox oy oz px py pz density_name), got {}",
                    args.len()
                );
            }
            let seed = match args[0].parse::<i64>() {
                Ok(v) => v,
                Err(e) => return format!("ERR arg 0 (seed) parse error: {e}"),
            };
            let ox = match args[1].parse::<f64>() {
                Ok(v) => v,
                Err(e) => return format!("ERR arg 1 (ox) parse error: {e}"),
            };
            let oy = match args[2].parse::<f64>() {
                Ok(v) => v,
                Err(e) => return format!("ERR arg 2 (oy) parse error: {e}"),
            };
            let oz = match args[3].parse::<f64>() {
                Ok(v) => v,
                Err(e) => return format!("ERR arg 3 (oz) parse error: {e}"),
            };
            let px = match args[4].parse::<i32>() {
                Ok(v) => v,
                Err(e) => return format!("ERR arg 4 (px) parse error: {e}"),
            };
            let py = match args[5].parse::<i32>() {
                Ok(v) => v,
                Err(e) => return format!("ERR arg 5 (py) parse error: {e}"),
            };
            let pz = match args[6].parse::<i32>() {
                Ok(v) => v,
                Err(e) => return format!("ERR arg 6 (pz) parse error: {e}"),
            };
            let density_name = args[7];
            let origin = Vec3::new(ox, oy, oz);
            let patch_pos = Pos3 {
                x: px,
                y: py,
                z: pz,
            };
            let idx = as_index(patch_pos, 16, HEIGHT as i32, 16) as usize;
            if idx >= DIMS {
                return format!("ERR invalid patch position: {idx}");
            }
            let mut orchestrator = orchestration::Orchestration::new(seed);
            match run_single_density(density_name, origin, &mut orchestrator) {
                Some(output) => {
                    let value = output[idx];
                    format!("OK {value}")
                }
                None => format!(
                    "ERR unknown density_name: {density_name}. Valid: {}",
                    VALID_DENSITY_NAMES.join(", ")
                ),
            }
        }

        // density_single_chunk_all <seed> <ox> <oy> <oz> <density_name>
        "density_single_chunk_all" => {
            if args.len() != 5 {
                return format!(
                    "ERR expected 5 args (seed ox oy oz density_name), got {}",
                    args.len()
                );
            }
            let seed = match args[0].parse::<i64>() {
                Ok(v) => v,
                Err(e) => return format!("ERR arg 0 (seed) parse error: {e}"),
            };
            let ox = match args[1].parse::<f64>() {
                Ok(v) => v,
                Err(e) => return format!("ERR arg 1 (ox) parse error: {e}"),
            };
            let oy = match args[2].parse::<f64>() {
                Ok(v) => v,
                Err(e) => return format!("ERR arg 2 (oy) parse error: {e}"),
            };
            let oz = match args[3].parse::<f64>() {
                Ok(v) => v,
                Err(e) => return format!("ERR arg 3 (oz) parse error: {e}"),
            };
            let density_name = args[4];
            let origin = Vec3::new(ox, oy, oz);
            let mut orchestration = orchestration::Orchestration::new(seed);
            match run_single_density(density_name, origin, &mut orchestration) {
                Some(output) => {
                    let formatted: Vec<String> = output.iter().map(|v| format!("{}", v)).collect();
                    format!("OK {}", formatted.join(" "))
                }
                None => format!(
                    "ERR unknown density_name: {density_name}. Valid: {}",
                    VALID_DENSITY_NAMES.join(", ")
                ),
            }
        }

        "estimated_surface_height" => {
            if args.len() != 3 {
                return format!("ERR expected 4 args (seed ox oy oz), got {}", args.len());
            }
            let seed = match args[0].parse::<i64>() {
                Ok(v) => v,
                Err(e) => return format!("ERR arg 0 (seed) parse error: {e}"),
            };
            let ox = match args[1].parse::<f64>() {
                Ok(v) => v,
                Err(e) => return format!("ERR arg 1 (ox) parse error: {e}"),
            };
            let oz = match args[2].parse::<f64>() {
                Ok(v) => v,
                Err(e) => return format!("ERR arg 3 (oz) parse error: {e}"),
            };
            let result = get_estimated_surface_height(seed, (ox as i32), (oz as i32));
            let e = format!("OK {}", result);
            println!("{}", e);
            e
        }

        // ── Xoroshiro128++ commands ───────────────────────────────────────────

        // xoroshiro_next <seed_lo> <seed_hi>
        // Returns the first next_raw() value for the given seed pair.
        "xoroshiro_next" => with_i64_args(args, 2, |a| {
            let mut rng = Xoroshiro128PlusPlusRandom::new(a[0], a[1]);
            let v = rng.next_raw();
            format!("OK {v}")
        }),

        // xoroshiro_next_n <seed_lo> <seed_hi> <n>
        // Returns n space-separated next_raw() values.
        "xoroshiro_next_n" => with_i64_args(args, 3, |a| {
            let mut rng = Xoroshiro128PlusPlusRandom::new(a[0], a[1]);
            let n = a[2].max(0) as usize;
            let vals: Vec<String> = (0..n).map(|_| rng.next_raw().to_string()).collect();
            format!("OK {}", vals.join(" "))
        }),

        // xoroshiro_seed_long <seed>
        // Returns seed_lo seed_hi produced by create_xoroshiro_seed(seed).
        "xoroshiro_seed_long" => with_i64_args(args, 1, |a| {
            let s = create_xoroshiro_seed(a[0]);
            format!("OK {} {}", s.seed_lo, s.seed_hi)
        }),

        // xoroshiro_seed_str <name>
        // Returns seed_lo seed_hi produced by create_xoroshiro_seed_str(name).
        "xoroshiro_seed_str" => {
            if args.len() != 1 {
                format!("ERR expected 1 arg (name), got {}", args.len())
            } else {
                let s = create_xoroshiro_seed_str(args[0]);
                format!("OK {} {}", s.seed_lo, s.seed_hi)
            }
        }

        other => format!("ERR unknown function: {other}"),
    }
}

/// Parse `args` as `expected` number of i64s, then call `f`.
fn with_i64_args(args: &[&str], expected: usize, f: impl FnOnce(&[i64]) -> String) -> String {
    if args.len() != expected {
        return format!("ERR expected {expected} args, got {}", args.len());
    }
    let mut parsed = Vec::with_capacity(expected);
    for (i, s) in args.iter().enumerate() {
        match s.parse::<i64>() {
            Ok(v) => parsed.push(v),
            Err(e) => return format!("ERR arg {i} parse error: {e}"),
        }
    }
    f(&parsed)
}

/// Parse `args` as one i64 seed followed by `n_f32` f32s, then call `f(seed, floats)`.
fn with_seed_and_f64_args(
    args: &[&str],
    n_f32: usize,
    f: impl FnOnce(i64, &[f64]) -> String,
) -> String {
    let expected = 1 + n_f32;
    if args.len() != expected {
        return format!("ERR expected {expected} args, got {}", args.len());
    }
    let seed = match args[0].parse::<i64>() {
        Ok(v) => v,
        Err(e) => return format!("ERR arg 0 (seed) parse error: {e}"),
    };
    let mut floats = Vec::with_capacity(n_f32);
    for (i, s) in args[1..].iter().enumerate() {
        match s.parse::<f64>() {
            Ok(v) => floats.push(v),
            Err(e) => return format!("ERR arg {} parse error: {e}", i + 1),
        }
    }
    f(seed, &floats)
}

/// Parse `args` as `expected` number of f32s, then call `f`.
fn with_args(args: &[&str], expected: usize, f: impl FnOnce(&[f64]) -> String) -> String {
    if args.len() != expected {
        return format!("ERR expected {expected} args, got {}", args.len());
    }

    let mut parsed = Vec::with_capacity(expected);
    for (i, s) in args.iter().enumerate() {
        match s.parse::<f64>() {
            Ok(v) => parsed.push(v),
            Err(e) => return format!("ERR arg {i} parse error: {e}"),
        }
    }

    f(&parsed)
}
