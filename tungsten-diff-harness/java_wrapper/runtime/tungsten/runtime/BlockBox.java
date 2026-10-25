package tungsten.runtime;

/** Extern type (SPEC section 6.4). Rust: tungsten-mc/src/mathf64.rs BlockBox (same field order). */
public final class BlockBox {
    public final double min_x, min_y, min_z, max_x, max_y, max_z;

    public BlockBox(double min_x, double min_y, double min_z, double max_x, double max_y, double max_z) {
        this.min_x = min_x;
        this.min_y = min_y;
        this.min_z = min_z;
        this.max_x = max_x;
        this.max_y = max_y;
        this.max_z = max_z;
    }
}
