use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// ---------------- Gaussian Generator ----------------
// Simple Box-Muller transform for Gaussian noise
#[derive(Clone)]
pub struct GaussianGenerator {
    has_spare: bool,
    spare: f64,
}

impl GaussianGenerator {
    pub fn new() -> Self {
        Self {
            has_spare: false,
            spare: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.has_spare = false;
    }

    pub fn next(&mut self, seed: &mut i64) -> f64 {
        if self.has_spare {
            self.has_spare = false;
            return self.spare;
        }

        let (mut u, mut v, mut s);
        loop {
            u = (Self::next_double(seed) * 2.0) - 1.0;
            v = (Self::next_double(seed) * 2.0) - 1.0;
            s = u * u + v * v;
            if s < 1.0 && s != 0.0 {
                break;
            }
        }

        s = (-2.0 * s.ln() / s).sqrt();
        self.spare = v * s;
        self.has_spare = true;
        u * s
    }

    fn next_double(seed: &mut i64) -> f64 {
        let hi = Self::next_bits(seed, 26) as u64;
        let lo = Self::next_bits(seed, 27) as u64;
        ((hi << 27) | lo) as f64 / (1u64 << 53) as f64
    }

    fn next_bits(seed: &mut i64, bits: i32) -> i32 {
        *seed = (*seed * 25214903917i64 + 11i64) & 0xFFFFFFFFFFFFi64;
        (*seed >> (48 - bits)) as i32
    }
}

// ---------------- CheckedRandom ----------------
#[derive(Clone)]
pub struct CheckedRandom {
    seed: i64,
    gaussian: GaussianGenerator,
}

pub trait Random {
    fn next(&mut self, bits: i32) -> i32;
    fn next_double(&mut self) -> f64;
    fn next_gaussian(&mut self) -> f64;
    fn next_int(&mut self, bound: i32) -> i32;
    fn next_long(&mut self) -> i64;
    fn skip(&mut self, n: i64);
    fn next_splitter(&mut self) -> RandomSplitter;
}

impl CheckedRandom {
    //const SEED_MASK: u64 = (1u64 << 48) - 1;
    const MULTIPLIER: i64 = 25214903917;
    const SEED_MASK: i64 = 281474976710655;

    const INCREMENT: i64 = 11;

    pub fn new(seed: i64) -> Self {
        let mut rng = Self {
            seed: 0,
            gaussian: GaussianGenerator::new(),
        };
        rng.set_seed(seed);
        rng
    }
    pub fn set_seed(&mut self, s: i64) {
        self.seed = (s ^ Self::MULTIPLIER) & Self::SEED_MASK;
        self.gaussian.reset();
    }
}

impl Random for CheckedRandom {
    fn next(&mut self, bits: i32) -> i32 {
        self.seed = (self
            .seed
            .wrapping_mul(Self::MULTIPLIER)
            .wrapping_add(Self::INCREMENT))
            & Self::SEED_MASK;
        (self.seed >> (48 - bits)) as i32
    }

    fn next_double(&mut self) -> f64 {
        let hi = self.next(26) as u64;
        let lo = self.next(27) as u64;
        ((hi << 27) | lo) as f64 / (1u64 << 53) as f64
    }

    fn next_gaussian(&mut self) -> f64 {
        self.gaussian.next(&mut self.seed)
    }

    fn next_int(&mut self, bound: i32) -> i32 {
        if bound & (bound - 1) == 0 {
            // power of two
            (((bound as i64) * (self.next(31) as i64)) >> 31) as i32
        } else {
            loop {
                let i = self.next(31);
                let j = i % bound;
                if i - j + (bound - 1) >= 0 {
                    return j;
                }
            }
        }
    }

    fn next_long(&mut self) -> i64 {
        //((self.next(32) as i64) << 32) | (self.next(32) as i64 & 0xFFFFFFFF)
        let i = (self.next(32) as i64) << 32;
        let j = self.next(32) as i64;
        i + j
    }

    fn skip(&mut self, n: i64) {
        for _ in 0..n {
            self.next(32);
            self.next(32);
        }
    }

    fn next_splitter(&mut self) -> RandomSplitter {
        RandomSplitter::new(self.next_long())
    }
}

// ---------------- RandomSplitter ----------------
pub struct RandomSplitter {
    base_seed: i64,
}

impl RandomSplitter {
    pub fn new(seed: i64) -> Self {
        Self { base_seed: seed }
    }

    pub fn split(&self, x: i32, y: i32, z: i32) -> CheckedRandom {
        let h = Self::hash_code(x, y, z);
        CheckedRandom::new(h ^ self.base_seed)
    }

    pub fn split_str(&self, s: &str) -> CheckedRandom {
        let h = java_str_hash_code(s) as i64;
        CheckedRandom::new(h ^ self.base_seed)
    }

    pub fn split_seed(&self, seed: i64) -> CheckedRandom {
        CheckedRandom::new(seed)
    }

    fn hash_code(x: i32, y: i32, z: i32) -> i64 {
        let mut h: i64 = (x as i64).wrapping_mul(3129871) ^ (z as i64).wrapping_mul(116129781);
        h ^= y as i64;
        h = h
            .wrapping_mul(h)
            .wrapping_mul(42317861)
            .wrapping_add(h.wrapping_mul(11));
        h >> 16
    }
}

/*
private static int hashCode(int result, byte[] a, int fromIndex, int length) {
        int end = fromIndex + length;
        for (int i = fromIndex; i < end; i++) {
            result = 31 * result + a[i];
        }
        return result;
    }

     */
pub fn java_str_hash_code(s: &str) -> i32 {
    let mut h: i32 = 0;
    for c in s.chars() {
        h = h.wrapping_mul(31).wrapping_add(c as i32);
    }
    h
}
