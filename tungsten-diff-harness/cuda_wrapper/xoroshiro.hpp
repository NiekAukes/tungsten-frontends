#pragma once

#include <cstdint>
#include <cmath>
#include <string>
#include <array>
#include <stdexcept>

class Xoroshiro128PlusPlusRandom;
class XoroshiroRandomSplitter;

constexpr int64_t GOLDEN_RATIO_64 = -7046029254386353131LL;
constexpr int64_t SILVER_RATIO_64 = 7640891576956012809LL;

// Left rotate for 64-bit unsigned integers
inline uint64_t rotl64(uint64_t x, int k) {
    return (x << k) | (x >> (64 - k));
}

/// Port of RandomSeed.mixStafford13
inline int64_t mix_stafford13(int64_t seed) {
    uint64_t s = static_cast<uint64_t>(seed);
    s = (s ^ (s >> 30)) * 13787848793156543929ULL; // Cast of -4658895280553007687 to unsigned
    s = (s ^ (s >> 27)) * 10723151780598845931ULL; // Cast of -7723592293110705685 to unsigned
    return static_cast<int64_t>(s ^ (s >> 31));
}

struct XoroshiroSeed {
    int64_t seed_lo;
    int64_t seed_hi;

    XoroshiroSeed(int64_t lo, int64_t hi) : seed_lo(lo), seed_hi(hi) {}

    /// XOR both halves with the provided pair — port of XoroshiroSeed.split(long, long)
    XoroshiroSeed split_with(int64_t lo, int64_t hi) const {
        return XoroshiroSeed(lo ^ seed_lo, hi ^ seed_hi);
    }

    /// Apply mixStafford13 to both halves — port of XoroshiroSeed.mix()
    XoroshiroSeed mix() const {
        return XoroshiroSeed(mix_stafford13(seed_lo), mix_stafford13(seed_hi));
    }
};

/// Port of RandomSeed.createXoroshiroSeed(long)
inline XoroshiroSeed create_xoroshiro_seed(int64_t seed) {
    int64_t lo = seed ^ SILVER_RATIO_64;
    // Unsigned math for wrapping add
    //int64_t hi = static_cast<int64_t>(static_cast<uint64_t>(lo) + static_cast<uint64_t>(GOLDEN_RATIO_64));
    int64_t hi = lo + GOLDEN_RATIO_64;
    return XoroshiroSeed(lo, hi).mix();
}

/// Port of RandomSeed.createUnmixedXoroshiroSeed(long)
inline XoroshiroSeed create_unmixed_xoroshiro_seed(int64_t seed) {
    int64_t lo = seed ^ SILVER_RATIO_64;
    int64_t hi = static_cast<int64_t>(static_cast<uint64_t>(lo) + static_cast<uint64_t>(GOLDEN_RATIO_64));
    return XoroshiroSeed(lo, hi);
}

/// Port of RandomSeed.createXoroshiroSeed(String) — MD5-based seed from a name string
// inline XoroshiroSeed create_xoroshiro_seed_str(const std::string& seed) {
//     std::array<uint8_t, 16> hash = compute_md5(seed);
    
//     // Construct 64-bit integers from Big-Endian bytes
//     uint64_t lo = 0;
//     for (int i = 0; i < 8; ++i) {
//         lo = (lo << 8) | hash[i];
//     }
    
//     uint64_t hi = 0;
//     for (int i = 8; i < 16; ++i) {
//         hi = (hi << 8) | hash[i];
//     }
    
//     return XoroshiroSeed(static_cast<int64_t>(lo), static_cast<int64_t>(hi));
// }

class XoroshiroRandomSplitter {
private:
    XoroshiroSeed seed;

public:
    explicit XoroshiroRandomSplitter(XoroshiroSeed s) : seed(s) {}

    Xoroshiro128PlusPlusRandom split(int64_t seed_low, int64_t seed_high);
};

/// Port of Xoroshiro128PlusPlusRandomImpl
class Xoroshiro128PlusPlusRandom {
public:
    int64_t seed_lo;
    int64_t seed_hi;
    bool has_spare;
    double spare;

