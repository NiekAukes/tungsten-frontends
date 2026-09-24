use approx::relative_eq;

use crate::ffi::{RunResult, DIMS};

const WIDTH: usize = 16;
const HEIGHT: usize = 384;
const DEPTH: usize = 16;

/// Reverses the flattened index back into the (x, y, z) grid coordinate,
/// using tungsten-mc's chunk layout: x fastest-varying, then z, then y
/// (index = (y * DEPTH + z) * WIDTH + x).
fn index_to_coord(idx: usize) -> (usize, usize, usize) {
    let x = idx % WIDTH;
    let z = (idx / WIDTH) % DEPTH;
    let y = idx / (WIDTH * DEPTH);
    debug_assert!(y < HEIGHT, "index {idx} out of range for a single chunk");
    (x, y, z)
}

/// Collects every sample index where the Rust and CUDA pipelines disagree
/// beyond `epsilon`.
fn find_mismatches(result: &RunResult, epsilon: f64) -> Vec<(usize, f64, f64)> {
    let mut mismatches = Vec::new();

    for idx in 0..DIMS {
        let rust_value = result.rust_output[idx];
        let cuda_value = result.cuda_output[idx] as f64;

        if !relative_eq!(rust_value, cuda_value, epsilon = epsilon, max_relative = epsilon) {
            mismatches.push((idx, rust_value, cuda_value));
        }
    }

    mismatches
}

/// Non-panicking version of [`assert_pipelines_agree`], used by the shrink
/// loop to test whether a candidate reduction still reproduces the bug.
pub fn pipelines_agree(result: &RunResult, epsilon: f64) -> bool {
    find_mismatches(result, epsilon).is_empty()
}

/// Phase 4: the differential oracle. Panics with a detailed report on the
/// first mismatch (and a summary of how many others were found) if the Rust
/// and CUDA pipelines disagree beyond `epsilon`.
pub fn assert_pipelines_agree(result: &RunResult, epsilon: f64) {
    let mismatches = find_mismatches(result, epsilon);

    let avg_rust = result.rust_output.iter().copied().sum::<f64>() / DIMS as f64;
    let avg_cuda = result.cuda_output.iter().map(|&v| v as f64).sum::<f64>() / DIMS as f64;
    if mismatches.is_empty() {
        println!("\tavg_rust = {avg_rust:.17}");
        println!("\tavg_cuda = {avg_cuda:.17}");
        println!("Pipelines agree within epsilon={epsilon:e}");
        return;
    }

    let (first_idx, first_rust, first_cuda) = mismatches[0];
    let (x, y, z) = index_to_coord(first_idx);

    panic!(
        "Rust/CUDA density mismatch: {total} of {dims} samples differ by more than epsilon={epsilon:e}.\n\
         First mismatch at flattened index {first_idx} (grid coord x={x}, y={y}, z={z}):\n\
         \trust = {first_rust:.17}\n\
         \tcuda = {first_cuda:.17}\n\
         \tdiff = {diff:.17}\n\n\
         \tavg_rust = {avg_rust:.17}\n\
         \tavg_cuda = {avg_cuda:.17}",
         
        total = mismatches.len(),
        dims = DIMS,
        diff = (first_rust - first_cuda).abs(),
    );
}
