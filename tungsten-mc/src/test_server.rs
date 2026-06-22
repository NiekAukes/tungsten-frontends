use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

use crate::utils::make_permutation_table;

use crate::mathf64::{Pos3, Vec3, as_index};
use crate::perlin::{create_perlin_noise_sampler, sample_perlin};
use crate::utilsf64::{PerlinNoiseSampler, abs, clamp, fade, hermite, max, min};
use crate::xoroshiro::{
    Xoroshiro128PlusPlusRandom, XoroshiroSeed, create_xoroshiro_seed, create_xoroshiro_seed_str,
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

use crate::orchestration::{self as orch, OrchestrationOutput, PermutationTables};

const NUM_OUTPUTS: usize = 1;
const MIN_Y: i32 = -64;
const HEIGHT: usize = (320 - MIN_Y) as usize;
const DIMS: usize = 16 * HEIGHT * 16;

/// Run a single-density orchestration by name.
/// Returns the pruned output buffer or None if the name is unknown.
fn run_single_density(
    name: &str,
    origin: Vec3,
    perm_tables: &PermutationTables,
) -> Option<Box<[f64; DIMS]>> {
    match name {
        // "barrier" => Some(orch::orchestrate_barrier(origin, perm_tables)),
        // "continents" => Some(orch::orchestrate_continents(origin, perm_tables)),
        // "depth" => Some(orch::orchestrate_depth(origin, perm_tables)),
        // "erosion" => Some(orch::orchestrate_erosion(origin, perm_tables)),
        "final_density" => Some(orch::orchestrate_final_density(origin, perm_tables)),
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

fn get_output(outputs: &OrchestrationOutput, idx: usize) -> Option<&Box<[f64; DIMS]>> {
    match idx {
        // 0 => Some(&outputs.temperature),
        // 1 => Some(&outputs.vegetation),
        // 2 => Some(&outputs.initial_density_without_jaggedness),
        // 3 => Some(&outputs.vein_toggle),
        // 4 => Some(&outputs.continents),
        // 5 => Some(&outputs.vein_gap),
        // 6 => Some(&outputs.vein_ridged),
        // 7 => Some(&outputs.depth),
        // 8 => Some(&outputs.fluid_level_spread),
        9 => Some(&outputs.final_density),
        // 10 => Some(&outputs.ridges),
        // 11 => Some(&outputs.lava),
        // 12 => Some(&outputs.erosion),
        // 13 => Some(&outputs.barrier),
        // 14 => Some(&outputs.fluid_level_floodedness),
        _ => None,
    }
}

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

        // density_point <seed> <ox> <oy> <oz> <px> <py> <pz> <output_idx>
        "density_point" => with_seed_and_f64_args(args, 7, |seed, a| {
            let origin = Vec3::new(a[0], a[1], a[2]);
            let patch_pos = Pos3 {
                x: a[3] as i32,
                y: a[4] as i32,
                z: a[5] as i32,
            };
            let output_idx = a[6] as usize;

            if output_idx >= NUM_OUTPUTS {
                return format!(
                    "ERR output_idx must be 0-{}, got {}",
                    NUM_OUTPUTS - 1,
                    output_idx
                );
            }

            let outputs = super::orchestration_seeded(seed, origin);
            let idx = as_index(patch_pos, 16, 256, 16) as usize;

            if idx >= 65536 {
                return format!("ERR invalid patch position: {}", idx);
            }

            match get_output(&outputs, output_idx) {
                Some(output) => {
                    let value = output[idx];
                    format!("OK {value:.10}")
                }
                None => format!("ERR output_idx out of range"),
            }
        }),

        // density_chunk <seed> <ox> <oy> <oz>
        "density_chunk" => with_seed_and_f64_args(args, 3, |seed, a| {
            let origin = Vec3::new(a[0], a[1], a[2]);

            let outputs = super::orchestration_seeded(seed, origin);

            // Return min/max/mean for each named output
            let all_outputs: [(&str, &Box<[f64; DIMS]>); NUM_OUTPUTS] = [
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
                ("final_density", &outputs.final_density),
                // ("ridges", &outputs.ridges),
                // ("lava", &outputs.lava),
                // ("erosion", &outputs.erosion),
                // ("barrier", &outputs.barrier),
                // ("fluid_level_floodedness", &outputs.fluid_level_floodedness),
            ];
            let mut stats = Vec::new();
            for (name, output) in &all_outputs {
                {
                    let output = *output;
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
            let output_idx = a[3] as usize;

            if output_idx >= NUM_OUTPUTS {
                return format!(
                    "ERR output_idx must be 0-{}, got {}",
                    NUM_OUTPUTS - 1,
                    output_idx
                );
            }

            let outputs = super::orchestration_seeded(seed, origin);

            match get_output(&outputs, output_idx) {
                Some(output) => {
                    // Format all 65536 values from the requested output
                    let formatted: Vec<String> =
                        output.iter().map(|v| format!("{:.6}", v)).collect();

                    format!("OK {}", formatted.join(" "))
                }
                None => format!("ERR output_idx out of range"),
            }
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
            let perm_tables = utils::set_perlin_seed(seed);
            match run_single_density(density_name, origin, perm_tables) {
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
            let perm_tables = utils::set_perlin_seed(seed);
            match run_single_density(density_name, origin, perm_tables) {
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
            let perm_tables = utils::set_perlin_seed(seed);
            match run_single_density(density_name, origin, perm_tables) {
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

// ---------------------------------------------------------------------------
// Unit tests for the dispatcher itself
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping() {
        assert_eq!(dispatch("ping"), "OK pong");
    }

    #[test]
    fn unknown_function() {
        assert!(dispatch("foobar 1 2 3").starts_with("ERR"));
    }

    #[test]
    fn wrong_arg_count() {
        // hermite expects 5 args
        assert!(dispatch("hermite 1 2").starts_with("ERR"));
    }

    #[test]
    fn bad_float() {
        assert!(dispatch("abs notanumber").starts_with("ERR"));
    }

    #[test]
    fn abs_works() {
        let resp = dispatch("abs -3.14");
        assert!(resp.starts_with("OK"));
        let val: f32 = resp.strip_prefix("OK ").unwrap().trim().parse().unwrap();
        assert!((val - 3.14).abs() < 1e-4);
    }

    #[test]
    fn seeded_perlin_is_finite_and_deterministic() {
        let resp1 = dispatch("seeded_perlin 42 0 1.5 2.5");
        let resp2 = dispatch("seeded_perlin 42 0 1.5 2.5");
        assert!(resp1.starts_with("OK"));
        let v1: f32 = resp1.strip_prefix("OK ").unwrap().trim().parse().unwrap();
        let v2: f32 = resp2.strip_prefix("OK ").unwrap().trim().parse().unwrap();
        assert!(v1.is_finite(), "seeded_perlin should return a finite value");
        assert_eq!(v1, v2, "same seed and position should give the same result");
    }

    #[test]
    fn seeded_perlin_different_seeds_differ() {
        let resp1 = dispatch("seeded_perlin 0 0 1.5 2.5");
        let resp2 = dispatch("seeded_perlin 1 0 1.5 2.5");
        assert!(resp1.starts_with("OK") && resp2.starts_with("OK"));
        let v1: f32 = resp1.strip_prefix("OK ").unwrap().trim().parse().unwrap();
        let v2: f32 = resp2.strip_prefix("OK ").unwrap().trim().parse().unwrap();
        assert!(
            (v1 - v2).abs() > 1e-6,
            "different seeds should produce different values"
        );
    }

    #[test]
    fn perlin_lattice_zero() {
        let resp = dispatch("perlin 0 0 0");
        assert!(resp.starts_with("OK"));
        let val: f32 = resp.strip_prefix("OK ").unwrap().trim().parse().unwrap();
        assert!(val.abs() < 1e-6, "perlin at origin should be ~0, got {val}");
    }

    #[test]
    fn clamp_below() {
        let resp = dispatch("clamp -5.0 0.0 1.0");
        let val: f32 = resp.strip_prefix("OK ").unwrap().trim().parse().unwrap();
        assert!((val - 0.0).abs() < 1e-6);
    }

    #[test]
    fn density_point_basic() {
        let resp = dispatch("density_point 0 0 0 0 0 0 0 0");
        assert!(resp.starts_with("OK"));
        let val: f32 = resp.strip_prefix("OK ").unwrap().trim().parse().unwrap();
        assert!(val.is_finite(), "density_point should return finite value");
    }

    #[test]
    fn density_point_invalid_output() {
        let resp = dispatch("density_point 0 0 0 0 0 0 0 20");
        assert!(
            resp.starts_with("ERR"),
            "should reject output_idx >= NUM_OUTPUTS"
        );
    }

    #[test]
    fn density_chunk_returns_stats() {
        let resp = dispatch("density_chunk 0 0 0 0");
        assert!(resp.starts_with("OK"));
        // Response is space-separated tokens of the form "name:min/max/mean"
        let content = resp.strip_prefix("OK ").unwrap().trim();
        let parts: Vec<&str> = content.split_whitespace().collect();
        assert_eq!(
            parts.len(),
            17,
            "density_chunk should return 17 output stats"
        );

        let expected_names = [
            "temperature",
            "vegetation",
            "initial_density_without_jaggedness",
            "vein_toggle",
            "continents",
            "vein_gap",
            "vein_ridged",
            "depth",
            "fluid_level_spread",
            "final_density",
            "ridges",
            "lava",
            "erosion",
            "barrier",
            "minecraft_cave_layer",
            "fluid_level_floodedness",
            "minecraft_temperature_med",
        ];

        // Each token is "name:min/max/mean"
        for (i, part) in parts.iter().enumerate() {
            let (name, stats) = part
                .split_once(':')
                .unwrap_or_else(|| panic!("stat {} '{}' missing ':'", i, part));
            assert_eq!(name, expected_names[i], "stat {} name mismatch", i);
            let nums: Vec<&str> = stats.split('/').collect();
            assert_eq!(
                nums.len(),
                3,
                "stat {} should have 3 values (min/max/mean)",
                i
            );
            for (j, num) in nums.iter().enumerate() {
                let _: f32 = num.parse().unwrap_or_else(|_| {
                    panic!(
                        "stat {} value {} '{}' should be parseable as f32",
                        i, j, num
                    )
                });
            }
        }
    }

    #[test]
    fn density_chunk_all_invalid_output() {
        let resp = dispatch("density_chunk_all 0 0 0 0 17");
        assert!(
            resp.starts_with("ERR"),
            "should reject output_idx >= NUM_OUTPUTS"
        );
    }

    #[test]
    fn make_permutation_table_returns_origin_and_perm() {
        let resp = dispatch("make_permutation_table 0 minecraft:ore_gap 0 minecraft:ore_gap");
        assert!(resp.starts_with("OK"), "should start with OK, got: {resp}");
        let content = resp.strip_prefix("OK ").unwrap().trim();
        let parts: Vec<&str> = content.split_whitespace().collect();
        // 3 xo=/yo=/zo= tokens + 2 p0=/p255= tokens + 256 permutation bytes = 261 tokens
        assert_eq!(parts.len(), 261, "expected 261 tokens, got {}", parts.len());
        // First 3 should be xo=, yo=, zo=
        assert!(
            parts[0].starts_with("xo="),
            "first token should start with xo="
        );
        assert!(
            parts[1].starts_with("yo="),
            "second token should start with yo="
        );
        assert!(
            parts[2].starts_with("zo="),
            "third token should start with zo="
        );
        // p0= and p255=
        assert!(
            parts[3].starts_with("p0="),
            "fourth token should start with p0="
        );
        assert!(
            parts[4].starts_with("p255="),
            "fifth token should start with p255="
        );
        // Remaining 256 should be permutation bytes
        for i in 5..261 {
            let _: u8 = parts[i]
                .parse()
                .unwrap_or_else(|_| panic!("perm {} should be u8", i - 5));
        }
    }

    #[test]
    fn make_permutation_table_deterministic() {
        let cmd = "make_permutation_table 42 minecraft:ore_gap 0 minecraft:ore_gap";
        let resp1 = dispatch(cmd);
        let resp2 = dispatch(cmd);
        assert_eq!(resp1, resp2, "same args should produce identical results");
    }

    #[test]
    fn double_perlin_basic() {
        let resp = dispatch("double_perlin 0 minecraft:ore_gap minecraft:ore_gap 1.5 2.5 3.5");
        assert!(resp.starts_with("OK"), "should start with OK, got: {resp}");
        let val: f32 = resp.strip_prefix("OK ").unwrap().trim().parse().unwrap();
        assert!(
            val.is_finite(),
            "double_perlin should return a finite value"
        );
    }

    #[test]
    fn double_perlin_deterministic() {
        let cmd = "double_perlin 0 minecraft:ore_gap minecraft:ore_gap 1.5 2.5 3.5";
        let resp1 = dispatch(cmd);
        let resp2 = dispatch(cmd);
        assert!(resp1.starts_with("OK") && resp2.starts_with("OK"));
        let v1: f32 = resp1.strip_prefix("OK ").unwrap().trim().parse().unwrap();
        let v2: f32 = resp2.strip_prefix("OK ").unwrap().trim().parse().unwrap();
        assert_eq!(v1, v2, "same args should produce the same result");
    }

    #[test]
    fn double_perlin_wrong_args() {
        let resp = dispatch("double_perlin 0 1 2");
        assert!(resp.starts_with("ERR"), "should reject wrong arg count");
    }

    #[test]
    fn density_point_different_positions() {
        let resp1 = dispatch("density_point 0 0 0 0 0 0 0 0");
        let resp2 = dispatch("density_point 0 0 0 0 1 0 0 0");

        assert!(resp1.starts_with("OK") && resp2.starts_with("OK"));

        let val1: f32 = resp1.strip_prefix("OK ").unwrap().trim().parse().unwrap();
        let val2: f32 = resp2.strip_prefix("OK ").unwrap().trim().parse().unwrap();

        // Different positions in same chunk should generally produce different values
        // (very unlikely they'd be exactly equal)
        // Note: We only assert they're both finite, not different, since it's probabilistic
        assert!(val1.is_finite() && val2.is_finite());
    }

    #[test]
    fn density_single_point_basic() {
        let resp = dispatch("density_single_point 0 0 0 0 0 0 0 barrier");
        assert!(resp.starts_with("OK"), "got: {resp}");
        let val: f32 = resp.strip_prefix("OK ").unwrap().trim().parse().unwrap();
        assert!(
            val.is_finite(),
            "density_single_point should return finite value"
        );
    }

    #[test]
    fn density_single_point_unknown_density() {
        let resp = dispatch("density_single_point 0 0 0 0 0 0 0 nonexistent");
        assert!(
            resp.starts_with("ERR"),
            "should reject unknown density_name"
        );
    }

    #[test]
    fn density_single_chunk_basic() {
        let resp = dispatch("density_single_chunk 0 0 0 0 barrier");
        assert!(resp.starts_with("OK"), "got: {resp}");
        assert!(
            resp.contains("barrier:"),
            "should contain density name in stats"
        );
    }

    #[test]
    fn density_single_chunk_all_basic() {
        let resp = dispatch("density_single_chunk_all 0 0 0 0 barrier");
        assert!(resp.starts_with("OK"), "got: {resp}");
        let content = resp.strip_prefix("OK ").unwrap().trim();
        let count = content.split_whitespace().count();
        assert_eq!(count, 65536, "should return 65536 floats, got {count}");
    }

    #[test]
    fn density_single_matches_full_orchestration() {
        // Verify that single-density output matches the corresponding field
        // from the full orchestration
        let full_resp = dispatch("density_point 0 0 0 0 0 0 0 4"); // 4 = continents
        let single_resp = dispatch("density_single_point 0 0 0 0 0 0 0 continents");
        assert!(full_resp.starts_with("OK") && single_resp.starts_with("OK"));
        let full_val: f32 = full_resp
            .strip_prefix("OK ")
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        let single_val: f32 = single_resp
            .strip_prefix("OK ")
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        assert_eq!(
            full_val, single_val,
            "single-density should match full orchestration: full={full_val}, single={single_val}"
        );
    }
}
