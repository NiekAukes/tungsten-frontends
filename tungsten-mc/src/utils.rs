use crate::orchestration::{PermutationTables, make_permutation_tables, orchestration};
use crate::perlin::{create_perlin_noise_sampler, sample_perlin};
use crate::random::Random;
use crate::{
    math::Vec3,
    xoroshiro::{Xoroshiro128PlusPlusRandom, create_xoroshiro_seed},
};
use std::cell::RefCell;

// Thread-local storage for the seeded Perlin noise sampler
thread_local! {
    static PERLIN_SAMPLER: RefCell<Option<&'static PermutationTables>> = RefCell::new(None);
    static CURRENT_SEED: RefCell<Option<i64>> = RefCell::new(None);
}

#[derive(Debug, Clone)]
pub struct PerlinNoiseSampler {
    pub permutation: [u8; 256],
    pub origin_x: f64,
    pub origin_y: f64,
    pub origin_z: f64,
}

/// Initialize the thread-local Perlin sampler with the given seed.
/// Must be called before any perlin() calls in this thread.
pub fn set_perlin_seed(seed: i64) -> &'static PermutationTables {
    if CURRENT_SEED.with(|s| s.borrow().map_or(true, |current| current != seed)) {
        let perm_tables = make_permutation_tables(seed);
        // leak the PermutationTables to get a 'static reference (safe because we never modify it after this)
        let perm_tables_ref: &'static PermutationTables = Box::leak(Box::new(perm_tables));

        PERLIN_SAMPLER.with(|p| *p.borrow_mut() = Some(perm_tables_ref));
        CURRENT_SEED.with(|s| *s.borrow_mut() = Some(seed));
    }
    PERLIN_SAMPLER.with(|p| p.borrow().unwrap())
}

// Helper noise / interpolation functions
pub fn hermite(t: f32, p0: f32, p1: f32, m0: f32, m1: f32) -> f32 {
    let t2 = t * t;
    let t3 = t2 * t;
    (2.0 * t3 - 3.0 * t2 + 1.0) * p0
        + (t3 - 2.0 * t2 + t) * m0
        + (-2.0 * t3 + 3.0 * t2) * p1
        + (t3 - t2) * m1
}

pub fn fade(t: Vec3) -> Vec3 {
    Vec3::new(
        t.x * t.x * t.x * (t.x * (t.x * 6.0 - 15.0) + 10.0),
        t.y * t.y * t.y * (t.y * (t.y * 6.0 - 15.0) + 10.0),
        t.z * t.z * t.z * (t.z * (t.z * 6.0 - 15.0) + 10.0),
    )
}

#[inline(always)]
pub fn abs(x: f32) -> f32 {
    x.abs()
}

#[inline(always)]
pub fn max(a: f32, b: f32) -> f32 {
    a.max(b)
}

#[inline(always)]
pub fn min(a: f32, b: f32) -> f32 {
    a.min(b)
}

#[inline(always)]
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    x.clamp(min, max)
}

pub fn make_buffer<const C: usize>() -> Box<[f32; C]> {
    // make a buffer of size C
    // without initialising it (to avoid the cost of zeroing it out, and having large arrays on the stack)
    let mut buffer: Box<[f32; C]> =
        unsafe { Box::new(std::mem::MaybeUninit::uninit().assume_init()) };
    buffer
}

pub fn make_permutation_table(
    seed: i64,
    seed_low_1: i64,
    seed_high_1: i64,
    iteration_count: i64,
    seed_low_2: i64,
    seed_high_2: i64,
) -> Box<PerlinNoiseSampler> {
    let mut rngparent =
        Xoroshiro128PlusPlusRandom::from_seed(&create_xoroshiro_seed(seed)).next_splitter();
    let mut rng1 = rngparent.split(seed_low_1, seed_high_1);
    for _ in 0..iteration_count {
        rng1.next_splitter();
    }

    let mut rng1_split = rng1.next_splitter();

    let mut rng2 = rng1_split.split(seed_low_2, seed_high_2);

    Box::new(create_perlin_noise_sampler(&mut rng2))
}

