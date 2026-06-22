use crate::random::{Random, RandomSplitter};

// ── Constants (from RandomSeed.java) ─────────────────────────────────────────
pub const GOLDEN_RATIO_64: i64 = -7046029254386353131_i64;
pub const SILVER_RATIO_64: i64 = 7640891576956012809_i64;

// ── Seed mixing (Stafford variant-13) ────────────────────────────────────────

/// Port of RandomSeed.mixStafford13
pub fn mix_stafford13(mut seed: i64) -> i64 {
    seed = (seed ^ ((seed as u64 >> 30) as i64)).wrapping_mul(-4658895280553007687_i64);
    seed = (seed ^ ((seed as u64 >> 27) as i64)).wrapping_mul(-7723592293110705685_i64);
    seed ^ ((seed as u64 >> 31) as i64)
}

// ── XoroshiroSeed (inner record from RandomSeed.java) ────────────────────────

#[derive(Clone, Debug)]
pub struct XoroshiroSeed {
    pub seed_lo: i64,
    pub seed_hi: i64,
}

impl XoroshiroSeed {
    pub fn new(seed_lo: i64, seed_hi: i64) -> Self {
        Self { seed_lo, seed_hi }
    }

    /// XOR both halves with the provided pair — port of XoroshiroSeed.split(long, long)
    pub fn split_with(&self, lo: i64, hi: i64) -> Self {
        Self {
            seed_lo: lo ^ self.seed_lo,
            seed_hi: hi ^ self.seed_hi,
        }
    }

    /// Apply mixStafford13 to both halves — port of XoroshiroSeed.mix()
    pub fn mix(&self) -> Self {
        Self {
            seed_lo: mix_stafford13(self.seed_lo),
            seed_hi: mix_stafford13(self.seed_hi),
        }
    }
}

// ── Seed constructors (from RandomSeed.java) ─────────────────────────────────

/// Port of RandomSeed.createXoroshiroSeed(long)
pub fn create_xoroshiro_seed(seed: i64) -> XoroshiroSeed {
    let lo = seed ^ SILVER_RATIO_64;
    let hi = lo.wrapping_add(GOLDEN_RATIO_64);
    XoroshiroSeed::new(lo, hi).mix()
}

/// Port of RandomSeed.createUnmixedXoroshiroSeed(long)
pub fn create_unmixed_xoroshiro_seed(seed: i64) -> XoroshiroSeed {
    let lo = seed ^ SILVER_RATIO_64;
    let hi = lo.wrapping_add(GOLDEN_RATIO_64);
    XoroshiroSeed::new(lo, hi)
}

/// Port of RandomSeed.createXoroshiroSeed(String) — MD5-based seed from a name string
pub fn create_xoroshiro_seed_str(seed: &str) -> XoroshiroSeed {
    let hash = md5::compute(seed.as_bytes());
    let lo = i64::from_be_bytes([
        hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7],
    ]);
    let hi = i64::from_be_bytes([
        hash[8], hash[9], hash[10], hash[11], hash[12], hash[13], hash[14], hash[15],
    ]);
    XoroshiroSeed::new(lo, hi)
}

// ── Xoroshiro128++ random implementation ─────────────────────────────────────

/// Port of Xoroshiro128PlusPlusRandomImpl — the core PRNG used by modern Minecraft worldgen.
#[derive(Clone)]
pub struct Xoroshiro128PlusPlusRandom {
    pub seed_lo: i64,
    pub seed_hi: i64,
    // Gaussian spare state (Box-Muller)
    has_spare: bool,
    spare: f64,
}

impl Xoroshiro128PlusPlusRandom {
    /// Construct directly from two seed halves.
    /// Zero-guard: if both halves are 0 the generator would be stuck,
    /// so Minecraft replaces them with the two ratio constants.
    pub fn new(mut seed_lo: i64, mut seed_hi: i64) -> Self {
        if (seed_lo | seed_hi) == 0 {
            seed_lo = GOLDEN_RATIO_64;
            seed_hi = SILVER_RATIO_64;
        }
        Self {
            seed_lo,
            seed_hi,
            has_spare: false,
            spare: 0.0,
        }
    }

    /// Construct from a `XoroshiroSeed`.
    pub fn from_seed(seed: &XoroshiroSeed) -> Self {
        Self::new(seed.seed_lo, seed.seed_hi)
    }

