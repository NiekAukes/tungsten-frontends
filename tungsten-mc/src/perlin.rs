use std::mem::transmute;

use crate::{
    random::{CheckedRandom, Random},
    utils::PerlinNoiseSampler,
    xoroshiro::Xoroshiro128PlusPlusRandom,
};

// ------------------ Perlin Noise Sampler ------------------
pub fn create_perlin_noise_sampler(rng: &mut Xoroshiro128PlusPlusRandom) -> PerlinNoiseSampler {
    let mut p: [u8; 256] = [0; 256];

    let origin_x = rng.next_double() * 256.0;
    let origin_y = rng.next_double() * 256.0;
    let origin_z = rng.next_double() * 256.0;

    for i in 0..256 {
        p[i] = i as u8;
    }
    for i in 0..256 {
        let j = rng.next_int_bound(256 - i as i32) as usize;
        let k = p[i];
        p[i] = p[j + i];
        p[j + i] = k;
    }

    PerlinNoiseSampler {
        permutation: p,
        origin_x: origin_x,
        origin_y: origin_y,
        origin_z: origin_z,
    }
}

pub fn raw_perlin_noise_sampler(
    perm_table: &[i8; 256],
    origin_x: f64,
    origin_y: f64,
    origin_z: f64,
) -> PerlinNoiseSampler {
    // raw convert the i8 table to u8, treating the i8 as unsigned
    let permutation = unsafe { transmute(*perm_table) };
    PerlinNoiseSampler {
        permutation,
        origin_x,
        origin_y,
        origin_z,
    }
}

// ------------------ Noise Sampling ------------------
#[inline(always)]
pub fn sample_perlin(pns: &PerlinNoiseSampler, mut x: f64, mut y: f64, mut z: f64) -> f64 {
    x += pns.origin_x;
    y += pns.origin_y;
    z += pns.origin_z;

    let i = floor_int(x);
    let j = floor_int(y);
    let k = floor_int(z);

    let g = x - i as f64;
    let h = y - j as f64;
    let l = z - k as f64;

    sample_perlin_section(pns, i, j, k, g, h, l, h)
}

pub fn sample_perlin_scaled(
    pns: &PerlinNoiseSampler,
    mut x: f64,
    mut y: f64,
    mut z: f64,
    y_scale: f64,
    y_max: f64,
) -> f64 {
    x += pns.origin_x;
    y += pns.origin_y;
    z += pns.origin_z;

    let i = floor_int(x);
    let j = floor_int(y);
    let k = floor_int(z);

    let g = x - i as f64;
    let h = y - j as f64;
    let l = z - k as f64;

    let mut n = 0.0;
    if y_scale != 0.0 {
        let m = if y_max >= 0.0 && y_max < h { y_max } else { h };
        n = (m / y_scale + 1.0e-7).floor() * y_scale;
    }

    sample_perlin_section(pns, i, j, k, g, h - n, l, h)
}
#[inline(always)]
pub fn sample_perlin_section(
    pns: &PerlinNoiseSampler,
    section_x: i32,
    section_y: i32,
    section_z: i32,
    x: f64,
    y: f64,
    z: f64,
    fade_y: f64,
) -> f64 {
    let i = map(pns, section_x);
    let j = map(pns, section_x + 1);
    let k = map(pns, i + section_y);
    let l = map(pns, i + section_y + 1);
    let m = map(pns, j + section_y);
    let n = map(pns, j + section_y + 1);

    let d = grad(map(pns, k + section_z), x, y, z);
    let e = grad(map(pns, m + section_z), x - 1.0, y, z);
    let f = grad(map(pns, l + section_z), x, y - 1.0, z);
    let g = grad(map(pns, n + section_z), x - 1.0, y - 1.0, z);
    let h = grad(map(pns, k + section_z + 1), x, y, z - 1.0);
    let o = grad(map(pns, m + section_z + 1), x - 1.0, y, z - 1.0);
    let p = grad(map(pns, l + section_z + 1), x, y - 1.0, z - 1.0);
    let q = grad(map(pns, n + section_z + 1), x - 1.0, y - 1.0, z - 1.0);

    let r = fade(x);
    let s = fade(fade_y);
    let t = fade(z);

    lerp3(r, s, t, d, e, f, g, h, o, p, q)
}

// ------------------ Helper Functions ------------------
#[inline(always)]
pub fn floor_int(v: f64) -> i32 {
    let i = v as i32;
    if v < i as f64 { i - 1 } else { i }
}

#[inline(always)]
pub fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

#[inline(always)]
pub fn fade_derivative(t: f64) -> f64 {
    30.0 * t * t * (t * (t - 2.0) + 1.0)
}

#[inline(always)]
pub fn lerp(delta: f64, start: f64, end: f64) -> f64 {
    start + delta * (end - start)
}

#[inline(always)]
pub fn dot(g: [i32; 3], x: f64, y: f64, z: f64) -> f64 {
    g[0] as f64 * x + g[1] as f64 * y + g[2] as f64 * z
}

// Example 16 gradient directions
pub const GRAD3: [[i32; 3]; 16] = [
    [1, 1, 0],
    [-1, 1, 0],
    [1, -1, 0],
    [-1, -1, 0],
    [1, 0, 1],
    [-1, 0, 1],
    [1, 0, -1],
    [-1, 0, -1],
    [0, 1, 1],
    [0, -1, 1],
    [0, 1, -1],
    [0, -1, -1],
    [1, 1, 0],
    [0, -1, 1],
    [-1, 1, 0],
    [0, -1, -1],
];

#[inline(always)]
pub fn grad(hash: i32, x: f64, y: f64, z: f64) -> f64 {
    dot(GRAD3[(hash & 15) as usize], x, y, z)
}

#[inline(always)]
pub fn map(pns: &PerlinNoiseSampler, input: i32) -> i32 {
    pns.permutation[(input & 0xFF) as usize] as i32 & 0xFF
}

#[inline(always)]
pub fn lerp2(delta_x: f64, delta_y: f64, x0y0: f64, x1y0: f64, x0y1: f64, x1y1: f64) -> f64 {
    lerp(
        delta_y,
        lerp(delta_x, x0y0, x1y0),
        lerp(delta_x, x0y1, x1y1),
    )
}

#[inline(always)]
pub fn lerp3(
    delta_x: f64,
    delta_y: f64,
    delta_z: f64,
    x0y0z0: f64,
    x1y0z0: f64,
    x0y1z0: f64,
    x1y1z0: f64,
    x0y0z1: f64,
    x1y0z1: f64,
    x0y1z1: f64,
    x1y1z1: f64,
) -> f64 {
    lerp(
        delta_z,
        lerp2(delta_x, delta_y, x0y0z0, x1y0z0, x0y1z0, x1y1z0),
        lerp2(delta_x, delta_y, x0y0z1, x1y0z1, x0y1z1, x1y1z1),
    )
}

impl PerlinNoiseSampler {
    pub fn sample(&self, x: f64, y: f64, z: f64) -> f64 {
        sample_perlin(self, x, y, z)
    }

    pub fn sample_scaled(&self, x: f64, y: f64, z: f64, y_scale: f64, y_max: f64) -> f64 {
        sample_perlin_scaled(self, x, y, z, y_scale, y_max)
    }
}