    /// Construct directly from two seed halves.
    /// Zero-guard: if both halves are 0 the generator would be stuck,
    /// so Minecraft replaces them with the two ratio constants.
    Xoroshiro128PlusPlusRandom(int64_t s_lo, int64_t s_hi) 
        : has_spare(false), spare(0.0) {
        if ((s_lo | s_hi) == 0) {
            seed_lo = GOLDEN_RATIO_64;
            seed_hi = SILVER_RATIO_64;
        } else {
            seed_lo = s_lo;
            seed_hi = s_hi;
        }
    }

    /// Construct from a `XoroshiroSeed`.
    explicit Xoroshiro128PlusPlusRandom(const XoroshiroSeed& seed) 
        : Xoroshiro128PlusPlusRandom(seed.seed_lo, seed.seed_hi) {}

    /// Core step — port of Xoroshiro128PlusPlusRandomImpl.next()
    int64_t next_raw() {
        uint64_t lo = static_cast<uint64_t>(seed_lo);
        uint64_t hi = static_cast<uint64_t>(seed_hi);
        
        uint64_t result = rotl64(lo + hi, 17) + lo;
        uint64_t m = hi ^ lo;
        
        seed_lo = static_cast<int64_t>(rotl64(lo, 49) ^ m ^ (m << 21));
        seed_hi = static_cast<int64_t>(rotl64(m, 28));
        
        return static_cast<int64_t>(result);
    }

    /// Returns the top `bits` bits of the next raw value as a non-negative int32.
    int32_t next(int32_t bits) {
        return static_cast<int32_t>(static_cast<uint64_t>(next_raw()) >> (64 - bits));
    }

    /// Returns the full 64-bit raw output.
    int64_t next_long() {
        return next_raw();
    }

    /// Returns a uniform double in [0, 1) using 53 bits of the raw output.
    double next_double() {
        uint64_t raw = static_cast<uint64_t>(next_raw()) >> 11;
        return static_cast<double>(raw) / static_cast<double>(1ULL << 53);
    }

    /// Box-Muller Gaussian — spare value is cached in the struct.
    double next_gaussian() {
        if (has_spare) {
            has_spare = false;
            return spare;
        }
        double u, v, s;
        do {
            u = next_double() * 2.0 - 1.0;
            v = next_double() * 2.0 - 1.0;
            s = u * u + v * v;
        } while (s >= 1.0 || s == 0.0);

        s = std::sqrt(-2.0 * std::log(s) / s);
        spare = v * s;
        has_spare = true;
        return u * s;
    }

    int32_t next_int() {
        return static_cast<int32_t>(next_raw());
    }

    int32_t next_int_bound(int32_t bound) {
        if (bound <= 0) return 0; // Guard for invalid bounds
        
        // Zero-extend next_int() into a 64-bit unsigned int
        uint64_t l = static_cast<uint32_t>(next_int());
        uint64_t m = l * static_cast<uint64_t>(static_cast<uint32_t>(bound));
        uint64_t n = m & 0xFFFFFFFFULL;
        
        if (n < static_cast<uint64_t>(bound)) {
            // Equivalent to remainder_unsigned((!(bound as u32)) + 1, bound as u32)
            uint32_t threshold = (~static_cast<uint32_t>(bound) + 1) % static_cast<uint32_t>(bound);
            
            while (n < static_cast<uint64_t>(threshold)) {
                l = static_cast<uint32_t>(next_int());
                m = l * static_cast<uint64_t>(static_cast<uint32_t>(bound));
                n = m & 0xFFFFFFFFULL;
            }
        }
        return static_cast<int32_t>(m >> 32);
    }

    void skip(int64_t n) {
        for (int64_t i = 0; i < n; ++i) {
            next_raw();
        }
    }

    XoroshiroRandomSplitter next_splitter() {
        int64_t lo = next_long();
        int64_t hi = next_long();
        return XoroshiroRandomSplitter(XoroshiroSeed(lo, hi));
    }
};

// ── Out-of-line Implementations ──────────────────────────────────────────────

inline Xoroshiro128PlusPlusRandom XoroshiroRandomSplitter::split(int64_t seed_low, int64_t seed_high) {
    XoroshiroSeed new_seed = seed.split_with(seed_low, seed_high);
    return Xoroshiro128PlusPlusRandom(new_seed);
}