// ---------------------------------------------------------------------------
// Minecraft ImprovedNoise – Ken Perlin's improved noise (reference permutation)
// ---------------------------------------------------------------------------

/// Ken Perlin's reference permutation table, doubled to 512 to avoid extra masking.
#[rustfmt::skip]
const PERM: [u8; 512] = [
    151,160,137, 91, 90, 15,131, 13,201, 95, 96, 53,194,233,  7,225,
    140, 36,103, 30, 69,142,  8, 99, 37,240, 21, 10, 23,190,  6,148,
    247,120,234, 75,  0, 26,197, 62, 94,252,219,203,117, 35, 11, 32,
     57,177, 33, 88,237,149, 56, 87,174, 20,125,136,171,168, 68,175,
     74,165, 71,134,139, 48, 27,166, 77,146,158,231, 83,111,229,122,
     60,211,133,230,220,105, 92, 41, 55, 46,245, 40,244,102,143, 54,
     65, 25, 63,161,  1,216, 80, 73,209, 76,132,187,208, 89, 18,169,
    200,196,135,130,116,188,159, 86,164,100,109,198,173,186,  3, 64,
     52,217,226,250,124,123,  5,202, 38,147,118,126,255, 82, 85,212,
    207,206, 59,227, 47, 16, 58, 17,182,189, 28, 42,223,183,170,213,
    119,248,152,  2, 44,154,163, 70,221,153,101,155,167, 43,172,  9,
    129, 22, 39,253, 19, 98,108,110, 79,113,224,232,178,185,112,104,
    218,246, 97,228,251, 34,242,193,238,210,144, 12,191,179,162,241,
     81, 51,145,235,249, 14,239,107, 49,192,214, 31,181,199,106,157,
    184, 84,204,176,115,121, 50, 45,127,  4,150,254,138,236,205, 93,
    222,114, 67, 29, 24, 72,243,141,128,195, 78, 66,215, 61,156,180,
    // repeat
    151,160,137, 91, 90, 15,131, 13,201, 95, 96, 53,194,233,  7,225,
    140, 36,103, 30, 69,142,  8, 99, 37,240, 21, 10, 23,190,  6,148,
    247,120,234, 75,  0, 26,197, 62, 94,252,219,203,117, 35, 11, 32,
     57,177, 33, 88,237,149, 56, 87,174, 20,125,136,171,168, 68,175,
     74,165, 71,134,139, 48, 27,166, 77,146,158,231, 83,111,229,122,
     60,211,133,230,220,105, 92, 41, 55, 46,245, 40,244,102,143, 54,
     65, 25, 63,161,  1,216, 80, 73,209, 76,132,187,208, 89, 18,169,
    200,196,135,130,116,188,159, 86,164,100,109,198,173,186,  3, 64,
     52,217,226,250,124,123,  5,202, 38,147,118,126,255, 82, 85,212,
    207,206, 59,227, 47, 16, 58, 17,182,189, 28, 42,223,183,170,213,
    119,248,152,  2, 44,154,163, 70,221,153,101,155,167, 43,172,  9,
    129, 22, 39,253, 19, 98,108,110, 79,113,224,232,178,185,112,104,
    218,246, 97,228,251, 34,242,193,238,210,144, 12,191,179,162,241,
     81, 51,145,235,249, 14,239,107, 49,192,214, 31,181,199,106,157,
    184, 84,204,176,115,121, 50, 45,127,  4,150,254,138,236,205, 93,
    222,114, 67, 29, 24, 72,243,141,128,195, 78, 66,215, 61,156,180,
];

/// Minecraft's `gradDot` – picks one of 16 gradient directions (12 unique)
/// from `SimplexNoise.GRADIENT` and dots it with (x, y, z).
#[inline(always)]
fn grad_dot(hash: u8, x: f32, y: f32, z: f32) -> f32 {
    match hash & 0xF {
        0x0 => x + y,
        0x1 => -x + y,
        0x2 => x - y,
        0x3 => -x - y,
        0x4 => x + z,
        0x5 => -x + z,
        0x6 => x - z,
        0x7 => -x - z,
        0x8 => y + z,
        0x9 => -y + z,
        0xA => y - z,
        0xB => -y - z,
        0xC => y + x,  // repeat of {1,1,0}
        0xD => -y + z, // repeat of {0,-1,1}
        0xE => y - x,  // repeat of {-1,1,0}
        0xF => -y - z, // repeat of {0,-1,-1}
        _ => unreachable!(),
    }
}

