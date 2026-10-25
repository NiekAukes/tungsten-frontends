use approx::relative_eq;

use crate::ffi::{PipelineOutput, DIMS};
use crate::paths::{HarnessConfig, Target};

const WIDTH: usize = 16;
const HEIGHT: usize = 384;
const DEPTH: usize = 16;

/// How a target's output is compared against the Rust reference.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Policy {
    /// Absolute and relative tolerance (CUDA).
    Epsilon(f64),
    /// Identical bit patterns, except that any two NaNs are equal
    /// (`tungsten-wg/SPEC.md` section 12; Java).
    Bitwise,
}

fn policy_for(target: Target, config: &HarnessConfig) -> Policy {
    match target {
        Target::Java if config.java_epsilon == 0.0 => Policy::Bitwise,
        Target::Java => Policy::Epsilon(config.java_epsilon),
        Target::Cuda | Target::Rust => Policy::Epsilon(config.epsilon),
    }
}

/// Bit-for-bit equality. `0.0` and `-0.0` differ, values 1 ULP apart differ,
/// and any two NaNs are equal regardless of payload.
fn bits_equal(a: f64, b: f64) -> bool {
    (a.is_nan() && b.is_nan()) || a.to_bits() == b.to_bits()
}

fn values_agree(reference: f64, other: f64, policy: Policy) -> bool {
    match policy {
        Policy::Bitwise => bits_equal(reference, other),
        Policy::Epsilon(eps) => relative_eq!(reference, other, epsilon = eps, max_relative = eps),
    }
}

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

/// Collects every sample index where `other` disagrees with `reference`.
fn find_mismatches(reference: &[f64], other: &[f64], policy: Policy) -> Vec<(usize, f64, f64)> {
    assert_eq!(reference.len(), DIMS, "reference output has the wrong length");
    assert_eq!(other.len(), DIMS, "target output has the wrong length");
    reference
        .iter()
        .zip(other)
        .enumerate()
        .filter(|&(_, (&r, &o))| !values_agree(r, o, policy))
        .map(|(idx, (&r, &o))| (idx, r, o))
        .collect()
}

fn split_reference(outputs: &[PipelineOutput]) -> (&PipelineOutput, &[PipelineOutput]) {
    let (reference, others) = outputs.split_first().expect("no pipeline outputs");
    assert_eq!(
        reference.target,
        Target::Rust,
        "the first pipeline output must be the Rust reference"
    );
    (reference, others)
}

/// Non-panicking version of [`assert_pipelines_agree`], used by the shrink
/// loop to test whether a candidate reduction still reproduces the bug.
pub fn pipelines_agree(outputs: &[PipelineOutput], config: &HarnessConfig) -> bool {
    let (reference, others) = split_reference(outputs);
    others.iter().all(|o| {
        find_mismatches(&reference.values, &o.values, policy_for(o.target, config)).is_empty()
    })
}

/// Phase 4: the differential oracle. Compares every non-reference target
/// against Rust and panics with a detailed report if one of them disagrees.
/// Returns the targets that were compared, which is empty when only Rust ran.
pub fn assert_pipelines_agree(outputs: &[PipelineOutput], config: &HarnessConfig) -> Vec<Target> {
    let (reference, others) = split_reference(outputs);
    let avg = |v: &[f64]| v.iter().sum::<f64>() / DIMS as f64;
    let avg_rust = avg(&reference.values);

    for other in others {
        let policy = policy_for(other.target, config);
        let name = other.target.name();
        let mismatches = find_mismatches(&reference.values, &other.values, policy);
        let avg_other = avg(&other.values);

        if mismatches.is_empty() {
            println!("\tavg_rust = {avg_rust:.17}");
            println!("\tavg_{name} = {avg_other:.17}");
            match policy {
                Policy::Bitwise => println!("Rust and {name} agree bitwise"),
                Policy::Epsilon(eps) => println!("Rust and {name} agree within epsilon={eps:e}"),
            }
            continue;
        }

        let (first_idx, first_rust, first_other) = mismatches[0];
        let (x, y, z) = index_to_coord(first_idx);
        let criterion = match policy {
            Policy::Bitwise => "are not bit-identical".to_string(),
            Policy::Epsilon(eps) => format!("differ by more than epsilon={eps:e}"),
        };

        panic!(
            "Rust/{name} density mismatch: {total} of {dims} samples {criterion}.\n\
             First mismatch at flattened index {first_idx} (grid coord x={x}, y={y}, z={z}):\n\
             \trust = {first_rust:.17} ({rust_bits:#018x})\n\
             \t{name} = {first_other:.17} ({other_bits:#018x})\n\
             \tdiff = {diff:.17}\n\n\
             \tavg_rust = {avg_rust:.17}\n\
             \tavg_{name} = {avg_other:.17}",
            total = mismatches.len(),
            dims = DIMS,
            rust_bits = first_rust.to_bits(),
            other_bits = first_other.to_bits(),
            diff = (first_rust - first_other).abs(),
        );
    }

    others.iter().map(|o| o.target).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nans_with_different_payloads_are_equal() {
        let quiet = f64::NAN;
        let other_payload = f64::from_bits(quiet.to_bits() | 0x1234);
        assert!(other_payload.is_nan());
        assert_ne!(quiet.to_bits(), other_payload.to_bits());
        assert!(bits_equal(quiet, other_payload));
        assert!(bits_equal(-quiet, quiet));
    }

    #[test]
    fn nan_is_not_equal_to_a_number() {
        assert!(!bits_equal(f64::NAN, 0.0));
        assert!(!bits_equal(1.0, f64::NAN));
    }

    #[test]
    fn signed_zeros_differ() {
        assert!(!bits_equal(0.0, -0.0));
        assert!(!bits_equal(-0.0, 0.0));
        assert!(bits_equal(-0.0, -0.0));
    }

    #[test]
    fn one_ulp_apart_differs() {
        let x = 1.0f64;
        let next = f64::from_bits(x.to_bits() + 1);
        assert!(!bits_equal(x, next));
        assert!(!bits_equal(next, x));
        assert!(bits_equal(x, x));
    }

    #[test]
    fn bitwise_policy_is_stricter_than_epsilon() {
        let x = 1.0f64;
        let next = f64::from_bits(x.to_bits() + 1);
        assert!(!values_agree(x, next, Policy::Bitwise));
        assert!(values_agree(x, next, Policy::Epsilon(1e-6)));
    }
}
