use tungsten_wg::{
    body, build, extern_fns,
    spmt::{
        VariableType::{Array, Extern, Vec3, F64, I32},
        *,
    },
};

use crate::{
    parse::model::Density,
    transform_spmt::{anonvar, density::DensityBuilder, newvar, prefixvar},
};
use builder::*;

impl<'a, 'm> DensityBuilder<'a, 'm> {
    pub fn beardify(&mut self, density: Density<'a>) -> Expression<'m> {
        let density_input =
            self.lower_density_input(density, Some("Beardifier".into()), Some(&lower_beardify));

        Expression::DensityVariable(density_input, None)
    }
}

enum Adjustment {
    AdjNone = 0,
    AdjBury = 1,
    AdjBeardThin = 2,
    AdjBeardBox = 3,
    AdjEncapsulate = 4,
}

impl From<Adjustment> for Expression<'_> {
    fn from(adj: Adjustment) -> Self {
        match adj {
            Adjustment::AdjNone => Expression::Int(0),
            Adjustment::AdjBury => Expression::Int(1),
            Adjustment::AdjBeardThin => Expression::Int(2),
            Adjustment::AdjBeardBox => Expression::Int(3),
            Adjustment::AdjEncapsulate => Expression::Int(4),
        }
    }
}

impl<'m> Adjustment {
    pub fn expr(self) -> Expression<'m> {
        self.into()
    }
}

/*
@Override
    public double sample(DensityFunction.NoisePos pos) {
        if (this.field_61467 == null) {
            return 0.0;
        }

        int i = pos.blockX();
        int j = pos.blockY();
        int k = pos.blockZ();
        if (!this.field_61467.contains(i, j, k)) {
            return 0.0;
        }

        double d = 0.0;

        for (StructureWeightSampler.Piece piece : this.field_61465) {
            BlockBox blockBox = piece.box();
            int l = piece.groundLevelDelta();
            int m = Math.max(0, Math.max(blockBox.getMinX() - i, i - blockBox.getMaxX()));
            int n = Math.max(0, Math.max(blockBox.getMinZ() - k, k - blockBox.getMaxZ()));
            int o = blockBox.getMinY() + l;
            int p = j - o;

            int q = switch (piece.terrainAdjustment()) {
                case NONE -> 0;
                case BURY, BEARD_THIN -> p;
                case BEARD_BOX -> Math.max(0, Math.max(o - j, j - blockBox.getMaxY()));
                case ENCAPSULATE -> Math.max(0, Math.max(blockBox.getMinY() - j, j - blockBox.getMaxY()));
            };

            d += switch (piece.terrainAdjustment()) {
                case NONE -> 0.0;
                case BURY -> getMagnitudeWeight(m, q / 2.0, n);
                case BEARD_THIN, BEARD_BOX -> getStructureWeight(m, q, n, p) * 0.8;
                case ENCAPSULATE -> getMagnitudeWeight(m / 2.0, q / 2.0, n / 2.0) * 0.8;
            };
        }

        for (JigsawJunction jigsawJunction : this.field_61466) {
            int r = i - jigsawJunction.getSourceX();
            int l = j - jigsawJunction.getSourceGroundY();
            int m = k - jigsawJunction.getSourceZ();
            d += getStructureWeight(r, l, m, l) * 0.4;
        }

        return d;
    }
*/

extern_fns!(
    get_structure_weight(x: F64, y: F64, z: F64, yy: F64): F64,
    get_magnitude_weight(x: F64, y: F64, z: F64): F64,
    // ground_level_delta(b: "BlockBox"): F64,
    do_beardify(b: "BlockBox", rpos: Vec3): Bool,
    valid_box(b: "BlockBox"): Bool
);

macro_rules! decl {
    ($builder:expr, $name:ident, $ty:expr) => {{
        let _t = prefixvar($builder.arena, stringify!($name), $ty);

        $builder.add_variable(_t);

        _t
    }};
}

