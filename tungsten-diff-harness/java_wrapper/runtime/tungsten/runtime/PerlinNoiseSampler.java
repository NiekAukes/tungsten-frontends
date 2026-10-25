package tungsten.runtime;

/**
 * Immutable seeded Perlin sampler (SPEC section 6.3).
 * Rust: tungsten-mc/src/utils.rs PerlinNoiseSampler, sampling in tungsten-mc/src/perlin.rs.
 *
 * <p>Every floating-point expression keeps Rust's operand order, including the {@code 0.0 * x}
 * terms in {@link #grad}, which matter for signed zeros and NaN.
 */
public final class PerlinNoiseSampler {
    /** Stored as bytes; read back with {@code & 0xFF}, like Rust's {@code u8}. */
    private final byte[] permutation;
    public final double origin_x, origin_y, origin_z;

    PerlinNoiseSampler(byte[] permutation, double origin_x, double origin_y, double origin_z) {
        this.permutation = permutation;
        this.origin_x = origin_x;
        this.origin_y = origin_y;
        this.origin_z = origin_z;
    }

    /** Rust: perlin.rs create_perlin_noise_sampler */
    public static PerlinNoiseSampler create(Xoroshiro128PlusPlusRandom rng) {
        byte[] p = new byte[256];

        double origin_x = rng.next_double() * 256.0;
        double origin_y = rng.next_double() * 256.0;
        double origin_z = rng.next_double() * 256.0;

        for (int i = 0; i < 256; i++) {
            p[i] = (byte) i;
        }
        for (int i = 0; i < 256; i++) {
            int j = rng.next_int_bound(256 - i);
            byte k = p[i];
            p[i] = p[j + i];
            p[j + i] = k;
        }

        return new PerlinNoiseSampler(p, origin_x, origin_y, origin_z);
    }

    // ------------------ Noise Sampling ------------------

    /** Rust: perlin.rs sample_perlin */
    public double sample(double x, double y, double z) {
        x += origin_x;
        y += origin_y;
        z += origin_z;

        int i = floor_int(x);
        int j = floor_int(y);
        int k = floor_int(z);

        double g = x - (double) i;
        double h = y - (double) j;
        double l = z - (double) k;

        return sample_section(i, j, k, g, h, l, h);
    }

    /** Rust: perlin.rs sample_perlin_scaled */
    public double sample_scaled(double x, double y, double z, double y_scale, double y_max) {
        x += origin_x;
        y += origin_y;
        z += origin_z;

        int i = floor_int(x);
        int j = floor_int(y);
        int k = floor_int(z);

        double g = x - (double) i;
        double h = y - (double) j;
        double l = z - (double) k;

        double n = 0.0;
        if (y_scale != 0.0) {
            double m = (y_max >= 0.0 && y_max < h) ? y_max : h;
            n = Math.floor(m / y_scale + 1.0e-7) * y_scale;
        }

        return sample_section(i, j, k, g, h - n, l, h);
    }

    /** Rust: perlin.rs sample_perlin_section */
    private double sample_section(int section_x, int section_y, int section_z, double x, double y, double z, double fade_y) {
        int i = map(section_x);
        int j = map(section_x + 1);
        int k = map(i + section_y);
        int l = map(i + section_y + 1);
        int m = map(j + section_y);
        int n = map(j + section_y + 1);

        double d = grad(map(k + section_z), x, y, z);
        double e = grad(map(m + section_z), x - 1.0, y, z);
        double f = grad(map(l + section_z), x, y - 1.0, z);
        double g = grad(map(n + section_z), x - 1.0, y - 1.0, z);
        double h = grad(map(k + section_z + 1), x, y, z - 1.0);
        double o = grad(map(m + section_z + 1), x - 1.0, y, z - 1.0);
        double p = grad(map(l + section_z + 1), x, y - 1.0, z - 1.0);
        double q = grad(map(n + section_z + 1), x - 1.0, y - 1.0, z - 1.0);

        double r = fade(x);
        double s = fade(fade_y);
        double t = fade(z);

        return lerp3(r, s, t, d, e, f, g, h, o, p, q);
    }

    /** Rust: perlin.rs map */
    private int map(int input) {
        return permutation[input & 0xFF] & 0xFF;
    }

    // ------------------ Helper Functions ------------------

    /** Rust: perlin.rs floor_int. Java's (int) cast saturates and maps NaN to 0, like Rust's {@code as i32}. */
    static int floor_int(double v) {
        int i = (int) v;
        return v < (double) i ? i - 1 : i;
    }

    static double fade(double t) {
        return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
    }

    static double lerp(double delta, double start, double end) {
        return start + delta * (end - start);
    }

    /** Rust: perlin.rs GRAD3 */
    private static final double[][] GRAD3 = {
        {1.0, 1.0, 0.0},
        {-1.0, 1.0, 0.0},
        {1.0, -1.0, 0.0},
        {-1.0, -1.0, 0.0},
        {1.0, 0.0, 1.0},
        {-1.0, 0.0, 1.0},
        {1.0, 0.0, -1.0},
        {-1.0, 0.0, -1.0},
        {0.0, 1.0, 1.0},
        {0.0, -1.0, 1.0},
        {0.0, 1.0, -1.0},
        {0.0, -1.0, -1.0},
        {1.0, 1.0, 0.0},
        {0.0, -1.0, 1.0},
        {-1.0, 1.0, 0.0},
        {0.0, -1.0, -1.0},
    };

    /** Rust: perlin.rs grad + dot */
    static double grad(int hash, double x, double y, double z) {
        double[] g = GRAD3[hash & 15];
        return g[0] * x + g[1] * y + g[2] * z;
    }

    static double lerp2(double delta_x, double delta_y, double x0y0, double x1y0, double x0y1, double x1y1) {
        return lerp(delta_y, lerp(delta_x, x0y0, x1y0), lerp(delta_x, x0y1, x1y1));
    }

    static double lerp3(
        double delta_x, double delta_y, double delta_z,
        double x0y0z0, double x1y0z0, double x0y1z0, double x1y1z0,
        double x0y0z1, double x1y0z1, double x0y1z1, double x1y1z1
    ) {
        return lerp(
            delta_z,
            lerp2(delta_x, delta_y, x0y0z0, x1y0z0, x0y1z0, x1y1z0),
            lerp2(delta_x, delta_y, x0y0z1, x1y0z1, x0y1z1, x1y1z1)
        );
    }
}
