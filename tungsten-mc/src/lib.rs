#![allow(warnings)]

include!(concat!(env!("OUT_DIR"), "/generated_worldgen.rs"));

//pub mod density_function;
pub mod math;
pub mod mathf64;
//mod orchestration;
pub mod perlin;
pub mod random;
pub mod utils;
pub mod utilsf64;
pub mod xoroshiro;

pub use mathf64::{Pos3, Vec3};
pub use utils::set_perlin_seed;

/// Compute a full 16x256x16 density chunk with the given seed and origin.
/// Returns 13 output arrays (one for each density function output).
// pub fn compute_density_chunk(
//     seed: u32,
//     origin: Vec3,
// ) -> [Box<[f32; 65536]>; 13] {
//     let (a, b, c, d, e, f, g, h, i, j, k, l, m) = orchestration_seeded(seed, origin);
//     [a, b, c, d, e, f, g, h, i, j, k, l, m]
// }

/// Sample density at a single point given a seed and origin.
/// Returns the main final density value (the 12th output, index 11).
pub fn sample_density_at(seed: i64, origin: Vec3) -> f64 {
    let outputs = orchestration_seeded(seed, origin);
    outputs.final_density[0]
}

/// Initialize the Perlin sampler with the given seed before computing density
pub fn orchestration_seeded(seed: i64, origin: Vec3) -> orchestration::OrchestrationOutput {
    let permutation_tables = set_perlin_seed(seed);
    orchestration::orchestration(origin, permutation_tables)
}
