package tungsten.runtime;

/**
 * Static helpers called by the generated code (SPEC sections 6.3 and 6.6).
 *
 * <p>Each method reproduces the Rust function named in the comment above it bit for bit, keeping
 * Rust's operand order. No method mutates an array argument. Rust {@code usize} indices are
 * {@code int} here, and integer arithmetic wraps like Rust in release builds.
 */
public final class Helpers {
    private Helpers() {}

    // ==========================================
    // Seeding
    // ==========================================

    // Rust: tungsten-mc/src/utilsf64.rs make_permutation_table
    public static PerlinNoiseSampler make_permutation_table(long seed, long id0, long id1, long subidx, long sub0, long sub1) {
        Xoroshiro128PlusPlusRandom.XoroshiroRandomSplitter rngparent =
            Xoroshiro128PlusPlusRandom.from_seed(Xoroshiro128PlusPlusRandom.create_xoroshiro_seed(seed)).next_splitter();
        Xoroshiro128PlusPlusRandom rng1 = rngparent.split(id0, id1);
        for (long i = 0; i < subidx; i++) {
            rng1.next_splitter();
        }

        Xoroshiro128PlusPlusRandom.XoroshiroRandomSplitter rng1_split = rng1.next_splitter();

        Xoroshiro128PlusPlusRandom rng2 = rng1_split.split(sub0, sub1);

        return PerlinNoiseSampler.create(rng2);
    }

    // Rust: tungsten-mc/src/utilsf64.rs make_base3d_perm_table
    public static InterpolatedNoiseSampler make_base3d_perm_table(long seed) {
        return InterpolatedNoiseSampler.create_base3d(seed);
    }

    // ==========================================
    // Noise
    // ==========================================

    // Rust: tungsten-mc/src/utilsf64.rs perlin
    public static double perlin(Vec3 p, PerlinNoiseSampler sampler) {
        // One octave of OctavePerlinNoiseSampler.sample: p is already position * frequency.
        return sampler.sample(
            InterpolatedNoiseSampler.maintain_precision(p.x),
            InterpolatedNoiseSampler.maintain_precision(p.y),
            InterpolatedNoiseSampler.maintain_precision(p.z));
    }

    // Rust: tungsten-mc/src/utilsf64.rs base3d_noise
    public static double base3d_noise(Vec3 p, InterpolatedNoiseSampler sampler, double smear_scale_multiplier, double xz_factor, double scaled_xz_scale, double y_factor, double scaled_y_scale) {
        return sampler.sample(p, smear_scale_multiplier, xz_factor, scaled_xz_scale, y_factor, scaled_y_scale);
    }

    // ==========================================
    // Splines (f32 throughout, like Rust)
    // ==========================================

    // Rust: tungsten-mc/src/utilsf64.rs hermite
    static float hermite(float t, float p0, float p1, float m0, float m1, float h_minus_g) {
        float p = (m0 * h_minus_g) - (p1 - p0);
        float q = (-m1 * h_minus_g) + (p1 - p0);

        float lerp1 = p0 + t * (p1 - p0);
        float lerp2 = p + t * (q - p);

        return lerp1 + ((t * (1.0f - t)) * lerp2);
    }

    // Rust: tungsten-mc/src/utilsf64.rs advanced_hermite
    public static float advanced_hermite(float[] spline_locations, float[] spline_values, float[] spline_derivatives, float coordinate, int index) {
        int n = spline_locations.length;
        // Rust casts index to usize, so a negative index counts as ">= N".
        if (index == 0 || index < 0 || index >= n) {
            int i = (index < 0 || index >= n) ? n - 1 : index;
            float value = spline_values[i];
            float derivative = spline_derivatives[i];
            float location = spline_locations[i];
            return value + derivative * (coordinate - location);
        }
        int index_minus_1 = index - 1;
        float h_minus_g = spline_locations[index] - spline_locations[index_minus_1];
        float t = (coordinate - spline_locations[index_minus_1])
            / (spline_locations[index] - spline_locations[index_minus_1]);
        float value_minus_1 = spline_values[index_minus_1];
        float value = spline_values[index];
        float derivative_minus_1 = spline_derivatives[index_minus_1];
        float derivative = spline_derivatives[index];
        return hermite(t, value_minus_1, value, derivative_minus_1, derivative, h_minus_g);
    }

    // Rust: tungsten-mc/src/utilsf64.rs binary_search (a linear scan)
    public static int binary_search(float[] arr, float target) {
        for (int i = 0; i < arr.length; i++) {
            if (arr[i] > target) {
                return i;
            }
        }
        return arr.length;
    }

    // ==========================================
    // Math
    // ==========================================

    // Rust: tungsten-mc/src/utilsf64.rs abs (f64::abs clears the sign bit; so does Math.abs since JDK 18)
    public static double abs(double x) {
        return Math.abs(x);
    }

