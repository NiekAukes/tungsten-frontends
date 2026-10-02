use crate::{
    parse::model::Density,
    transform_spmt::{anonvar, density::DensityBuilder},
};
use tungsten_wg::{
    build, extern_fns,
    spmt::{builder::macros::*, builder::SPMTBuilder, Expression, Statement, VariableType},
};

extern_fns! {
    preliminary_surface_index(pos: Pos3, y: I32, cell_height: I32, sx: I32, sy: I32, sz: I32): I32,
}

impl<'a, 'm> DensityBuilder<'a, 'm> {
    pub fn find_top_surface(
        &mut self,
        density: Density<'a>,
        cell_height: i32,
        lower_bound: i32,
        upper_bound: Density<'a>,
    ) -> Expression<'m> {
        /*
        public double sample(DensityFunction.NoisePos pos) {
            int i = MathHelper.floor(this.upperBound.sample(pos) / this.cellHeight) * this.cellHeight;
            if (i <= this.lowerBound) {
                return this.lowerBound;
            }

            for (int j = i; j >= this.lowerBound; j -= this.cellHeight) {
                if (this.density.sample(new DensityFunction.UnblendedNoisePos(pos.blockX(), j, pos.blockZ())) > 0.0) {
                    return j;
                }
            }

            return this.lowerBound;
        }

         */
        // first evaluate the density upper bound
        // dimensions = (x, total height / cell height, z)
        let workset = self.builder_state.as_ref().unwrap().working_set;
        let y_dim = self.noise_settings.height / cell_height;
        assert!(self.noise_settings.height % cell_height == 0);
        let new_dimensions = (workset.dimensions.0, y_dim, workset.dimensions.2);
        let new_scaled_position = (
            workset.scaled_position.0,
            cell_height as f64,
            workset.scaled_position.2,
        );
        let density_di: tungsten_wg::spmt::DensityInput<'_> = self.lower_density_input_expanded(
            density,
            None,
            new_dimensions,
            workset.scaled_origin,
            new_scaled_position,
        );

        // let upper_bound = Expression::DensityVariable(
        //     upper_bound_di,
        //     Some(preliminary_surface_index(self.p.clone(), cell_height).into()),
        // );
        let upper_bound = self.lower_density(upper_bound);
        let i = anonvar(self.arena, VariableType::I32);
        let j = anonvar(self.arena, VariableType::I32);
        let r = anonvar(self.arena, VariableType::F64);
        self.add_variable(i);
        self.add_variable(j);
        self.add_variable(r);
        build!(self, {
            // begin of SPMT
            i = floor(upper_bound / (cell_height as f64)) * (cell_height as f64);
            if (i.le(lower_bound)) {
                i = lower_bound;
            } else {
                j = i / cell_height;
                while (j.ge(lower_bound / cell_height)) {
                    // EMBED
                    let sample = Expression::DensityVariable(
                        density_di,
                        Some(
                            preliminary_surface_index(
                                self.p.clone(),
                                j,
                                cell_height,
                                workset.dimensions.0,
                                y_dim,
                                workset.dimensions.2,
                            )
                            .into(),
                        ),
                    );
                    //end of EMBED

                    if (sample.gt(0.0)) {
                        i = j * cell_height;
                        break;
                    }
                    j = j - 1;
                }
            }
            r = i;
            // end of SPMT
        });
        r.into()
    }
}