fn lower_beardify<'a, 'm>(
    builder: &mut DensityBuilder<'a, 'm>,
    density: Density<'a>,
) -> Expression<'m> {
    let jigsaw_array = newvar(
        builder.arena,
        "jigsaw_array",
        Array(Extern("Jigsaw").into(), 50),
    );
    let piece_array = newvar(
        builder.arena,
        "piece_array",
        Array(Extern("Piece").into(), 50),
    );
    let base_box = newvar(builder.arena, "base_box", Extern("BlockBox"));

    builder.density_function.host_inputs = vec![
        HostInput::Static(base_box),
        HostInput::Static(jigsaw_array),
        HostInput::Static(piece_array),
    ];

    let d = prefixvar(builder.arena, "ret", VariableType::F64);
    builder.add_variable(d);
    let rpos = builder.rpos3;
    // let terrain_adjustment = |piece: Expression<'m>| piece.field("terrain_adjustment", I32);
    fn terrain_adjustment<'m>(piece: impl Into<Expression<'m>>) -> Expression<'m> {
        piece.into().field("terrain_adjustment", I32)
    }
    fn get_box<'m>(piece: impl Into<Expression<'m>>) -> Expression<'m> {
        piece.into().field("block_box", Extern("BlockBox"))
    };
    fn ground_level_delta<'m>(piece: impl Into<Expression<'m>>) -> Expression<'m> {
        piece.into().field("ground_level_delta", I32)
    };
    build!(builder, {
        d = 0.0;

        if (do_beardify(base_box, rpos)) {
            // let current_piece = {
            //     let _t = prefixvar(builder.arena, "current_piece", Extern("Jigsaw"));
            //     builder.add_variable(_t);
            //     _t
            // };
            let current_piece = decl!(builder, current_piece, Extern("Piece"));
            let current_jigsaw = decl!(builder, current_jigsaw, Extern("Jigsaw"));
            let i = decl!(builder, i, VariableType::I32);
            let m = decl!(builder, m, VariableType::I32);
            let n = decl!(builder, n, VariableType::I32);
            let o = decl!(builder, o, VariableType::I32);
            let p = decl!(builder, p, VariableType::I32);
            let q = decl!(builder, q, VariableType::I32);
            let s = decl!(builder, s, VariableType::I32);

            let x = rpos.field("x", F64).reusable();
            let y = rpos.field("y", F64).reusable();
            let z = rpos.field("z", F64).reusable();

            i = 0;
            while (i.lt(50)) {
                /*
                BlockBox blockBox = piece.box();
                int l = piece.groundLevelDelta();
                int m = Math.max(0, Math.max(blockBox.getMinX() - i, i - blockBox.getMaxX()));
                int n = Math.max(0, Math.max(blockBox.getMinZ() - k, k - blockBox.getMaxZ()));
                int o = blockBox.getMinY() + l;
                int p = j - o;
                 */
                current_piece = piece_array.index(i);

                if (!valid_box(get_box(current_piece))) {
                    break;
                }

                m = max(
                    0.0f64,
                    max(
                        base_box.field("min_x", F64) - x(),
                        x() - base_box.field("max_x", F64),
                    ),
                );
                n = max(
                    0.0f64,
                    max(
                        base_box.field("min_z", F64) - z(),
                        z() - base_box.field("max_z", F64),
                    ),
                );
                o = base_box.field("min_y", F64) + ground_level_delta(current_piece).cast(F64);
                p = y() - o.cast(F64);

                /*
                int q = switch (piece.terrainAdjustment()) {
                    case NONE -> 0;
                    case BURY, BEARD_THIN -> p;
                    case BEARD_BOX -> Math.max(0, Math.max(o - j, j - blockBox.getMaxY()));
                    case ENCAPSULATE -> Math.max(0, Math.max(blockBox.getMinY() - j, j - blockBox.getMaxY()));
                };
                 */
                s = terrain_adjustment(current_piece);
                q = 0;
                // not needed, is a no-op
                // if (s.eq_expr(Adjustment::AdjNone.expr())) {
                //     q = 0;
                // }
                if (s
                    .eq_expr(Adjustment::AdjBury.expr())
                    .or(p.eq_expr(Adjustment::AdjBeardThin.expr())))
                {
                    q = p;
                }
                if (s.eq_expr(Adjustment::AdjBeardBox.expr())) {
                    q = max(
                        0.0f64,
                        max(o.cast(F64) - y(), y() - base_box.field("max_y", F64)),
                    );
                }
                if (s.eq_expr(Adjustment::AdjEncapsulate.expr())) {
                    q = max(
                        0.0f64,
                        max(
                            base_box.field("min_y", F64) - y(),
                            y() - base_box.field("max_y", F64),
                        ),
                    );
                }

                /*
                d += switch (piece.terrainAdjustment()) {
                    case NONE -> 0.0;
                    case BURY -> getMagnitudeWeight(m, q / 2.0, n);
                    case BEARD_THIN, BEARD_BOX -> getStructureWeight(m, q, n, p) * 0.8;
                    case ENCAPSULATE -> getMagnitudeWeight(m / 2.0, q / 2.0, n / 2.0) * 0.8;
                };
                 */
                // no-op
                // if (s.eq_expr(Adjustment::AdjNone.expr())) {
                //     d += 0.0;
                // }
                if (s.eq_expr(Adjustment::AdjBury.expr())) {
                    d += get_magnitude_weight(m.cast(F64), q.cast(F64) / 2.0, n.cast(F64));
                }
                if (s
                    .eq_expr(Adjustment::AdjBeardThin.expr())
                    .or(s.eq_expr(Adjustment::AdjBeardBox.expr())))
                {
                    d += get_structure_weight(m, q, n, p) * 0.8;
                }
                if (s.eq_expr(Adjustment::AdjEncapsulate.expr())) {
                    d += get_magnitude_weight(
                        m.cast(F64) / 2.0,
                        q.cast(F64) / 2.0,
                        n.cast(F64) / 2.0,
                    ) * 0.8;
                }

                i += 1;
            }

            /*
            for (JigsawJunction jigsawJunction : this.field_61466) {
                int r = i - jigsawJunction.getSourceX();
                int l = j - jigsawJunction.getSourceGroundY();
                int m = k - jigsawJunction.getSourceZ();
                d += getStructureWeight(r, l, m, l) * 0.4;
            }
             */
            i = 0;
            let r = decl!(builder, r, I32);
            let l = decl!(builder, l, I32);
            let m = decl!(builder, m, I32);
            while (i.lt(50i32)) {
                current_jigsaw = jigsaw_array.index(i);
                r = x().cast(I32) - current_jigsaw.field("source_x", I32);
                l = y().cast(I32) - current_jigsaw.field("source_ground_y", I32);
                m = z().cast(I32) - current_jigsaw.field("source_z", I32);
                d += get_structure_weight(r, l, m, l) * 0.4f64;

                i += 1;
            }
        }
    });
    d.into()
}

fn lower_ground_level_delta<'a, 'm>(builder: &mut DensityBuilder<'a, 'm>) -> FunctionRef<'m> {
    todo!()
}

fn lower_terrain_adjustment<'a, 'm>(builder: &mut DensityBuilder<'a, 'm>) -> FunctionRef<'m> {
    todo!()
}

fn lower_get_structure_weight<'a, 'm>(builder: &mut DensityBuilder<'a, 'm>) -> FunctionRef<'m> {
    todo!()
}
