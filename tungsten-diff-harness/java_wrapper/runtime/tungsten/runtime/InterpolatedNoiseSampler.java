package tungsten.runtime;

/**
 * Immutable seeded interpolated ("base3d") sampler (SPEC section 6.3).
 * Rust: tungsten-mc/src/utilsf64.rs InterpolatedNoiseSampler and base3d_noise.
 */
public final class InterpolatedNoiseSampler {
    private static final double BASE_3D_XZ_SCALE = 684.412;

    final PerlinNoiseSampler[] lower;         // rangeClosed(-15, 0)
    final PerlinNoiseSampler[] upper;         // rangeClosed(-15, 0)
    final PerlinNoiseSampler[] interpolation; // rangeClosed(-7, 0)

    /** Rust: InterpolatedNoiseSampler::new */
    InterpolatedNoiseSampler(Xoroshiro128PlusPlusRandom rng) {
        this.lower = create_legacy(rng, 16);
        this.upper = create_legacy(rng, 16);
        this.interpolation = create_legacy(rng, 8);
    }

    /** Rust: InterpolatedNoiseSampler::create_base3d */
    public static InterpolatedNoiseSampler create_base3d(long seed) {
        Xoroshiro128PlusPlusRandom rng = Xoroshiro128PlusPlusRandom.from_seed(Xoroshiro128PlusPlusRandom.create_xoroshiro_seed(seed));
        Xoroshiro128PlusPlusRandom.XoroshiroRandomSplitter random_splitter = rng.next_splitter();

        Xoroshiro128PlusPlusRandom.XoroshiroSeed xoroshiro_seed = Xoroshiro128PlusPlusRandom.create_xoroshiro_seed_str("minecraft:terrain");
        return new InterpolatedNoiseSampler(random_splitter.split(xoroshiro_seed.seed_lo(), xoroshiro_seed.seed_hi()));
    }

    /**
     * Rust: create_legacy. Equivalent to {@code OctavePerlinNoiseSampler.createLegacy(random,
     * IntStream.rangeClosed(-(num - 1), 0))}: slot {@code num - 1} is created first, then down to 0.
     */
    private static PerlinNoiseSampler[] create_legacy(Xoroshiro128PlusPlusRandom rng, int num) {
        PerlinNoiseSampler[] samplers = new PerlinNoiseSampler[num];
        for (int k = num - 1; k >= 0; k--) {
            samplers[k] = PerlinNoiseSampler.create(rng);
        }
        return samplers;
    }

    /** Rust: base3d_noise */
    public double sample(
        Vec3 p,
        double smear_scale_multiplier,
        double xz_factor,
        double scaled_xz_scale,
        double y_factor,
        double scaled_y_scale
    ) {
        double x = p.x;
        double y = p.y;
        double z = p.z;
        double d = x * scaled_xz_scale * BASE_3D_XZ_SCALE;
        double e = y * scaled_y_scale * BASE_3D_XZ_SCALE;
        double f = z * scaled_xz_scale * BASE_3D_XZ_SCALE;

        double g = d / xz_factor;
        double h = e / y_factor;
        double iz = f / xz_factor;
        double j = scaled_y_scale * BASE_3D_XZ_SCALE * smear_scale_multiplier;
        double k = j / y_factor;

        double l = 0.0;
        double m = 0.0;
        double n = 0.0;
        double o = 1.0;
        double inv_o = 1.0;

        for (int pi = 7; pi >= 0; pi--) {
            n += interpolation[pi].sample_scaled(
                maintain_precision(g * o),
                maintain_precision(h * o),
                maintain_precision(iz * o),
                k * o,
                h * o
            ) * inv_o;
            o *= 0.5;
            inv_o *= 2.0;
        }

        double q = (n * 0.1 + 1.0) * 0.5;
        boolean bl2 = q >= 1.0;
        boolean bl3 = q <= 0.0;
        o = 1.0;

        for (int r = 15; r >= 0; r--) {
            double s = maintain_precision(d * o);
            double t = maintain_precision(e * o);
            double u = maintain_precision(f * o);
            double v = j * o;

            if (!bl2) {
                l += lower[r].sample_scaled(s, t, u, v, e * o) / o;
            }
            if (!bl3) {
                m += upper[r].sample_scaled(s, t, u, v, e * o) / o;
            }

            o *= 0.5;
        }

        return clamped_lerp(l * 0.001953125, m * 0.001953125, q) * 0.0078125;
    }

    /** Rust: maintain_precision (OctavePerlinNoiseSampler.maintainPrecision) */
    private static double maintain_precision(double value) {
        return value - Math.floor(value / 3.3554432e7 + 0.5) * 3.3554432e7;
    }

    /** Rust: clamped_lerp (MathHelper.clampedLerp) */
    private static double clamped_lerp(double start, double end, double delta) {
        double d = Helpers.clamp(delta, 0.0, 1.0);
        return start + d * (end - start);
    }
}