    // Rust: tungsten-mc/src/utilsf64.rs max. f64::max is LLVM maxnum, which leaves ties between
    // +0.0 and -0.0 (and which NaN payload survives) up to the backend. Measured with rustc -O on
    // x86-64: if b is NaN the result is a (even if a is NaN too), otherwise a > b ? a : b, so
    // max(0.0, -0.0) is -0.0. Math.max differs (NaN wins, +0.0 > -0.0), so it isn't used.
    public static double max(double a, double b) {
        if (b != b) {
            return a;
        }
        return a > b ? a : b;
    }

    // Rust: tungsten-mc/src/utilsf64.rs min. Same lowering as max (measured the same way).
    public static double min(double a, double b) {
        if (b != b) {
            return a;
        }
        return a < b ? a : b;
    }

    // Rust: tungsten-mc/src/utilsf64.rs clamp (f64::clamp; NaN x stays NaN). Rust panics when
    // min > max or either bound is NaN.
    public static double clamp(double x, double min, double max) {
        if (!(min <= max)) {
            throw new IllegalArgumentException("min > max, or either was NaN. min = " + min + ", max = " + max);
        }
        if (x < min) {
            x = min;
        }
        if (x > max) {
            x = max;
        }
        return x;
    }

    // Rust: tungsten-mc/src/mathf64.rs floor
    public static double floor(double value) {
        return Math.floor(value);
    }

    // Rust: tungsten-mc/src/utilsf64.rs scale_tunnels
    public static double scale_tunnels(double value) {
        if (value < -0.5) {
            return 0.75;
        } else if (value < 0.0) {
            return 1.0;
        } else if (value < 0.5) {
            return 1.5;
        } else {
            return 2.0;
        }
    }

    // Rust: tungsten-mc/src/utilsf64.rs scale_caves
    public static double scale_caves(double value) {
        if (value < -0.75) {
            return 0.5;
        } else if (value < -0.5) {
            return 0.75;
        } else if (value < 0.5) {
            return 1.0;
        } else if (value < 0.75) {
            return 2.0;
        } else {
            return 3.0;
        }
    }

    // ==========================================
    // Interpolation and grid indexing
    // ==========================================

    // Rust: tungsten-mc/src/mathf64.rs lerp (note the (a, b, t) argument order)
    private static double lerp(double a, double b, double t) {
        return a + t * (b - a);
    }

    // Rust: tungsten-mc/src/mathf64.rs interpolate
    public static double interpolate(double v000, double v100, double v010, double v110, double v001, double v101, double v011, double v111, double fx, double fy, double fz) {
        double x00 = lerp(v000, v100, fx);
        double x10 = lerp(v010, v110, fx);
        double x01 = lerp(v001, v101, fx);
        double x11 = lerp(v011, v111, fx);

        double y0 = lerp(x00, x10, fy);
        double y1 = lerp(x01, x11, fy);

        return lerp(y0, y1, fz);
    }

    // Rust: tungsten-mc/src/mathf64.rs as_index
    static int as_index(int x, int y, int z, int size_x, int size_y, int size_z) {
        return z * size_y * size_x + y * size_x + x;
    }

    // Rust: tungsten-mc/src/mathf64.rs flat_y_zero_index
    public static int flat_y_zero_index(Pos3 pos, int size_x, int size_z) {
        return pos.z * size_x + pos.x;
    }

    // Rust: tungsten-mc/src/mathf64.rs cornerx{0,4}y{0,8}z{0,4}_8, via base_grid (x >> 2, y >> 3, z >> 2)
    public static int cornerx0y0z0_8(Pos3 pos3, int sx, int sy, int sz) { return as_index((pos3.x >> 2), (pos3.y >> 3), (pos3.z >> 2), sx, sy, sz); }
    public static int cornerx4y0z0_8(Pos3 pos3, int sx, int sy, int sz) { return as_index((pos3.x >> 2) + 1, (pos3.y >> 3), (pos3.z >> 2), sx, sy, sz); }
    public static int cornerx0y8z0_8(Pos3 pos3, int sx, int sy, int sz) { return as_index((pos3.x >> 2), (pos3.y >> 3) + 1, (pos3.z >> 2), sx, sy, sz); }
    public static int cornerx4y8z0_8(Pos3 pos3, int sx, int sy, int sz) { return as_index((pos3.x >> 2) + 1, (pos3.y >> 3) + 1, (pos3.z >> 2), sx, sy, sz); }
    public static int cornerx0y0z4_8(Pos3 pos3, int sx, int sy, int sz) { return as_index((pos3.x >> 2), (pos3.y >> 3), (pos3.z >> 2) + 1, sx, sy, sz); }
    public static int cornerx4y0z4_8(Pos3 pos3, int sx, int sy, int sz) { return as_index((pos3.x >> 2) + 1, (pos3.y >> 3), (pos3.z >> 2) + 1, sx, sy, sz); }
    public static int cornerx0y8z4_8(Pos3 pos3, int sx, int sy, int sz) { return as_index((pos3.x >> 2), (pos3.y >> 3) + 1, (pos3.z >> 2) + 1, sx, sy, sz); }
    public static int cornerx4y8z4_8(Pos3 pos3, int sx, int sy, int sz) { return as_index((pos3.x >> 2) + 1, (pos3.y >> 3) + 1, (pos3.z >> 2) + 1, sx, sy, sz); }

