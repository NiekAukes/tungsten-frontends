use crate::orchestration::{PermutationTables, make_permutation_tables, orchestration};
use crate::perlin::{create_perlin_noise_sampler, sample_perlin, sample_perlin_scaled};
use crate::random::Random;
use crate::xoroshiro::{self, create_xoroshiro_seed_str};
use crate::{
    mathf64::Vec3,
    xoroshiro::{Xoroshiro128PlusPlusRandom, create_xoroshiro_seed},
};
use std::cell::RefCell;

// Re-export PerlinNoiseSampler from utils so perlin.rs functions are compatible
pub use crate::utils::PerlinNoiseSampler;

/// Initialize the thread-local Perlin sampler with the given seed.
/// Must be called before any perlin() calls in this thread.
pub use crate::utils::set_perlin_seed;

// Helper noise / interpolation functions
// pub fn hermite(t: f32, p0: f32, p1: f32, m0: f32, m1: f32) -> f32 {
//     let t2 = t * t;
//     let t3 = t2 * t;
//     (2.0 * t3 - 3.0 * t2 + 1.0) * p0
//         + (t3 - 2.0 * t2 + t) * m0
//         + (-2.0 * t3 + 3.0 * t2) * p1
//         + (t3 - t2) * m1
// }

#[inline(always)]
pub fn hermite(t: f32, p0: f32, p1: f32, m0: f32, m1: f32, h_minus_g: f32) -> f32 {
    // 1. Compute intermediate tangents matching Minecraft's:
    // float p = l * (h - g) - (o - n);
    // float q = -m * (h - g) + (o - n);
    let p = (m0 * h_minus_g) - (p1 - p0);
    let q = (-m1 * h_minus_g) + (p1 - p0);

    // 2. Perform localized 32-bit linear interpolations
    let lerp1 = p0 + t * (p1 - p0);
    let lerp2 = p + t * (q - p);

    // 3. Enforce left-to-right evaluation grouping via parentheses
    lerp1 + ((t * (1.0_f32 - t)) * lerp2)
}

#[inline(always)]
pub fn advanced_hermite<const N: usize>(
    spline_locations: [f32; N],
    spline_values: [f32; N],
    spline_derivatives: [f32; N],
    coordinate: f32,
    index: i32,
) -> f32 {
    // let h_minus_g = second_value - first_value;
    // hermite(
    //     t,
    //     first_value,
    //     second_value,
    //     first_derivative,
    //     second_derivative,
    //     h_minus_g,
    // )
    let index: usize = index as usize;
    if index == 0 || index >= N {
        // extrapolate
        let value = spline_values[index.min(N - 1)];
        let derivative = spline_derivatives[index.min(N - 1)];
        let location = spline_locations[index.min(N - 1)];
        return value + derivative * (coordinate - location);
    }
    let index_minus_1 = index - 1;
    let h_minus_g = spline_values[index] - spline_values[index_minus_1];
    let t = (coordinate - spline_locations[index_minus_1])
        / (spline_locations[index] - spline_locations[index_minus_1]);
    let value_minus_1 = spline_values[index_minus_1];
    let value = spline_values[index];
    let derivative_minus_1 = spline_derivatives[index_minus_1];
    let derivative = spline_derivatives[index];
    hermite(
        t,
        value_minus_1,
        value,
        derivative_minus_1,
        derivative,
        h_minus_g,
    )
}

#[inline(always)]
pub fn fade(t: Vec3) -> Vec3 {
    Vec3::new(
        t.x * t.x * t.x * (t.x * (t.x * 6.0_f64 - 15.0) + 10.0),
        t.y * t.y * t.y * (t.y * (t.y * 6.0_f64 - 15.0) + 10.0),
        t.z * t.z * t.z * (t.z * (t.z * 6.0_f64 - 15.0) + 10.0),
    )
}

#[inline(always)]
pub fn abs(x: f64) -> f64 {
    x.abs()
}

#[inline(always)]
pub fn max(a: f64, b: f64) -> f64 {
    a.max(b)
}

