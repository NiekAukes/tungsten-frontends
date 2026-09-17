// Rust half of the differential harness's Phase 1: a thin `extern "C"` shim
// around the generated worldgen code, compiled to a cdylib via `cargo build`
// (invoked as a subprocess by the harness, see `src/rust_build.rs`).
//
// The generated `density.rs` (RCL output of tungsten-mc-compile) is written
// exactly like tungsten-mc/build.rs writes `generated_worldgen.rs`: it
// defines `pub mod density_function { ... }` and `pub mod orchestration
// { ... }` and expects `mathf64::*` / `utilsf64::*` to already be in scope.
// The supporting modules below are the same files tungsten-mc/src uses, so
// this wrapper stays in lockstep with the real crate instead of duplicating
// logic.
#![allow(warnings)]

#[path = "../../../tungsten-mc/src/math.rs"]
pub mod math;
#[path = "../../../tungsten-mc/src/mathf64.rs"]
pub mod mathf64;
#[path = "../../../tungsten-mc/src/perlin.rs"]
pub mod perlin;
#[path = "../../../tungsten-mc/src/random.rs"]
pub mod random;
#[path = "../../../tungsten-mc/src/utils.rs"]
pub mod utils;
#[path = "../../../tungsten-mc/src/utilsf64.rs"]
pub mod utilsf64;
#[path = "../../../tungsten-mc/src/xoroshiro.rs"]
pub mod xoroshiro;

use mathf64::*;
use utilsf64::*;

// DENSITY_RS_PATH is set at build time by the harness (see rust_build.rs)
// to the absolute path of the generated density.rs under test.
include!(env!("DENSITY_RS_PATH"));

/// Number of `f64` samples in a 16x384x16 final-density chunk.
pub const DIMS: usize = 16 * 384 * 16;

/// FFI entry point loaded by the diff harness via `libloading`.
///
/// Writes `DIMS` f64 values to `out_ptr`. `out_len` must equal `DIMS`;
/// mismatches are reported by writing nothing and returning a non-zero code.
#[unsafe(no_mangle)]
pub extern "C" fn run_rust_pipeline(
    origin_x: f64,
    origin_y: f64,
    origin_z: f64,
    seed: i64,
    out_ptr: *mut f64,
    out_len: usize,
) -> i32 {
    if out_len != DIMS {
        return 1;
    }

    let origin = mathf64::Vec3 {
        x: origin_x,
        y: origin_y,
        z: origin_z,
    };
    let perm_tables = utils::set_perlin_seed(seed);
    let result = orchestration::orchestrate_final_density(origin, perm_tables);

    // SAFETY: caller (the diff harness) guarantees out_ptr points at a
    // writable buffer of at least `out_len` f64s, checked above.
    unsafe {
        std::ptr::copy_nonoverlapping(result.as_ptr(), out_ptr, DIMS);
    }
    0
}


pub fn main() {
    // run with 0,0,0 origin and seed 0
    let mut output = vec![0.0f64; DIMS];
    let ret = run_rust_pipeline(0.0, 0.0, 0.0, 0, output.as_mut_ptr(), output.len());
    let perm_tables = utils::set_perlin_seed(0);
    // println!("Perm tables: {:?}", perm_tables);
    println!("Return code: {}", ret);
    println!("First 10 output values: {:?}", &output[..10]);
}