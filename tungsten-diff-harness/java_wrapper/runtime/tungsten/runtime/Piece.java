package tungsten.runtime;

/** Extern type (SPEC section 6.4). Rust: tungsten-mc/src/mathf64.rs Piece (same field order). */
public final class Piece {
    public final BlockBox block_box;
    public final int terrain_adjustment, ground_level_delta;

    public Piece(BlockBox block_box, int terrain_adjustment, int ground_level_delta) {
        this.block_box = block_box;
        this.terrain_adjustment = terrain_adjustment;
        this.ground_level_delta = ground_level_delta;
    }
}
