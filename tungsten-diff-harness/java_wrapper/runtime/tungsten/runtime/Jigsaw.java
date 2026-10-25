package tungsten.runtime;

/** Extern type (SPEC section 6.4). Rust: tungsten-mc/src/mathf64.rs Jigsaw (same field order). */
public final class Jigsaw {
    public final int source_x, source_ground_y, source_z, delta_y;

    public Jigsaw(int source_x, int source_ground_y, int source_z, int delta_y) {
        this.source_x = source_x;
        this.source_ground_y = source_ground_y;
        this.source_z = source_z;
        this.delta_y = delta_y;
    }
}
