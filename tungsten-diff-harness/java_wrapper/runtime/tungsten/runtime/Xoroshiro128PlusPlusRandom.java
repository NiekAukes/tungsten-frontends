package tungsten.runtime;

import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;

/**
 * Xoroshiro128++ PRNG and the seed helpers from RandomSeed.java.
 * Rust: tungsten-mc/src/xoroshiro.rs. Every Rust {@code u64} shift is {@code >>>} here.
 *
 * <p>The Gaussian sampler is not ported: no helper uses it, and Rust's {@code f64::ln} isn't
 * guaranteed to match any Java implementation bit for bit.
 */
public final class Xoroshiro128PlusPlusRandom {
    public static final long GOLDEN_RATIO_64 = -7046029254386353131L;
    public static final long SILVER_RATIO_64 = 7640891576956012809L;

    public long seed_lo;
    public long seed_hi;

    /** Rust: Xoroshiro128PlusPlusRandom::new (zero guard included). */
    public Xoroshiro128PlusPlusRandom(long seed_lo, long seed_hi) {
        if ((seed_lo | seed_hi) == 0) {
            seed_lo = GOLDEN_RATIO_64;
            seed_hi = SILVER_RATIO_64;
        }
        this.seed_lo = seed_lo;
        this.seed_hi = seed_hi;
    }

    public static Xoroshiro128PlusPlusRandom from_seed(XoroshiroSeed seed) {
        return new Xoroshiro128PlusPlusRandom(seed.seed_lo, seed.seed_hi);
    }

    // ── Seed mixing / constructors (RandomSeed.java) ─────────────────────────

    /** Rust: mix_stafford13 */
    public static long mix_stafford13(long seed) {
        seed = (seed ^ (seed >>> 30)) * -4658895280553007687L;
        seed = (seed ^ (seed >>> 27)) * -7723592293110705685L;
        return seed ^ (seed >>> 31);
    }

    /** Rust: create_xoroshiro_seed */
    public static XoroshiroSeed create_xoroshiro_seed(long seed) {
        return create_unmixed_xoroshiro_seed(seed).mix();
    }

    /** Rust: create_unmixed_xoroshiro_seed */
    public static XoroshiroSeed create_unmixed_xoroshiro_seed(long seed) {
        long lo = seed ^ SILVER_RATIO_64;
        long hi = lo + GOLDEN_RATIO_64;
        return new XoroshiroSeed(lo, hi);
    }

    /** Rust: create_xoroshiro_seed_str (MD5 of the UTF-8 bytes, both halves big-endian). */
    public static XoroshiroSeed create_xoroshiro_seed_str(String seed) {
        byte[] hash;
        try {
            hash = MessageDigest.getInstance("MD5").digest(seed.getBytes(StandardCharsets.UTF_8));
        } catch (NoSuchAlgorithmException e) {
            throw new IllegalStateException(e);
        }
        ByteBuffer buf = ByteBuffer.wrap(hash); // big-endian by default
        long lo = buf.getLong();
        long hi = buf.getLong();
        return new XoroshiroSeed(lo, hi);
    }

    // ── Generator ────────────────────────────────────────────────────────────

    /** Rust: next_raw */
    public long next_raw() {
        long lo = this.seed_lo;
        long hi = this.seed_hi;
        long result = Long.rotateLeft(lo + hi, 17) + lo;
        long m = hi ^ lo;
        this.seed_lo = Long.rotateLeft(lo, 49) ^ m ^ (m << 21);
        this.seed_hi = Long.rotateLeft(m, 28);
        return result;
    }

    /** Rust: next(bits) */
    public int next(int bits) {
        return (int) (next_raw() >>> (64 - bits));
    }

    public long next_long() {
        return next_raw();
    }

    /** Rust: next_double */
    public double next_double() {
        long raw = next_raw() >>> 11;
        return (double) raw / (double) (1L << 53);
    }

    /** Rust: next_int (no bound) */
    public int next_int() {
        return (int) next_raw();
    }

    /**
     * Rust: next_int_bound. Ported as written, including the retry loop sign-extending
     * {@code next_int()} where the first draw zero-extends it.
     */
    public int next_int_bound(int bound) {
        long l = Integer.toUnsignedLong(next_int());
        long m = l * (long) bound;
        long n = m & 0xFFFFFFFFL;
        if (n < (long) bound) {
            int threshold = Integer.remainderUnsigned(~bound + 1, bound);
            while (n < Integer.toUnsignedLong(threshold)) {
                l = (long) next_int();
                m = l * (long) bound;
                n = m & 0xFFFFFFFFL;
            }
        }
        return (int) (m >> 32);
    }

    public void skip(long n) {
        for (long i = 0; i < n; i++) {
            next_raw();
        }
    }

    /** Rust: next_splitter */
    public XoroshiroRandomSplitter next_splitter() {
        long lo = next_long();
        long hi = next_long();
        return new XoroshiroRandomSplitter(new XoroshiroSeed(lo, hi));
    }

    // ── Seed record and splitter ─────────────────────────────────────────────

    /** Rust: XoroshiroSeed */
    public record XoroshiroSeed(long seed_lo, long seed_hi) {
        /** Rust: split_with */
        public XoroshiroSeed split_with(long lo, long hi) {
            return new XoroshiroSeed(lo ^ seed_lo, hi ^ seed_hi);
        }

        /** Rust: mix */
        public XoroshiroSeed mix() {
            return new XoroshiroSeed(mix_stafford13(seed_lo), mix_stafford13(seed_hi));
        }
    }

    /** Rust: XoroshiroRandomSplitter */
    public record XoroshiroRandomSplitter(XoroshiroSeed seed) {
        public Xoroshiro128PlusPlusRandom split(long seed_low, long seed_high) {
            return from_seed(seed.split_with(seed_low, seed_high));
        }

        /** Rust: split_space. The hash is computed in i32 and widened afterwards, as in Rust. */
        public Xoroshiro128PlusPlusRandom split_space(int x, int y, int z) {
            long l = (long) (x * 3129871 ^ z * 116129781 ^ y);
            l = l * l * 42317861L + l * 11L;
            l >>= 16;
            long m = l ^ seed.seed_lo();
            return from_seed(seed.split_with(m, seed.seed_hi()));
        }
    }
}