    // Rust: tungsten-mc/src/mathf64.rs xfract4
    public static double xfract4(Pos3 pos3) {
        return (double) (pos3.x & 3) / 4.0;
    }

    // Rust: tungsten-mc/src/mathf64.rs yfract8
    public static double yfract8(Pos3 pos3) {
        return (double) (pos3.y & 7) / 8.0;
    }

    // Rust: tungsten-mc/src/mathf64.rs zfract4
    public static double zfract4(Pos3 pos3) {
        return (double) (pos3.z & 3) / 4.0;
    }

    // Rust: tungsten-mc/src/mathf64.rs preliminary_surface_index
    public static int preliminary_surface_index(Pos3 pos, int i, int cell_height, int sx, int sy, int sz) {
        int ni = as_index(pos.x, i / cell_height, pos.z, sx, sy, sz);
        // Rust compares the usize, so a negative index is out of bounds too.
        if (Integer.compareUnsigned(ni, 10000000) >= 0) {
            throw new IllegalStateException("preliminary_surface_index out of bounds: " + i
                + " (pos: " + new Pos3(pos.x, i / cell_height, pos.z) + ", sx: " + sx + ", sy: " + sy + ", sz: " + sz + ")");
        }
        return ni;
    }

    // ===============================================
    // Beardify
    // ===============================================

    // Rust: tungsten-mc/src/mathf64.rs clamped_map
    private static double clamped_map(double value, double old_start, double old_end, double new_start, double new_end) {
        double t = clamp((value - old_start) / (old_end - old_start), 0.0, 1.0);
        return new_start + t * (new_end - new_start);
    }

    // Rust: tungsten-mc/src/mathf64.rs get_magnitude_weight
    public static double get_magnitude_weight(double x, double y, double z) {
        double d = Math.sqrt(x * x + y * y + z * z);
        return clamped_map(d, 0.0, 6.0, 1.0, 0.0);
    }

    // Rust: tungsten-mc/src/mathf64.rs fast_inverse_sqrt
    private static double fast_inverse_sqrt(double x) {
        double half = 0.5 * x;
        long l = 6910469410427058090L - (Double.doubleToRawLongBits(x) >> 1);
        double y = Double.longBitsToDouble(l);
        return y * (1.5 - half * y * y);
    }

    // Rust: tungsten-mc/src/mathf64.rs get_structure_weight
    public static double get_structure_weight(int x, int y, int z, int yy) {
        int i = x + 12;
        int j = y + 12;
        int k = z + 12;
        if (index_in_bounds(i) && index_in_bounds(j) && index_in_bounds(k)) {
            double d = (double) yy + 0.5;
            double e = (double) x * (double) x + d * d + (double) z * (double) z;
            double f = -d * fast_inverse_sqrt(e / 2.0) / 2.0;
            return f * (double) StructureWeightTable.TABLE[k * 24 * 24 + i * 24 + j];
        } else {
            return 0.0;
        }
    }

    // The generated Java widens the i32 arguments to double (Rust passes them as i32). The
    // values are always whole numbers that fit an int, so narrowing back is exact.
    public static double get_structure_weight(double x, double y, double z, double yy) {
        return get_structure_weight((int) x, (int) y, (int) z, (int) yy);
    }

    private static boolean index_in_bounds(int i) {
        return i >= 0 && i < 24;
    }

    // Rust: tungsten-mc/src/mathf64.rs do_beardify
    public static boolean do_beardify(BlockBox block_box, Vec3 rpos3) {
        return rpos3.x >= block_box.min_x
            && rpos3.x <= block_box.max_x
            && rpos3.y >= block_box.min_y
            && rpos3.y <= block_box.max_y
            && rpos3.z >= block_box.min_z
            && rpos3.z <= block_box.max_z;
    }

    // Rust: tungsten-mc/src/mathf64.rs valid_box
    public static boolean valid_box(BlockBox block_box) {
        return block_box.min_x <= block_box.max_x
            && block_box.min_y <= block_box.max_y
            && block_box.min_z <= block_box.max_z
            && block_box.min_x != 0.0
            && block_box.min_y != 0.0
            && block_box.min_z != 0.0
            && block_box.max_x != 0.0
            && block_box.max_y != 0.0
            && block_box.max_z != 0.0;
    }
}