#[inline(always)]
fn lerp_f32(t: f32, a: f32, b: f32) -> f32 {
    a + t * (b - a)
}

/// Uses the seeded PerlinNoiseSampler when available (initialized via set_perlin_seed).
/// Falls back to reference permutation table if no sampler is set (for backward compatibility).
/// Returns a value roughly in [-1, 1].
pub fn perlin(p: Vec3, perm_table: &PerlinNoiseSampler) -> f32 {
    let x = sample_perlin(perm_table, p.x as f64, p.y as f64, p.z as f64) as f32;
    let y = x.abs();
    x + y * 0.0
}

/// Reference implementation of Perlin noise using Ken Perlin's permutation table
/// (kept for backward compatibility and fallback).
fn perlin_reference(p: Vec3) -> f32 {
    let x = p.x;
    let y = p.y;
    let z = p.z;

    // Integer lattice coordinates (wrapping into 0..255)
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let zi = z.floor() as i32;

    // Fractional part inside the unit cube
    let xf = x - xi as f32;
    let yf = y - yi as f32;
    let zf = z - zi as f32;

    // Fade curves  (6t^5 - 15t^4 + 10t^3)
    let u = xf * xf * xf * (xf * (xf * 6.0 - 15.0) + 10.0);
    let v = yf * yf * yf * (yf * (yf * 6.0 - 15.0) + 10.0);
    let w = zf * zf * zf * (zf * (zf * 6.0 - 15.0) + 10.0);

    let xi = (xi & 255) as usize;
    let yi = (yi & 255) as usize;
    let zi = (zi & 255) as usize;

    // Hash the 8 unit-cube corners through the permutation table
    let a = PERM[xi] as usize + yi;
    let aa = PERM[a] as usize + zi;
    let ab = PERM[a + 1] as usize + zi;
    let b = PERM[xi + 1] as usize + yi;
    let ba = PERM[b] as usize + zi;
    let bb = PERM[b + 1] as usize + zi;

    // Trilinear interpolation of gradient dot-products
    lerp_f32(
        w,
        lerp_f32(
            v,
            lerp_f32(
                u,
                grad_dot(PERM[aa], xf, yf, zf),
                grad_dot(PERM[ba], xf - 1.0, yf, zf),
            ),
            lerp_f32(
                u,
                grad_dot(PERM[ab], xf, yf - 1.0, zf),
                grad_dot(PERM[bb], xf - 1.0, yf - 1.0, zf),
            ),
        ),
        lerp_f32(
            v,
            lerp_f32(
                u,
                grad_dot(PERM[aa + 1], xf, yf, zf - 1.0),
                grad_dot(PERM[ba + 1], xf - 1.0, yf, zf - 1.0),
            ),
            lerp_f32(
                u,
                grad_dot(PERM[ab + 1], xf, yf - 1.0, zf - 1.0),
                grad_dot(PERM[bb + 1], xf - 1.0, yf - 1.0, zf - 1.0),
            ),
        ),
    )
}
pub fn old_blended_noise(
    p: Vec3,
    xz_scale: f32,
    y_scale: f32,
    xz_factor: f32,
    y_factor: f32,
    smear_scale_multiplier: f32,
) -> f32 {
    0.0
}

/// Computes the Y clamped gradient, equivalent to Minecraft's:
/// `clampedMap(y, fromY, toY, fromValue, toValue)`
///   = `clampedLerp(fromValue, toValue, (y - fromY) / (toY - fromY))`
pub fn y_clamped_gradient(y: f32, from_y: f32, to_y: f32, from_value: f32, to_value: f32) -> f32 {
    let delta = (y - from_y) / (to_y - from_y);
    if delta <= 0.0 {
        from_value
    } else if delta >= 1.0 {
        to_value
    } else {
        from_value + delta * (to_value - from_value)
    }
}