#[inline(always)]
pub fn min(a: f64, b: f64) -> f64 {
    a.min(b)
}

#[inline(always)]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    x.clamp(min, max)
}

pub fn make_buffer<const C: usize>() -> Box<[f64; C]> {
    // make a buffer of size C
    // without initialising it (to avoid the cost of zeroing it out, and having large arrays on the stack)
    //let buffer: Box<[f64; C]> = unsafe { Box::new(std::mem::MaybeUninit::uninit().assume_init()) };
    let buffer = Box::new_uninit();
    unsafe { buffer.assume_init() }
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
fn grad_dot(hash: u8, x: f64, y: f64, z: f64) -> f64 {
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
fn lerp_f64(t: f64, a: f64, b: f64) -> f64 {
    a + t * (b - a)
}

/// Uses the seeded PerlinNoiseSampler when available (initialized via set_perlin_seed).
/// Falls back to reference permutation table if no sampler is set (for backward compatibility).
/// Returns a value roughly in [-1, 1].
#[inline(always)]
pub fn perlin(p: Vec3, perm_table: &PerlinNoiseSampler) -> f64 {
    sample_perlin(perm_table, p.x, p.y, p.z)
}

/// Reference implementation of Perlin noise using Ken Perlin's permutation table
/// (kept for backward compatibility and fallback).
fn perlin_reference(p: Vec3) -> f64 {
    let x = p.x;
    let y = p.y;
    let z = p.z;

    // Integer lattice coordinates (wrapping into 0..255)
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let zi = z.floor() as i32;

    // Fractional part inside the unit cube
    let xf = x - xi as f64;
    let yf = y - yi as f64;
    let zf = z - zi as f64;

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
    lerp_f64(
        w,
        lerp_f64(
            v,
            lerp_f64(
                u,
                grad_dot(PERM[aa], xf, yf, zf),
                grad_dot(PERM[ba], xf - 1.0, yf, zf),
            ),
            lerp_f64(
                u,
                grad_dot(PERM[ab], xf, yf - 1.0, zf),
                grad_dot(PERM[bb], xf - 1.0, yf - 1.0, zf),
            ),
        ),
        lerp_f64(
            v,
            lerp_f64(
                u,
                grad_dot(PERM[aa + 1], xf, yf, zf - 1.0),
                grad_dot(PERM[ba + 1], xf - 1.0, yf, zf - 1.0),
            ),
            lerp_f64(
                u,
                grad_dot(PERM[ab + 1], xf, yf - 1.0, zf - 1.0),
                grad_dot(PERM[bb + 1], xf - 1.0, yf - 1.0, zf - 1.0),
            ),
        ),
    )
}
// pub fn old_blended_noise(
//     p: Vec3,
//     xz_scale: f64,
//     y_scale: f64,
//     xz_factor: f64,
//     y_factor: f64,
//     smear_scale_multiplier: f64,
// ) -> f64 {
//     let sampler = crate::old_blended_noise::InterpolatedNoiseSampler::create_base3d(
//         xz_scale,
//         y_scale,
//         xz_factor,
//         y_factor,
//         smear_scale_multiplier,
//     );
//     sampler.sample(p.x, p.y, p.z)
// }

/*
package net.minecraft.util.math.noise;

import com.google.common.annotations.VisibleForTesting;
import com.mojang.serialization.Codec;
import com.mojang.serialization.MapCodec;
import com.mojang.serialization.codecs.RecordCodecBuilder;
import java.util.Locale;
import java.util.stream.IntStream;
import net.minecraft.util.dynamic.CodecHolder;
import net.minecraft.util.math.MathHelper;
import net.minecraft.util.math.random.Random;
import net.minecraft.util.math.random.Xoroshiro128PlusPlusRandom;
import net.minecraft.world.gen.densityfunction.DensityFunction;

public class InterpolatedNoiseSampler implements DensityFunction.Base {
    private static final Codec<Double> SCALE_AND_FACTOR_RANGE = Codec.doubleRange(0.001, 1000.0);
    private static final MapCodec<InterpolatedNoiseSampler> MAP_CODEC = RecordCodecBuilder.mapCodec(
        instance -> instance.group(
                SCALE_AND_FACTOR_RANGE.fieldOf("xz_scale").forGetter(interpolatedNoiseSampler -> interpolatedNoiseSampler.xzScale),
                SCALE_AND_FACTOR_RANGE.fieldOf("y_scale").forGetter(interpolatedNoiseSampler -> interpolatedNoiseSampler.yScale),
                SCALE_AND_FACTOR_RANGE.fieldOf("xz_factor").forGetter(interpolatedNoiseSampler -> interpolatedNoiseSampler.xzFactor),
                SCALE_AND_FACTOR_RANGE.fieldOf("y_factor").forGetter(interpolatedNoiseSampler -> interpolatedNoiseSampler.yFactor),
                Codec.doubleRange(1.0, 8.0).fieldOf("smear_scale_multiplier").forGetter(interpolatedNoiseSampler -> interpolatedNoiseSampler.smearScaleMultiplier)
            )
            .apply(instance, InterpolatedNoiseSampler::createBase3dNoiseFunction)
    );
    public static final CodecHolder<InterpolatedNoiseSampler> CODEC = CodecHolder.of(MAP_CODEC);
    private final OctavePerlinNoiseSampler lowerInterpolatedNoise;
    private final OctavePerlinNoiseSampler upperInterpolatedNoise;
    private final OctavePerlinNoiseSampler interpolationNoise;
    private final double scaledXzScale;
    private final double scaledYScale;
    private final double xzFactor;
    private final double yFactor;
    private final double smearScaleMultiplier;
    private final double maxValue;
    private final double xzScale;
    private final double yScale;

    public static InterpolatedNoiseSampler createBase3dNoiseFunction(double xzScale, double yScale, double xzFactor, double yFactor, double smearScaleMultiplier) {
        return new InterpolatedNoiseSampler(new Xoroshiro128PlusPlusRandom(0L), xzScale, yScale, xzFactor, yFactor, smearScaleMultiplier);
    }

    private InterpolatedNoiseSampler(
        OctavePerlinNoiseSampler lowerInterpolatedNoise,
        OctavePerlinNoiseSampler upperInterpolatedNoise,
        OctavePerlinNoiseSampler interpolationNoise,
        double xzScale,
        double yScale,
        double xzFactor,
        double yFactor,
        double smearScaleMultiplier
    ) {
        this.lowerInterpolatedNoise = lowerInterpolatedNoise;
        this.upperInterpolatedNoise = upperInterpolatedNoise;
        this.interpolationNoise = interpolationNoise;
        this.xzScale = xzScale;
        this.yScale = yScale;
        this.xzFactor = xzFactor;
        this.yFactor = yFactor;
        this.smearScaleMultiplier = smearScaleMultiplier;
        this.scaledXzScale = 684.412 * this.xzScale;
        this.scaledYScale = 684.412 * this.yScale;
        this.maxValue = lowerInterpolatedNoise.method_40556(this.scaledYScale);
    }

    @VisibleForTesting
    public InterpolatedNoiseSampler(Random random, double xzScale, double yScale, double xzFactor, double yFactor, double smearScaleMultiplier) {
        this(
            OctavePerlinNoiseSampler.createLegacy(random, IntStream.rangeClosed(-15, 0)),
            OctavePerlinNoiseSampler.createLegacy(random, IntStream.rangeClosed(-15, 0)),
            OctavePerlinNoiseSampler.createLegacy(random, IntStream.rangeClosed(-7, 0)),
            xzScale,
            yScale,
            xzFactor,
            yFactor,
            smearScaleMultiplier
        );
    }

    public InterpolatedNoiseSampler copyWithRandom(Random random) {
        return new InterpolatedNoiseSampler(random, this.xzScale, this.yScale, this.xzFactor, this.yFactor, this.smearScaleMultiplier);
    }

    @Override
    public double sample(DensityFunction.NoisePos pos) {
        double d = pos.blockX() * this.scaledXzScale;
        double e = pos.blockY() * this.scaledYScale;
        double f = pos.blockZ() * this.scaledXzScale;
        double g = d / this.xzFactor;
        double h = e / this.yFactor;
        double i = f / this.xzFactor;
        double j = this.scaledYScale * this.smearScaleMultiplier;
        double k = j / this.yFactor;
        double l = 0.0;
        double m = 0.0;
        double n = 0.0;
        boolean bl = true;
        double o = 1.0;

        for (int p = 0; p < 8; p++) {
            PerlinNoiseSampler perlinNoiseSampler = this.interpolationNoise.getOctave(p);
            if (perlinNoiseSampler != null) {
                n += perlinNoiseSampler.sample(
                        OctavePerlinNoiseSampler.maintainPrecision(g * o),
                        OctavePerlinNoiseSampler.maintainPrecision(h * o),
                        OctavePerlinNoiseSampler.maintainPrecision(i * o),
                        k * o,
                        h * o
                    )
                    / o;
            }

            o /= 2.0;
        }

        double q = (n / 10.0 + 1.0) / 2.0;
        boolean bl2 = q >= 1.0;
        boolean bl3 = q <= 0.0;
        o = 1.0;

        for (int r = 0; r < 16; r++) {
            double s = OctavePerlinNoiseSampler.maintainPrecision(d * o);
            double t = OctavePerlinNoiseSampler.maintainPrecision(e * o);
            double u = OctavePerlinNoiseSampler.maintainPrecision(f * o);
            double v = j * o;
            if (!bl2) {
                PerlinNoiseSampler perlinNoiseSampler2 = this.lowerInterpolatedNoise.getOctave(r);
                if (perlinNoiseSampler2 != null) {
                    l += perlinNoiseSampler2.sample(s, t, u, v, e * o) / o;
                }
            }

            if (!bl3) {
                PerlinNoiseSampler perlinNoiseSampler2 = this.upperInterpolatedNoise.getOctave(r);
                if (perlinNoiseSampler2 != null) {
                    m += perlinNoiseSampler2.sample(s, t, u, v, e * o) / o;
                }
            }

            o /= 2.0;
        }

        return MathHelper.clampedLerp(l / 512.0, m / 512.0, q) / 128.0;
    }

    @Override
    public double minValue() {
        return -this.maxValue();
    }

    @Override
    public double maxValue() {
        return this.maxValue;
    }

    @VisibleForTesting
    public void addDebugInfo(StringBuilder info) {
        info.append("BlendedNoise{minLimitNoise=");
        this.lowerInterpolatedNoise.addDebugInfo(info);
        info.append(", maxLimitNoise=");
        this.upperInterpolatedNoise.addDebugInfo(info);
        info.append(", mainNoise=");
        this.interpolationNoise.addDebugInfo(info);
        info.append(
                String.format(
                    Locale.ROOT,
                    ", xzScale=%.3f, yScale=%.3f, xzMainScale=%.3f, yMainScale=%.3f, cellWidth=4, cellHeight=8",
                    684.412,
                    684.412,
                    8.555150000000001,
                    4.277575000000001
                )
            )
            .append('}');
    }

    @Override
    public CodecHolder<? extends DensityFunction> getCodecHolder() {
        return CODEC;
    }
}





public class OctavePerlinNoiseSampler {
    private static final int field_31704 = 33554432;
    private final PerlinNoiseSampler[] octaveSamplers;
    private final int firstOctave;
    private final DoubleList amplitudes;
    private final double persistence;
    private final double lacunarity;
    private final double maxValue;

    @Deprecated
    public static OctavePerlinNoiseSampler createLegacy(Random random, IntStream octaves) {
        return new OctavePerlinNoiseSampler(
            random, calculateAmplitudes(new IntRBTreeSet((Collection<? extends Integer>)octaves.boxed().collect(ImmutableList.toImmutableList()))), false
        );
    }

    @Deprecated
    public static OctavePerlinNoiseSampler createLegacy(Random random, int offset, DoubleList amplitudes) {
        return new OctavePerlinNoiseSampler(random, Pair.of(offset, amplitudes), false);
    }

    public static OctavePerlinNoiseSampler create(Random random, IntStream octaves) {
        return create(random, (List<Integer>)octaves.boxed().collect(ImmutableList.toImmutableList()));
    }

    public static OctavePerlinNoiseSampler create(Random random, List<Integer> octaves) {
        return new OctavePerlinNoiseSampler(random, calculateAmplitudes(new IntRBTreeSet(octaves)), true);
    }

    public static OctavePerlinNoiseSampler create(Random random, int offset, double firstAmplitude, double... amplitudes) {
        DoubleArrayList doubleArrayList = new DoubleArrayList(amplitudes);
        doubleArrayList.add(0, firstAmplitude);
        return new OctavePerlinNoiseSampler(random, Pair.of(offset, doubleArrayList), true);
    }

    public static OctavePerlinNoiseSampler create(Random random, int offset, DoubleList amplitudes) {
        return new OctavePerlinNoiseSampler(random, Pair.of(offset, amplitudes), true);
    }

    private static Pair<Integer, DoubleList> calculateAmplitudes(IntSortedSet octaves) {
        if (octaves.isEmpty()) {
            throw new IllegalArgumentException("Need some octaves!");
        } else {
            int i = -octaves.firstInt();
            int j = octaves.lastInt();
            int k = i + j + 1;
            if (k < 1) {
                throw new IllegalArgumentException("Total number of octaves needs to be >= 1");
            } else {
                DoubleList doubleList = new DoubleArrayList(new double[k]);
                IntBidirectionalIterator intBidirectionalIterator = octaves.iterator();

                while (intBidirectionalIterator.hasNext()) {
                    int l = intBidirectionalIterator.nextInt();
                    doubleList.set(l + i, 1.0);
                }

                return Pair.of(-i, doubleList);
            }
        }
    }

    protected OctavePerlinNoiseSampler(Random random, Pair<Integer, DoubleList> firstOctaveAndAmplitudes, boolean xoroshiro) {
        this.firstOctave = firstOctaveAndAmplitudes.getFirst();
        this.amplitudes = firstOctaveAndAmplitudes.getSecond();
        int i = this.amplitudes.size();
        int j = -this.firstOctave;
        this.octaveSamplers = new PerlinNoiseSampler[i];
        if (xoroshiro) {
            RandomSplitter randomSplitter = random.nextSplitter();

            for (int k = 0; k < i; k++) {
                if (this.amplitudes.getDouble(k) != 0.0) {
                    int l = this.firstOctave + k;
                    this.octaveSamplers[k] = new PerlinNoiseSampler(randomSplitter.split("octave_" + l));
                }
            }
        } else {
            PerlinNoiseSampler perlinNoiseSampler = new PerlinNoiseSampler(random);
            if (j >= 0 && j < i) {
                double d = this.amplitudes.getDouble(j);
                if (d != 0.0) {
                    this.octaveSamplers[j] = perlinNoiseSampler;
                }
            }

            for (int kx = j - 1; kx >= 0; kx--) {
                if (kx < i) {
                    double e = this.amplitudes.getDouble(kx);
                    if (e != 0.0) {
                        this.octaveSamplers[kx] = new PerlinNoiseSampler(random);
                    } else {
                        skipCalls(random);
                    }
                } else {
                    skipCalls(random);
                }
            }

            if (Arrays.stream(this.octaveSamplers).filter(Objects::nonNull).count() != this.amplitudes.stream().filter(amplitude -> amplitude != 0.0).count()) {
                throw new IllegalStateException("Failed to create correct number of noise levels for given non-zero amplitudes");
            }

            if (j < i - 1) {
                throw new IllegalArgumentException("Positive octaves are temporarily disabled");
            }
        }

        this.lacunarity = Math.pow(2.0, -j);
        this.persistence = Math.pow(2.0, i - 1) / (Math.pow(2.0, i) - 1.0);
        this.maxValue = this.getTotalAmplitude(2.0);
    }

    protected double getMaxValue() {
        return this.maxValue;
    }

    private static void skipCalls(Random random) {
        random.skip(262);
    }

    public double sample(double x, double y, double z) {
        return this.sample(x, y, z, 0.0, 0.0, false);
    }

    @Deprecated
    public double sample(double x, double y, double z, double yScale, double yMax, boolean useOrigin) {
        double d = 0.0;
        double e = this.lacunarity;
        double f = this.persistence;

        for (int i = 0; i < this.octaveSamplers.length; i++) {
            PerlinNoiseSampler perlinNoiseSampler = this.octaveSamplers[i];
            if (perlinNoiseSampler != null) {
                double g = perlinNoiseSampler.sample(
                    maintainPrecision(x * e), useOrigin ? -perlinNoiseSampler.originY : maintainPrecision(y * e), maintainPrecision(z * e), yScale * e, yMax * e
                );
                d += this.amplitudes.getDouble(i) * g * f;
            }

            e *= 2.0;
            f /= 2.0;
        }

        return d;
    }

    public double method_40556(double d) {
        return this.getTotalAmplitude(d + 2.0);
    }

    private double getTotalAmplitude(double scale) {
        double d = 0.0;
        double e = this.persistence;

        for (int i = 0; i < this.octaveSamplers.length; i++) {
            PerlinNoiseSampler perlinNoiseSampler = this.octaveSamplers[i];
            if (perlinNoiseSampler != null) {
                d += this.amplitudes.getDouble(i) * scale * e;
            }

            e /= 2.0;
        }

        return d;
    }

    @Nullable
    public PerlinNoiseSampler getOctave(int octave) {
        return this.octaveSamplers[this.octaveSamplers.length - 1 - octave];
    }

    public static double maintainPrecision(double value) {
        return value - MathHelper.lfloor(value / 3.3554432E7 + 0.5) * 3.3554432E7;
    }

    protected int getFirstOctave() {
        return this.firstOctave;
    }

    protected DoubleList getAmplitudes() {
        return this.amplitudes;
    }

    @VisibleForTesting
    public void addDebugInfo(StringBuilder info) {
        info.append("PerlinNoise{");
        List<String> list = this.amplitudes.stream().map(double_ -> String.format(Locale.ROOT, "%.2f", double_)).toList();
        info.append("first octave: ").append(this.firstOctave).append(", amplitudes: ").append(list).append(", noise levels: [");

        for (int i = 0; i < this.octaveSamplers.length; i++) {
            info.append(i).append(": ");
            PerlinNoiseSampler perlinNoiseSampler = this.octaveSamplers[i];
            if (perlinNoiseSampler == null) {
                info.append("null");
            } else {
                perlinNoiseSampler.addDebugInfo(info);
            }

            info.append(", ");
        }

        info.append("]");
        info.append("}");
    }
}


*/

/// Equivalent to `OctavePerlinNoiseSampler.createLegacy(random, IntStream.rangeClosed(-n, 0))`.
/// All amplitudes are 1.0 so every octave slot is filled.
/// Creation order matches Java: index `n-1` first, then `n-2` down to `0`.
pub fn create_legacy<const NUM: usize>(
    rng: &mut Xoroshiro128PlusPlusRandom,
    samplers: &mut [PerlinNoiseSampler; NUM],
) {
    if NUM == 0 {
        return;
    }

    let j = NUM - 1; // Represents -firstOctave where firstOctave is e.g. -15

    // In Java:
    // PerlinNoiseSampler perlinNoiseSampler = new PerlinNoiseSampler(random);
    // this.octaveSamplers[j] = perlinNoiseSampler;

    // Note: If `samplers` was created via `assume_init()` on uninitialized memory,
    // and `PerlinNoiseSampler` implements `Drop`, standard assignment will attempt
    // to drop garbage data and cause UB. Use `std::ptr::write` to be safe.
    unsafe {
        std::ptr::write(&mut samplers[j], create_perlin_noise_sampler(rng));
    }

    // In Java:
    // for (int kx = j - 1; kx >= 0; kx--) { ... }
    for kx in (0..j).rev() {
        unsafe {
            std::ptr::write(&mut samplers[kx], create_perlin_noise_sampler(rng));
        }
    }
}

/// Port of `InterpolatedNoiseSampler`
pub struct InterpolatedNoiseSampler {
    pub lower: [PerlinNoiseSampler; 16], // rangeClosed(-15, 0)
    pub upper: [PerlinNoiseSampler; 16], // rangeClosed(-15, 0)
    pub interpolation: [PerlinNoiseSampler; 8], // rangeClosed(-7, 0)
                                         // scaled_xz_scale: f64,
                                         // scaled_y_scale: f64,
                                         // xz_factor: f64,
                                         // y_factor: f64,
                                         // smear_scale_multiplier: f64,
}

impl InterpolatedNoiseSampler {
    /// Equivalent to `new InterpolatedNoiseSampler(random, xzScale, yScale, xzFactor, yFactor, smearScaleMultiplier)`.
    pub fn new_boxed(
        rng: &mut Xoroshiro128PlusPlusRandom,
        // xz_scale: f64,
        // y_scale: f64,
        // xz_factor: f64,
        // y_factor: f64,
        // smear_scale_multiplier: f64,
    ) -> Box<InterpolatedNoiseSampler> {
        // let lower = OctavePerlinNoiseSampler::create_legacy(rng, 16); // rangeClosed(-15, 0)
        // let upper = OctavePerlinNoiseSampler::create_legacy(rng, 16);
        // let interpolation = OctavePerlinNoiseSampler::create_legacy(rng, 8); // rangeClosed(-7, 0)

        // allocate empty uninitialized data for the samplers, then fill it in-place to avoid stack overflow from large arrays
        let mut base: Box<InterpolatedNoiseSampler> = unsafe { Box::new_uninit().assume_init() };
        // InterpolatedNoiseSampler {
        //     lower,
        //     upper,
        //     interpolation,
        //     scaled_xz_scale: 684.412 * xz_scale,
        //     scaled_y_scale: 684.412 * y_scale,
        //     xz_factor,
        //     y_factor,
        //     smear_scale_multiplier,
        // }
        create_legacy(rng, &mut base.lower);
        create_legacy(rng, &mut base.upper);
        create_legacy(rng, &mut base.interpolation);
        // base.scaled_xz_scale = 684.412 * xz_scale;
        // base.scaled_y_scale = 684.412 * y_scale;
        // base.xz_factor = xz_factor;
        // base.y_factor = y_factor;
        // base.smear_scale_multiplier = smear_scale_multiplier;

        return base;
    }

    /// Equivalent to `new InterpolatedNoiseSampler(new Xoroshiro128PlusPlusRandom(0L), ...)`.
    pub fn create_base3d(seed: i64) -> Box<InterpolatedNoiseSampler> {
        let mut rng = Xoroshiro128PlusPlusRandom::from_seed(&create_xoroshiro_seed(seed));
        let mut random_splitter = rng.next_splitter();

        let xoroshiro_seed = create_xoroshiro_seed_str("minecraft:terrain");
        let mut rng = random_splitter.split(xoroshiro_seed.seed_lo, xoroshiro_seed.seed_hi);
        Self::new_boxed(&mut rng)
    }
}

pub fn make_base3d_perm_table(seed: i64) -> Box<InterpolatedNoiseSampler> {
    InterpolatedNoiseSampler::create_base3d(seed)
}

const BASE_3D_XZ_SCALE: f64 = 684.412;

#[inline(always)]
pub fn base3d_noise(
    p: Vec3,
    sampler: &InterpolatedNoiseSampler,
    smear_scale_multiplier: f64,
    xz_factor: f64,
    scaled_xz_scale: f64,
    y_factor: f64,
    scaled_y_scale: f64,
) -> f64 {
    let x = p.x;
    let y = p.y;
    let z = p.z;
    let d = x * scaled_xz_scale * BASE_3D_XZ_SCALE;
    let e = y * scaled_y_scale * BASE_3D_XZ_SCALE;
    let f = z * scaled_xz_scale * BASE_3D_XZ_SCALE;

    let g = d / xz_factor;
    let h = e / y_factor;
    let iz = f / xz_factor;
    let j = scaled_y_scale * BASE_3D_XZ_SCALE * smear_scale_multiplier;
    let k = j / y_factor;

    let mut l = 0.0_f64;
    let mut m = 0.0_f64;
    let mut n = 0.0_f64;
    let mut o = 1.0_f64;
    let mut inv_o = 1.0_f64;

    for p in (0..8).rev() {
        n += sample_perlin_scaled(
            &sampler.interpolation[p],
            maintain_precision(g * o),
            maintain_precision(h * o),
            maintain_precision(iz * o),
            k * o,
            h * o,
        ) * inv_o;
        o *= 0.5;
        inv_o *= 2.0;
    }

    let q = (n * 0.1 + 1.0) * 0.5;
    let bl2 = q >= 1.0;
    let bl3 = q <= 0.0;
    o = 1.0;

    for r in (0..16).rev() {
        let s = maintain_precision(d * o);
        let t = maintain_precision(e * o);
        let u = maintain_precision(f * o);
        let v = j * o;

        if !bl2 {
            l += sample_perlin_scaled(&sampler.lower[r], s, t, u, v, e * o) / o;
        }
        if !bl3 {
            m += sample_perlin_scaled(&sampler.upper[r], s, t, u, v, e * o) / o;
        }

        o *= 0.5;
    }

    clamped_lerp(l * 0.001953125, m * 0.001953125, q) * 0.0078125
}

/// Port of `OctavePerlinNoiseSampler.maintainPrecision`.
#[inline(always)]
fn maintain_precision(value: f64) -> f64 {
    value - (value / 3.3554432e7 + 0.5).floor() * 3.3554432e7
}

/// Port of `MathHelper.clampedLerp(start, end, delta)`.
#[inline(always)]
fn clamped_lerp(start: f64, end: f64, delta: f64) -> f64 {
    let d = delta.clamp(0.0, 1.0);
    start + d * (end - start)
}

/// Computes the Y clamped gradient, equivalent to Minecraft's:
/// `clampedMap(y, fromY, toY, fromValue, toValue)`
///   = `clampedLerp(fromValue, toValue, (y - fromY) / (toY - fromY))`
pub fn y_clamped_gradient(y: f64, from_y: f64, to_y: f64, from_value: f64, to_value: f64) -> f64 {
    let delta = (y - from_y) / (to_y - from_y);
    if delta <= 0.0 {
        from_value
    } else if delta >= 1.0 {
        to_value
    } else {
        from_value + delta * (to_value - from_value)
    }
}

/*
protected static double scaleTunnels(double value) {
            if (value < -0.5) {
                return 0.75;
            } else if (value < 0.0) {
                return 1.0;
            } else {
                return value < 0.5 ? 1.5 : 2.0;
            }
        }
*/
#[inline]
pub fn scale_tunnels(value: f64) -> f64 {
    if value < -0.5 {
        0.75
    } else if value < 0.0 {
        1.0
    } else if value < 0.5 {
        1.5
    } else {
        2.0
    }
}

/*
protected static double scaleCaves(double value) {
            if (value < -0.75) {
                return 0.5;
            } else if (value < -0.5) {
                return 0.75;
            } else if (value < 0.5) {
                return 1.0;
            } else {
                return value < 0.75 ? 2.0 : 3.0;
            }
        } */
#[inline]
pub fn scale_caves(value: f64) -> f64 {
    if value < -0.75 {
        0.5
    } else if value < -0.5 {
        0.75
    } else if value < 0.5 {
        1.0
    } else if value < 0.75 {
        2.0
    } else {
        3.0
    }
}

pub fn binary_search<const N: usize>(arr: [f32; N], target: f32) -> i32 {
    // actually just use linear search since the arrays are small (length 2-11)
    for i in 0..N as usize {
        if arr[i] > target {
            return i as i32;
        }
    }
    return N as i32;
}
