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
            let host_consts = HostConsts {
                aquifer_max_y: 0,
                splitter: XoroshiroRandomSplitter::from_seed(XoroshiroSeed::new(214140, 12411)),
                estimate_surface_height: EstimatedSurfaceHeight,
            };
            let mut orchestrator = orchestration::Orchestration::new(0);

            orchestrator.orchestrate_aquifer(
                Vec3 {
                    x: 0.0,
                    y: -64.0,
                    z: 0.0,
                },
                &host_consts,
            );

            let t0 = Instant::now();
            let n = 1000;
            for i in 0..n {
                orchestrator.orchestrate_aquifer(
                    Vec3 {
                        x: n as f64 * 16.0,
                        y: -64.0,
                        z: n as f64 * 16.0,
                    },
                    &host_consts,
                );
            }
            let t1 = Instant::now();
            println!("Orchestration took {:?}", (t1 - t0) / n);

            // Block values: low bit is fluid tick, remaining bits the block type.
            // visualize::interactive_viewer(|chunk_x, chunk_z| {
            //     orchestrator
            //         .orchestrate_aquifer(
            //             Vec3 {
            //                 x: (chunk_x * 16) as f64,
            //                 y: -64.0,
            //                 z: (chunk_z * 16) as f64,
            //             },
            //             &host_consts,
            //         )
            //         .to_vec()
            // })
            // .unwrap();
        })
        .unwrap();

    handle.join().unwrap();
}
