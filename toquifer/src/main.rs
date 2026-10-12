#![allow(warnings)]

use crate::mathf64::*;
use crate::utilsf64::*;
use crate::xoroshiro::XoroshiroSeed;

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

//mod density_function;
//mod gpu_orchestrator;
pub mod math;
pub mod mathf64;
//mod orchestration;
pub mod perlin;
pub mod random;
pub mod utils;
pub mod utilsf64;
pub mod visualize;
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

            let host_consts = HostConsts {
                aquifer_max_y: 0,
                splitter: XoroshiroRandomSplitter::from_seed(XoroshiroSeed::new(214140, 12411)),
                estimate_surface_height: EstimatedSurfaceHeight,
                erosion_cache: Erosion,
                depth_cache: Depth,
            };
            let mut orchestrator = orchestration::Orchestration::new(0);
            let a = orchestrator.orchestrate_aquifer(
                Vec3 {
                    x: 0.0,
                    y: -64.0,
                    z: 0.0,
                },
                &host_consts,
            );

            // print the distribution of the blocks. The last bit indicates fluid_ticks, the remaining bits the block type
            let mut block_distribution = std::collections::HashMap::new();
            let mut fluid_ticks = 0;
            for block in a {
                let block_type = block >> 1;
                let fluid_tick = block & 1;
                let block_name = match block_type {
                    0 => "solid",
                    1 => "special",
                    2 => "air",
                    3 => "water",
                    4 => "lava",
                    _ => "unknown",
                };
                *block_distribution.entry(block_name).or_insert(0) += 1;
                fluid_ticks += fluid_tick;
            }
            println!("Block distribution: {:?}", block_distribution);
            println!("Fluid ticks: {}", fluid_ticks);
            a.to_vec()
        })
        .unwrap();

    let result = handle.join().unwrap();
    visualize::interactive_z_viewer(&result, 16, 384, 16).unwrap();
}