    /// Core step — port of Xoroshiro128PlusPlusRandomImpl.next()
    ///
    /// result = rotl(lo + hi, 17) + lo
    /// m      = hi ^ lo
    /// lo'    = rotl(lo, 49) ^ m ^ (m << 21)
    /// hi'    = rotl(m, 28)
    pub fn next_raw(&mut self) -> i64 {
        let lo = self.seed_lo;
        let hi = self.seed_hi;
        let result = i64::rotate_left(lo.wrapping_add(hi), 17).wrapping_add(lo);
        let m = hi ^ lo;
        self.seed_lo = i64::rotate_left(lo, 49) ^ m ^ (m << 21);
        self.seed_hi = i64::rotate_left(m, 28);
        result
    }

    /// Returns the top `bits` bits of the next raw value as a non-negative i32.
    pub fn next(&mut self, bits: i32) -> i32 {
        ((self.next_raw() as u64) >> (64 - bits)) as i32
    }

    /// Returns the full 64-bit raw output.
    pub fn next_long(&mut self) -> i64 {
        self.next_raw()
    }

    /// Returns a uniform f64 in [0, 1) using 53 bits of the raw output.
    pub fn next_double(&mut self) -> f64 {
        let raw = (self.next_raw() as u64) >> 11;
        raw as f64 / (1u64 << 53) as f64
    }

    /// Box-Muller Gaussian — spare value is cached in the struct.
    pub fn next_gaussian(&mut self) -> f64 {
        if self.has_spare {
            self.has_spare = false;
            return self.spare;
        }
        let (mut u, mut v, mut s);
        loop {
            u = self.next_double() * 2.0 - 1.0;
            v = self.next_double() * 2.0 - 1.0;
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

    // pub fn next_int(&mut self, bound: i32) -> i32 {
    //     if bound & (bound - 1) == 0 {
    //         (((bound as i64).wrapping_mul(self.next(31) as i64)) >> 31) as i32
    //     } else {
    //         loop {
    //             let i = self.next(31);
    //             let j = i % bound;
    //             if i - j + (bound - 1) >= 0 {
    //                 return j;
    //             }
    //         }
    //     }
    // }

    pub fn next_int(&mut self) -> i32 {
        self.next_raw() as i32
    }

    /*
    public int nextInt(int bound) {
        if (bound <= 0) {
            throw new IllegalArgumentException("Bound must be positive");
        } else {
            long l = Integer.toUnsignedLong(this.nextInt());
            long m = l * bound;
            long n = m & 4294967295L;
            if (n < bound) {
                for (int i = Integer.remainderUnsigned(~bound + 1, bound); n < i; n = m & 4294967295L) {
                    l = Integer.toUnsignedLong(this.nextInt());
                    m = l * bound;
                }
            }

            long o = m >> 32;
            return (int)o;
        }
    }*/
    pub fn next_int_bound(&mut self, bound: i32) -> i32 {
        let mut l = self.next_int() as u32 as u64 as i64;
        let mut m = l.wrapping_mul(bound as i64);
        let mut n = m & 0xFFFFFFFF;
        if n < bound as i64 {
            let threshold = remainder_unsigned((!(bound as u32)) + 1, bound as u32);
            while n < threshold as i64 {
                l = self.next_int() as i64;
                m = l.wrapping_mul(bound as i64);
                n = (m & 0xFFFFFFFF);
            }
        }
        (m >> 32) as i32
    }

    pub fn skip(&mut self, n: i64) {
        for _ in 0..n {
            self.next_raw();
        }
    }

    pub fn next_splitter(&mut self) -> XoroshiroRandomSplitter {
        let lo = self.next_long();
        let hi = self.next_long();
        XoroshiroRandomSplitter {
            seed: XoroshiroSeed::new(lo, hi),
        }
    }
}

pub struct XoroshiroRandomSplitter {
    seed: XoroshiroSeed,
}

impl XoroshiroRandomSplitter {
    pub fn split(&mut self, seed_low: i64, seed_high: i64) -> Xoroshiro128PlusPlusRandom {
        let new_seed = self.seed.split_with(seed_low, seed_high);
        Xoroshiro128PlusPlusRandom::from_seed(&new_seed)
    }
}

fn remainder_unsigned(a: u32, b: u32) -> u32 {
    ((a as u64) % (b as u64)) as u32
}
