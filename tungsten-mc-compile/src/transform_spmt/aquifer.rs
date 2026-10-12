use std::cell;

use tungsten_wg::extern_fns;
use tungsten_wg::spmt::builder::floor;
use tungsten_wg::spmt::builder::min;
use tungsten_wg::spmt::DensityFunction;
use tungsten_wg::spmt::DensityFunctionRef;
use tungsten_wg::spmt::DensityInput;
use tungsten_wg::spmt::Function;
use tungsten_wg::spmt::FunctionRef;
use tungsten_wg::spmt::HostInput;
use tungsten_wg::spmt::Interned;
use tungsten_wg::spmt::VariableType;
use tungsten_wg::spmt::VariableType::Vec3;
use tungsten_wg::spmt::VariableType::F64;
use tungsten_wg::spmt::VariableType::I32;
use tungsten_wg::spmt::VariableType::I64;
use tungsten_wg::{body, build, spmt::Expression};

use crate::parse::model::DensityType;
use crate::parse::model::DensityType::Aquifer;
use crate::transform_spmt::newvar;
use crate::transform_spmt::spline::old;
use crate::{parse::model::Density, transform_spmt::density::DensityBuilder};
use tungsten_wg::spmt::builder::SPMTBuilder;
use tungsten_wg::spmt::Statement;

impl<'a, 'm> DensityBuilder<'a, 'm> {
    pub fn aquifer(&mut self, density: Density<'a>) -> Expression<'m> {
        // The aquifer sampler is split up into 2 parts: the general aquifer and the special aquifer.
        // The general aquifer handles common cases, where the aquifer is not influenced by special conditions.
        // The special aquifer handles special cases, such as aquifers touching, or other unique configurations.
        // This is necessary for the GPU-model to make sense. The special case is extremely heavy, but also sparse.
        // we should not compute expensive special case noises unless necessary.

        // The general case will indicate whether the special aquifer needs to be computed.

        let DensityType::Aquifer {
            final_density,
            depth,
            erosion,
            floodedness,
            spread,
            fluid_type,
        } = *density
        else {
            panic!("Expected Aquifer density");
        };
        self.density_function.return_type = I32;

        let density_input = self.lower_density_input(
            density,
            Some("GeneralAquifer".into()),
            Some(&|builder, density| generate_general_aquifer_sampler(builder, density)),
        );

        Expression::DensityVariable(density_input, None)
    }
}

struct AquiferSamplerResult {
    needs_fluid_tick: bool,
    block_state: BlockState,
}

enum BlockState {
    Solid,
    Special,
    Air,
    Water,
    Lava,
}

impl From<BlockState> for i32 {
    fn from(state: BlockState) -> Self {
        match state {
            BlockState::Solid => 0,
            BlockState::Special => 1,
            BlockState::Air => 2,
            BlockState::Water => 3,
            BlockState::Lava => 4,
        }
    }
}

impl BlockState {
    fn needs_fluid_tick(self, needs_fluid_tick: bool) -> AquiferSamplerResult {
        AquiferSamplerResult {
            needs_fluid_tick,
            block_state: self,
        }
    }
}

impl<'m> From<AquiferSamplerResult> for Expression<'m> {
    fn from(result: AquiferSamplerResult) -> Self {
        (i32::from(result.block_state) << 1 | (result.needs_fluid_tick as i32)).into()
    }
}

extern_fns!(
    floor_div(a: I32, b: I32): I32,
    split_random(splitter: "XoroshiroRandomSplitter", x: I32, y: I32, z: I32): "Xoroshiro128PlusPlusRandom",
    random_next_int(random: "Xoroshiro128PlusPlusRandom", bound: I32): I32,
    packed_pos(x: I32, y: I32, z: I32): I64,
    cell_to_block_x(x: I32, random_offset: I32): I32,
    cell_to_block_y(y: I32, random_offset: I32): I32,
    cell_to_block_z(z: I32, random_offset: I32): I32,
    aquifer_index(start_x: I32, start_y: I32, start_z: I32, end_x: I32, end_z: I32, sy: I32, x: I32, y: I32, z: I32): I32,

    unpack_long_x(x: I64): I32,
    unpack_long_y(x: I64): I32,
    unpack_long_z(x: I64): I32,

    esh(esh: "EstimatedSurfaceHeight", x: I32, z: I32): I32,
    depth(d_cache: "Depth", x: I32, y: I32, z: I32): I32,
    erosion(e_cache: "Erosion", x: I32, y: I32, z: I32): I32,

    clamp(value: F64, min: F64, max: F64): F64,
    abs(value: F64): F64,
    min_i32(a: I32, b: I32): I32,
    max_distance(a: F64, b: F64): F64,
    get_block_state_from_packed(closest_fluid_level: I64, y_level: I32): I32,
    block_state(state: I32, fluid_level: I32, y_level: I32): I32,
);

pub fn generate_general_aquifer_sampler<'a, 'm>(
    builder: &mut DensityBuilder<'a, 'm>,
    density: Density<'a>,
) -> Expression<'m> {
    let Aquifer {
        final_density,
        depth,
        erosion,
        floodedness,
        spread,
        fluid_type,
    } = density
    else {
        panic!("Expected Aquifer density type");
    };

    // lower final_density first
    // let final_density =
    //     builder.lower_density_input(final_density, Some("FinalDensity".into()), None);
    // let final_density_expr = Expression::DensityVariable(final_density, None);
    let final_density_expr: Expression = 0.0.into();
    let mut variables = vec![];

    /*
    // Step 0: solid terrain stays solid. Nothing to do.
    if (density > 0.0) {
        this.needsFluidTick = false;
        return null;
    }
    */
    build!(builder, {
        if (final_density_expr.gt(0.0)) {
            return Expression::from(BlockState::Solid.needs_fluid_tick(false));
        }
    });
    /*

    int blockX = pos.blockX();
    int blockY = pos.blockY();
    int blockZ = pos.blockZ();
    AquiferSampler.FluidLevel defaultFluidLevel = this.fluidLevelSampler.getFluidLevel(blockX, blockY, blockZ);
    */
    let x = builder.rpos3.field("x", F64).reusable();
    let y = builder.rpos3.field("y", F64).reusable();
    let z = builder.rpos3.field("z", F64).reusable();

    let (fluid_level, fluid_block) = generate_fluid_level_sampler(builder);
    builder.density_function.helper_functions.push(fluid_level);
    builder.density_function.helper_functions.push(fluid_block);

    let block_x = newvar(builder.arena, "blockX", I32);
    let block_y = newvar(builder.arena, "blockY", I32);
    let block_z = newvar(builder.arena, "blockZ", I32);
    let fluid_block_var = newvar(builder.arena, "default_fluid_block_var", I32);
    let fluid_level_var = newvar(builder.arena, "default_fluid_level_var", I32);
    variables.extend(vec![
        block_x,
        block_y,
        block_z,
        fluid_block_var,
        fluid_level_var,
    ]);
    build!(builder, {
        block_x = x().cast(I32);
        block_y = y().cast(I32);
        block_z = z().cast(I32);
    });

    let fluid_level_expr = |y| Expression::FunctionCall {
        function: fluid_level,
        parameters: vec![y],
    };
    let fluid_block_expr = |y| Expression::FunctionCall {
        function: fluid_block,
        parameters: vec![y],
    };

    let estimate_hightest_surface_level = newvar(builder.arena, "aquifer_max_y", I32);

    builder
        .density_function
        .host_inputs
        .push(HostInput::Static(estimate_hightest_surface_level));

    /*
    // Step 1: way above any terrain, aquifers are irrelevant. Use the default fluid (sea level) or air.
    if (blockY > this.aquiferMaxY) {
        this.needsFluidTick = false;
        return defaultFluidLevel.getBlockState(blockY);
    }
    */
    build!(builder, {
        fluid_block_var = fluid_block_expr(block_y.into());
        fluid_level_var = fluid_level_expr(block_y.into());
        if (block_y.gt(estimate_hightest_surface_level)) {
            return (block_state(fluid_block_var, fluid_level_var, block_y).shl(1));
        }
    });
    /*

    // Step 2: the global lava level (the lava lakes at the bottom of the world) overrides everything.
    if (defaultFluidLevel.getBlockState(blockY).isOf(Blocks.LAVA)) {
        this.needsFluidTick = false;
        return SharedConstants.DISABLE_FLUID_GENERATION ? Blocks.AIR.getDefaultState() : Blocks.LAVA.getDefaultState();
    }

    */
    build!(builder, {
        if (fluid_block_var.eq_expr(Expression::from(i32::from(BlockState::Lava)))) {
            return Expression::from(BlockState::Lava.needs_fluid_tick(false));
        }
    });

    /*
    // Step 3: find the 4 closest aquifer sites to this block.
    // Convert the block position to a cell coordinate (with the grid's shifts). The 12 candidate
    // cells are the 2 (x) * 3 (y) * 2 (z) cells starting at this base cell.
    int baseCellX = blockToCellX(blockX + -5);
    int baseCellY = blockToCellY(blockY + 1);
    int baseCellZ = blockToCellZ(blockZ + -5);
    */
    let base_cell_x = (block_x + -5).shr(4);
    let base_cell_y = floor_div(block_y + 1, 12);
    let base_cell_z = (block_z + -5).shr(4);

    let min_y = builder.noise_generator_settings.noise.min_y;
    let max_y = builder.noise_generator_settings.noise.height + min_y;
    let start_y = floordiv(min_y, 12) - 1;
    let end_y = floordiv(max_y, 12) + 1;
    let y_dim = end_y - start_y + 1;

    // let p = newvar(builder.arena, "p", VariableType::Pos3);
    // let idx = newvar(builder.arena, "idx", VariableType::I32);
    let origin = newvar(builder.arena, "origin", VariableType::Vec3);
    let start_x = newvar(builder.arena, "start_x", VariableType::I32);
    let start_z = newvar(builder.arena, "start_z", VariableType::I32);
    let end_x = newvar(builder.arena, "end_x", VariableType::I32);
    let end_z = newvar(builder.arena, "end_z", VariableType::I32);

    build!(builder, {
        start_x = (origin.field("x", F64) - 5.0).cast(I32).shr(4);
        start_z = (origin.field("z", F64) - 5.0).cast(I32).shr(4);
        end_x = (origin.field("x", F64) + 15.0 - 5.0).cast(I32).shr(4) + 1;
        end_z = (origin.field("z", F64) + 15.0 - 5.0).cast(I32).shr(4) + 1;
    });

    variables.extend(vec![start_x, start_z, end_x, end_z]);

    /*
    // Squared distances to the four closest sites (1 = closest) and the cell indices they belong to.
    int closestDistanceSq1 = Integer.MAX_VALUE;
    int closestDistanceSq2 = Integer.MAX_VALUE;
    int closestDistanceSq3 = Integer.MAX_VALUE;
    int closestDistanceSq4 = Integer.MAX_VALUE;
    int closestIndex1 = 0;
    int closestIndex2 = 0;
    int closestIndex3 = 0;
    int closestIndex4 = 0;
    */
    let closest_distance_sq1 = newvar(builder.arena, "closest_distance_sq1", I32);
    let closest_distance_sq2 = newvar(builder.arena, "closest_distance_sq2", I32);
    let closest_distance_sq3 = newvar(builder.arena, "closest_distance_sq3", I32);
    let closest_distance_sq4 = newvar(builder.arena, "closest_distance_sq4", I32);
    let closest_index1 = newvar(builder.arena, "closest_index1", I32);
    let closest_index2 = newvar(builder.arena, "closest_index2", I32);
    let closest_index3 = newvar(builder.arena, "closest_index3", I32);
    let closest_index4 = newvar(builder.arena, "closest_index4", I32);
    let neighbor_index = newvar(builder.arena, "neighbor_index", I32);
    // let cell_site = newvar(builder.arena, "cell_site", VariableType::I64);

    let (closest_sites, dimensions) = generate_closest_sites(builder);
    let csdi_var = newvar(builder.arena, "closest_sites", VariableType::DensityInput);
    let cell_site = newvar(builder.arena, "cell_site", VariableType::I64);

    let csdi = DensityInput {
        var: csdi_var,
        density_function: closest_sites,
        scaled_origin: (1.0, 1.0, 1.0),
        scaled_position: (1.0, 1.0, 1.0),
        dimensions,
    };
    variables.extend(vec![
        closest_distance_sq1,
        closest_distance_sq2,
        closest_distance_sq3,
        closest_distance_sq4,
        closest_index1,
        closest_index2,
        closest_index3,
        closest_index4,
        neighbor_index,
        cell_site,
    ]);

    builder.density_function.add_density_input(csdi.clone());
    /*
    for (int cellOffsetX = 0; cellOffsetX <= 1; cellOffsetX++) {
        for (int cellOffsetY = -1; cellOffsetY <= 1; cellOffsetY++) {
            for (int cellOffsetZ = 0; cellOffsetZ <= 1; cellOffsetZ++) {

                int neighborCellX = baseCellX + cellOffsetX;
                int neighborCellY = baseCellY + cellOffsetY;
                int neighborCellZ = baseCellZ + cellOffsetZ;
                int neighborIndex = this.index(neighborCellX, neighborCellY, neighborCellZ);
                **EXTRACTED STEP**

                int deltaX = BlockPos.unpackLongX(sitePosition) - blockX;
                int deltaY = BlockPos.unpackLongY(sitePosition) - blockY;
                int deltaZ = BlockPos.unpackLongZ(sitePosition) - blockZ;
                int distanceSq = deltaX * deltaX + deltaY * deltaY + deltaZ * deltaZ;
                // Insert into the sorted "top 4 closest" list, shifting the farther entries down.
                if (closestDistanceSq1 >= distanceSq) {
                    closestIndex4 = closestIndex3;
                    closestIndex3 = closestIndex2;
                    closestIndex2 = closestIndex1;
                    closestIndex1 = neighborIndex;
                    closestDistanceSq4 = closestDistanceSq3;
                    closestDistanceSq3 = closestDistanceSq2;
                    closestDistanceSq2 = closestDistanceSq1;
                    closestDistanceSq1 = distanceSq;
                } else if (closestDistanceSq2 >= distanceSq) {
                    closestIndex4 = closestIndex3;
                    closestIndex3 = closestIndex2;
                    closestIndex2 = neighborIndex;
                    closestDistanceSq4 = closestDistanceSq3;
                    closestDistanceSq3 = closestDistanceSq2;
                    closestDistanceSq2 = distanceSq;
                } else if (closestDistanceSq3 >= distanceSq) {
                    closestIndex4 = closestIndex3;
                    closestIndex3 = neighborIndex;
                    closestDistanceSq4 = closestDistanceSq3;
                    closestDistanceSq3 = distanceSq;
                } else if (closestDistanceSq4 >= distanceSq) {
                    closestIndex4 = neighborIndex;
                    closestDistanceSq4 = distanceSq;
                }
            }
        }
    } */

    let cell_offset_x = newvar(builder.arena, "cell_offset_x", I32);
    let cell_offset_y = newvar(builder.arena, "cell_offset_y", I32);
    let cell_offset_z = newvar(builder.arena, "cell_offset_z", I32);

    let delta_x = newvar(builder.arena, "delta_x", I32);
    let delta_y = newvar(builder.arena, "delta_y", I32);
    let delta_z = newvar(builder.arena, "delta_z", I32);
    let distance_sq = newvar(builder.arena, "distance_sq", I32);

    variables.extend(vec![
        cell_offset_x,
        cell_offset_y,
        cell_offset_z,
        delta_x,
        delta_y,
        delta_z,
        distance_sq,
    ]);

    build!(builder, {
        closest_distance_sq1 = i32::MAX;
        closest_distance_sq2 = i32::MAX;
        closest_distance_sq3 = i32::MAX;
        closest_distance_sq4 = i32::MAX;
        closest_index1 = 0;
        closest_index2 = 0;
        closest_index3 = 0;
        closest_index4 = 0;

        cell_offset_x = 0;
        cell_offset_y = -1;
        cell_offset_z = 0;
        while (cell_offset_x.lt(2)) {
            while (cell_offset_y.lt(2)) {
                while (cell_offset_z.lt(2)) {
                    neighbor_index = aquifer_index(
                        start_x,
                        start_y,
                        start_z,
                        end_x,
                        end_z,
                        y_dim,
                        base_cell_x + cell_offset_x,
                        base_cell_y + cell_offset_y,
                        base_cell_z + cell_offset_z,
                    );
                    cell_site = Expression::DensityVariable(
                        csdi.clone(),
                        Some(Box::new(neighbor_index.into())),
                    );
                    delta_x = unpack_long_x(cell_site) - block_x;
                    delta_y = unpack_long_y(cell_site) - block_y;
                    delta_z = unpack_long_z(cell_site) - block_z;
                    distance_sq = delta_x * delta_x + delta_y * delta_y + delta_z * delta_z;
                    if (distance_sq.le(closest_distance_sq1)) {
                        closest_distance_sq4 = closest_distance_sq3;
                        closest_distance_sq3 = closest_distance_sq2;
                        closest_distance_sq2 = closest_distance_sq1;
                        closest_distance_sq1 = distance_sq;
                        closest_index4 = closest_index3;
                        closest_index3 = closest_index2;
                        closest_index2 = closest_index1;
                        closest_index1 = neighbor_index;
                    } else {
                        if (distance_sq.le(closest_distance_sq2)) {
                            closest_distance_sq4 = closest_distance_sq3;
                            closest_distance_sq3 = closest_distance_sq2;
                            closest_distance_sq2 = distance_sq;
                            closest_index4 = closest_index3;
                            closest_index3 = closest_index2;
                            closest_index2 = neighbor_index;
                        } else {
                            if (distance_sq.le(closest_distance_sq3)) {
                                closest_distance_sq4 = closest_distance_sq3;
                                closest_distance_sq3 = distance_sq;
                                closest_index4 = closest_index3;
                                closest_index3 = neighbor_index;
                            } else {
                                if (distance_sq.le(closest_distance_sq4)) {
                                    closest_distance_sq4 = distance_sq;
                                    closest_index4 = neighbor_index;
                                }
                            }
                        }
                    }

                    cell_offset_z = cell_offset_z + 1;
                }
                cell_offset_y = cell_offset_y + 1;
                cell_offset_z = 0; // Reset the z-offset for the next row in the y-direction
            }
            cell_offset_x = cell_offset_x + 1;
            cell_offset_y = -1; // Reset the y-offset for the next column in the x-direction
        }
    });

    /*

    // Step 4: the block "belongs" to the nearest site, so it uses that site's fluid.
    AquiferSampler.FluidLevel closestFluidLevel = this.getCellFluidLevel(closestIndex1);
    */
    let closest_fluid_level = newvar(builder.arena, "closest_fluid_level", VariableType::I64);
    let fluid_levels = newvar(builder.arena, "fluid_levels", VariableType::DensityInput);
    let (cell_fluid_level, cell_fluid_dimensions) = generate_water_levels(
        builder,
        csdi.clone(),
        floodedness,
        spread,
        fluid_type,
        fluid_level,
        fluid_block,
    );

    let fluid_level_input = DensityInput {
        var: fluid_levels,
        density_function: cell_fluid_level,
        scaled_origin: (1.0, 1.0, 1.0),
        scaled_position: (1.0, 1.0, 1.0),
        dimensions: cell_fluid_dimensions,
    };
    builder
        .density_function
        .add_density_input(fluid_level_input.clone());
    variables.push(closest_fluid_level);

    build!(builder, {
        closest_fluid_level = Expression::DensityVariable(
            fluid_level_input.clone(),
            Some(Box::new(closest_index1.into())),
        );
    });
    /*
    // How close the 1st and 2nd nearest sites are to each other (as seen from this block):
    // 1.0 = exactly on the border between them, <= 0 = comfortably inside the nearest site's region.
    double closestPairFactor = maxDistance(closestDistanceSq1, closestDistanceSq2);
    BlockState closestFluidState = closestFluidLevel.getBlockState(blockY);
    BlockState resultState = SharedConstants.DISABLE_FLUID_GENERATION ? Blocks.AIR.getDefaultState() : closestFluidState;
    */
    let closest_pair_factor = newvar(builder.arena, "closest_pair_factor", VariableType::F64);
    let result_state = newvar(builder.arena, "result_state", VariableType::I32);
    build!(builder, {
        closest_pair_factor = max_distance(closest_distance_sq1, closest_distance_sq2);
        result_state = get_block_state_from_packed(closest_fluid_level, block_y);
    });
    variables.push(closest_pair_factor);
    variables.push(result_state);

    /*
    if (closestPairFactor <= 0.0) {
        // Well inside one site's region: no wall needed, just return its fluid.
        // Only flag a fluid tick if we're still reasonably near a border with a different fluid level.
        if (closestPairFactor >= NEEDS_FLUID_TICK_DISTANCE_THRESHOLD) {
            AquiferSampler.FluidLevel secondClosestFluidLevelForTick = this.getCellFluidLevel(closestIndex2);
            this.needsFluidTick = !closestFluidLevel.equals(secondClosestFluidLevelForTick);
        } else {
            this.needsFluidTick = false;
        }

        return resultState;
    } */
    build!(builder, {
        if (closest_pair_factor.le(0.0)) {
            // Well inside one site's region: no wall needed, just return its fluid.
            // Only flag a fluid tick if we're still reasonably near a border with a different fluid level.
            let second_closest_fluid_level_for_tick = Expression::DensityVariable(
                fluid_level_input,
                Some(Box::new(closest_index2.into())),
            )
            .cast(I64);
            let needs_fluid_tick = closest_pair_factor
                .ge(-0.76)
                .and(!closest_fluid_level.eq_expr(second_closest_fluid_level_for_tick));
            // AquiferSampler.FluidLevel secondClosestFluidLevelForTick = this.getCellFluidLevel(closestIndex2);
            // this.needsFluidTick = !closestFluidLevel.equals(secondClosestFluidLevelForTick);
            return (result_state.shl(1).bitor(needs_fluid_tick.cast(I32)));
        }
    });

    //delegate to special aquifer sampler for handling the border region.
    // todo!()

    for v in variables.iter() {
        builder.add_variable(*v);
    }

    builder.density_function.return_type = I32;

    // result_state.into()
    BlockState::Special.needs_fluid_tick(false).into()
}

fn generate_closest_sites<'a, 'm>(
    builder: &mut DensityBuilder<'a, 'm>,
) -> (DensityFunctionRef<'m>, (i32, i32, i32)) {
    let min_y = builder.noise_generator_settings.noise.min_y;
    let max_y = builder.noise_generator_settings.noise.height + min_y;
    let start_y = floordiv(min_y, 12) - 1;
    let end_y = floordiv(max_y, 12) + 1;
    let y_dim = end_y - start_y + 1;

    /*


        // Get (or lazily generate and cache) this cell's random site position.
        long cachedSitePosition = this.cellSitePositions[neighborIndex];
        long sitePosition;
        if (cachedSitePosition != Long.MAX_VALUE) {
            sitePosition = cachedSitePosition;
        } else {
            // Seeded by the cell coordinates, so it is identical no matter which chunk asks first.
            Random random = this.randomDeriver.split(neighborCellX, neighborCellY, neighborCellZ);
            sitePosition = BlockPos.asLong(
                    cellToBlockX(neighborCellX, random.nextInt(10)),
                    cellToBlockY(neighborCellY, random.nextInt(9)),
                    cellToBlockZ(neighborCellZ, random.nextInt(10))
            );
            this.cellSitePositions[neighborIndex] = sitePosition;
        }


    }
     */

    let random_deriver = newvar(
        builder.arena,
        "splitter",
        VariableType::Extern("XoroshiroRandomSplitter"),
    );

    // host constant
    let random = newvar(
        builder.arena,
        "rnd",
        VariableType::Extern("Xoroshiro128PlusPlusRandom"),
    );

    // let p = newvar(builder.arena, "p", VariableType::Pos3);
    // let idx = newvar(builder.arena, "idx", VariableType::I32);

    // let start_x = newvar(builder.arena, "start_x", VariableType::I32);
    // let start_z = newvar(builder.arena, "start_z", VariableType::I32);

    /*
    private int index(int x, int y, int z) {
        int i = x - this.startX;
        int j = y - this.startY;
        int k = z - this.startZ;
        return (j * this.sizeZ + k) * this.sizeX + i;
    }

    private (i32, i32, i32) reverse_index(int index, origin) {
        int i = pos3.x
        int k = pos3.y
        int j = pos3.z
        int startX =(origin.x - 5).shiftRight(4);
        int startY = static
        int startZ =(origin.z - 5).shiftRight(4);
        return (i + startX, j + startY, k + startZ);
    }
     */

    let p = newvar(builder.arena, "pos3", VariableType::Pos3);
    let origin = newvar(builder.arena, "origin", VariableType::Pos3);

    let x = newvar(builder.arena, "x", VariableType::I32);
    let y = newvar(builder.arena, "y", VariableType::I32);
    let z = newvar(builder.arena, "z", VariableType::I32);

    let r = newvar(builder.arena, "r", VariableType::I64);

    let rnd_x = newvar(builder.arena, "rnd_x", I32);
    let rnd_y = newvar(builder.arena, "rnd_y", I32);
    let rnd_z = newvar(builder.arena, "rnd_z", I32);

    // let rpos3 = newvar(builder.arena, "rpos3", VariableType::Vec3);
    // let origin = newvar(builder.arena, "origin", VariableType::Vec3);
    // let origin_scale = newvar(builder.arena, "origin_scale", VariableType::Vec3);
    // let position_scale = newvar(builder.arena, "position_scale", VariableType::Vec3);

    let body = body!(builder, {
        x = (origin.field("x", F64).cast(I32) - 5).shr(4) + p.field("x", I32);
        y = Expression::from(start_y) + p.field("y", I32);
        z = (origin.field("z", F64).cast(I32) - 5).shr(4) + p.field("z", I32);
        random = split_random(random_deriver, x, y, z);
        rnd_x = random_next_int(random.as_mut_ref(), 10);
        rnd_y = random_next_int(random.as_mut_ref(), 9);
        rnd_z = random_next_int(random.as_mut_ref(), 10);
        r = packed_pos(
            cell_to_block_x(x, rnd_x),
            cell_to_block_y(y, rnd_y),
            cell_to_block_z(z, rnd_z),
        );
        return r;
    });

    let df = DensityFunction {
        canonical_name: Some("aquifer_random".into()),
        density_inputs: vec![],
        permutation_table_inputs: vec![],
        host_inputs: vec![HostInput::Static(random_deriver)],
        variables: vec![random, r, p, x, y, z, rnd_x, rnd_y, rnd_z],
        helper_functions: vec![],
        constants: vec![],
        source_hash: 0,
        return_type: VariableType::I64,
        body: body,
    };

    let dfr = Interned::new(builder.arena.alloc(df));

    let mut bs = builder.builder_state.take().unwrap();
    let c_density = DensityType::Custom("closest_sites");
    bs.insert_density_cache(Box::leak(Box::new(c_density)), dfr);
    // dimensions:
    // x: (chunk_size >> 4) + 2
    // y: y_dim
    // z: (chunk_size >> 4) + 2
    let r = (
        dfr,
        (
            (bs.sampling_set.dimensions.0 >> 4) + 2,
            y_dim,
            (bs.sampling_set.dimensions.2 >> 4) + 2,
        ),
    );
    builder.builder_state = Some(bs);
    r
}

fn generate_water_levels<'a, 'm>(
    master_builder: &mut DensityBuilder<'a, 'm>,
    closest_sites: DensityInput<'m>,
    fluid_level_floodedness: Density<'a>,
    fluid_level_spread: Density<'a>,
    fluid_type_noise: Density<'a>,
    fluid_level_func: FunctionRef<'m>,
    fluid_block_func: FunctionRef<'m>,
    // fluid_level: impl Fn(Expression<'m>) -> Expression<'m>,
    // fluid_type: impl Fn(Expression<'m>) -> Expression<'m>,
) -> (DensityFunctionRef<'m>, (i32, i32, i32)) {
    let bs = master_builder.builder_state.take().unwrap();
    let mut builder = DensityBuilder::new(
        master_builder.arena,
        bs,
        master_builder.noise_generator_settings,
    );

    let min_y = builder.noise_generator_settings.noise.min_y;
    let max_y = builder.noise_generator_settings.noise.height + min_y;
    let start_y = floordiv(min_y, 12) - 1;
    let end_y = floordiv(max_y, 12) + 1;
    let y_dim = end_y - start_y + 1;

    // let p = newvar(builder.arena, "p", VariableType::Pos3);
    // let idx = newvar(builder.arena, "idx", VariableType::I32);

    // let start_x = newvar(builder.arena, "start_x", VariableType::I32);
    // let start_z = newvar(builder.arena, "start_z", VariableType::I32);

    let mut bs = builder.builder_state.take().unwrap();
    let dimensions = (
        (bs.sampling_set.dimensions.0 >> 4) + 2,
        y_dim,
        (bs.sampling_set.dimensions.2 >> 4) + 2,
    );

    let x_dim_esh = bs.sampling_set.dimensions.0 >> 2;

    builder.builder_state = Some(bs);
    /*
    private int index(int x, int y, int z) {
        int i = x - this.startX;
        int j = y - this.startY;
        int k = z - this.startZ;
        return (j * this.sizeZ + k) * this.sizeX + i;
    }

    private (i32, i32, i32) reverse_index(int index, origin) {
        int i = pos3.x
        int k = pos3.y
        int j = pos3.z
        int startX =(origin.x - 5).shiftRight(4);
        int startY = static
        int startZ =(origin.z - 5).shiftRight(4);
        return (i + startX, j + startY, k + startZ);
    }
     */

    let p = newvar(builder.arena, "pos3", VariableType::Pos3);
    let origin = newvar(builder.arena, "origin", VariableType::Vec3);

    let x = newvar(builder.arena, "x", VariableType::I32);
    let y = newvar(builder.arena, "y", VariableType::I32);
    let z = newvar(builder.arena, "z", VariableType::I32);

    let r = newvar(builder.arena, "r", VariableType::I64);

    let mut variables = vec![x, y, z, r];

    let default_fluid_level = |y| Expression::FunctionCall {
        function: fluid_level_func,
        parameters: vec![y],
    };
    let default_fluid_block = |y| Expression::FunctionCall {
        function: fluid_block_func,
        parameters: vec![y],
    };

    let cs_var = newvar(builder.arena, "cell_site", VariableType::I64);
    variables.push(cs_var);

    let esh_cache = newvar(
        builder.arena,
        "estimate_surface_height",
        VariableType::Extern("EstimatedSurfaceHeight"),
    );
    let erosion_cache = newvar(
        builder.arena,
        "erosion_cache",
        VariableType::Extern("Erosion"),
    );

    let depth_cache = newvar(builder.arena, "depth_cache", VariableType::Extern("Depth"));

    let fluid_level_var = newvar(builder.arena, "fluid_level_var", VariableType::I64);
    let fluid_type_var = newvar(builder.arena, "fluid_type_var", VariableType::I64);

    let lowest_surface_y = newvar(builder.arena, "lowest_surface_y", VariableType::I32);
    let upper_y = newvar(builder.arena, "upper_y", VariableType::I32);
    let lower_y = newvar(builder.arena, "lower_y", VariableType::I32);
    let origin_column_has_surface_fluid = newvar(
        builder.arena,
        "origin_column_has_surface_fluid",
        VariableType::Bool,
    );

    let c_surface_sample_chunk_offsets = newvar(
        builder.arena,
        "c_surface_sample_chunk_offsets",
        VariableType::Array(Box::new(I32), 26),
    );
    let c_value_vec = vec![
        //{0, 0}, {-2, -1}, {-1, -1}, {0, -1}, {1, -1}, {-3, 0}, {-2, 0}, {-1, 0}, {1, 0}, {-2, 1}, {-1, 1}, {0, 1}, {1, 1}
        0, 0, -2, -1, -1, -1, 0, -1, 1, -1, -3, 0, -2, 0, -1, 0, 1, 0, -2, 1, -1, 1, 0, 1, 1, 1,
    ];
    let c_value_len = c_value_vec.len() as i32;
    // builder.add_constant(
    //     c_surface_sample_chunk_offsets,
    //     const_literal_array(c_value_vec),
    // );

    let i = newvar(builder.arena, "i", VariableType::I32);
    let sample_x = newvar(builder.arena, "sample_x", VariableType::I32);
    let sample_z = newvar(builder.arena, "sample_z", VariableType::I32);
    let surface_y = newvar(builder.arena, "surface_y", VariableType::I32);
    let aquifer_surface_y = newvar(builder.arena, "aquifer_surface_y", VariableType::I32);
    let surface_fluid_level = newvar(builder.arena, "surface_fluid_level", VariableType::I32);
    let surface_fluid_block = newvar(builder.arena, "surface_fluid_block", VariableType::I64);
    let is_origin_column = newvar(builder.arena, "is_origin_column", VariableType::Bool);
    let is_near_surface = newvar(builder.arena, "is_near_surface", VariableType::Bool);

    variables.extend(vec![
        esh_cache,
        erosion_cache,
        depth_cache,
        fluid_level_var,
        fluid_type_var,
        lowest_surface_y,
        upper_y,
        lower_y,
        origin_column_has_surface_fluid,
        i,
        sample_x,
        sample_z,
        surface_y,
        aquifer_surface_y,
        surface_fluid_level,
        surface_fluid_block,
        is_origin_column,
        is_near_surface,
    ]);

    let mut body = body!(builder, {
        cs_var = Expression::DensityVariable(closest_sites.clone(), None);

        x = unpack_long_x(cs_var);
        y = unpack_long_y(cs_var);
        z = unpack_long_z(cs_var);

        //AquiferSampler.FluidLevel defaultFluidLevel = this.fluidLevelSampler.getFluidLevel(blockX, blockY, blockZ);
        fluid_level_var = default_fluid_level(y.into());
        fluid_type_var = default_fluid_block(y.into());

        /*int lowestSurfaceY = Integer.MAX_VALUE;
        int upperY = blockY + 12;
        int lowerY = blockY - 12;
        boolean originColumnHasSurfaceFluid = false;
        */
        lowest_surface_y = i32::MAX;
        upper_y = y + 12;
        lower_y = y - 12;
        origin_column_has_surface_fluid = false;
        /*
        // Look at the terrain surface at several nearby columns (offsets measured in chunks).
        for (int[] chunkOffset : SURFACE_SAMPLE_CHUNK_OFFSETS) {
        */
        i = 0;
        while (i.lt(c_value_len / 2)) {
            /*
            int sampleX = blockX + ChunkSectionPos.getBlockCoord(chunkOffset[0]);
            int sampleZ = blockZ + ChunkSectionPos.getBlockCoord(chunkOffset[1]);
            */
            sample_x = x + c_surface_sample_chunk_offsets.index(i * 2).shl(4);
            sample_z = z + c_surface_sample_chunk_offsets.index(i * 2 + 1).shl(4);

            /*
            int surfaceY = this.chunkNoiseSampler.estimateSurfaceHeight(sampleX, sampleZ);
            int aquiferSurfaceY = this.addSurfaceMargin(surfaceY);
            boolean isOriginColumn = chunkOffset[0] == 0 && chunkOffset[1] == 0;

            // The site is entirely above the ground at its own column: it's open air, use the default level.
            if (isOriginColumn && lowerY > aquiferSurfaceY) {
                return defaultFluidLevel;
            }
            */
            surface_y = esh(esh_cache, sample_x, sample_z);
            aquifer_surface_y = surface_y + 8;
            is_origin_column = sample_x.eq_expr(x).and(sample_z.eq_expr(z));

            if (is_origin_column.and(lower_y.gt(aquifer_surface_y))) {
                return encode_fluid_level(fluid_level_var, fluid_type_var);
            }

            /*
            // Is the site within reach of this column's surface?
            boolean isNearSurface = upperY > aquiferSurfaceY;
            if (isNearSurface || isOriginColumn) {
                AquiferSampler.FluidLevel surfaceFluidLevel = this.fluidLevelSampler.getFluidLevel(sampleX, aquiferSurfaceY, sampleZ);
                if (!surfaceFluidLevel.getBlockState(aquiferSurfaceY).isAir()) {
                    // There is fluid (e.g. ocean water) at the surface of this column.
                    if (isOriginColumn) {
                        originColumnHasSurfaceFluid = true;
                    }

                    if (isNearSurface) {
                        // Near a flooded surface: copy that surface's fluid level.
                        return surfaceFluidLevel;
                    }
                }
            } */
            is_near_surface = upper_y.gt(aquifer_surface_y);
            if (is_near_surface.or(is_origin_column)) {
                // aquifer_fluid_block = block_state(
                //     default_fluid_block(aquifer_surface_y.into()),
                //     default_fluid_level(aquifer_surface_y.into()),
                //     aquifer_surface_y,
                // );
                surface_fluid_block = default_fluid_block(aquifer_surface_y.into());
                surface_fluid_level = default_fluid_level(aquifer_surface_y.into());
                // let condition =
                //     !block_state(surface_fluid_block, surface_fluid_level, aquifer_surface_y).eq_expr(i32::from(BlockState::Air));

                // a contraction of the above expression
                let condition = surface_fluid_level.le(aquifer_surface_y);

                if (condition) {
                    if (is_near_surface) {
                        return encode_fluid_level(surface_fluid_level, surface_fluid_block);
                    }
                    if (is_origin_column) {
                        origin_column_has_surface_fluid = true;
                    }
                }
            }

            /*
            lowestSurfaceY = Math.min(lowestSurfaceY, surfaceY);
            */
            lowest_surface_y = min_i32(lowest_surface_y, surface_y);
            i = i + 1;
        }
    });

    // lower floodedness
    let floodedness_func = Function {
        canonical_name: Some("fluid_level_floodedness".into()),
        parameters: vec![builder.rpos3],
        body: vec![],
        variables: vec![],
        return_type: F64,
    };

    builder.switch_function(floodedness_func);
    builder.always_inline_non_2d = true;
    let e = builder.lower_density(fluid_level_floodedness);
    let floodedness_func = builder.finish_and_continue(e, None);

    let spread_func = Function {
        canonical_name: Some("fluid_level_spread".into()),
        parameters: vec![builder.rpos3],
        body: vec![],
        variables: vec![],
        return_type: F64,
    };

    builder.switch_function(spread_func);
    let e = builder.lower_density(fluid_level_spread);
    let spread_func = builder.finish_and_continue(e, None);

    let fluid_type_func = Function {
        canonical_name: Some("fluid_type".into()),
        parameters: vec![builder.rpos3],
        body: vec![],
        variables: vec![],
        return_type: F64,
    };

    builder.switch_function(fluid_type_func);
    let e = builder.lower_density(fluid_type_noise);
    let fluid_type_func = builder.finish_and_continue(e, None);

    builder.always_inline_non_2d = false;
    /*
    // Deep enough underground: decide the level (and fluid type) using noise.
    DensityFunction.UnblendedNoisePos noisePos = new DensityFunction.UnblendedNoisePos(blockX, blockY, blockZ);
    double noiseBasedFluidScore; // > 0 -> use a noise-based cave fluid level
    double defaultFluidScore; // > 0 -> use the default fluid level
    */
    let noise_based_fluid_score =
        newvar(builder.arena, "noise_based_fluid_score", VariableType::F64);
    let default_fluid_score = newvar(builder.arena, "default_fluid_score", VariableType::F64);
    let fluid_y = newvar(builder.arena, "fluid_y", VariableType::I64);
    let surface_proximity = newvar(builder.arena, "surface_proximity", VariableType::F64);
    let floodedness_sample = newvar(builder.arena, "floodedness_sample", VariableType::F64);

    variables.extend(vec![
        noise_based_fluid_score,
        default_fluid_score,
        fluid_y,
        surface_proximity,
        floodedness_sample,
    ]);

    body.extend(body!(builder, {
        /*
        if (VanillaBiomeParameters.inDeepDarkParameters(this.erosionDensityFunction, this.depthDensityFunction, noisePos)) {
            // Deep Dark: both scores are negative, so aquifers here are always empty.
            noiseBasedFluidScore = -1.0;
            defaultFluidScore = -1.0;
        } else {
         */
        surface_proximity = 0.0;
        let e = erosion(erosion_cache, x, y, z);
        let d = depth(depth_cache, x, y, z);

        if (e.lt(-0.225).and(d.gt(0.9))) {
            // deep dark
            noise_based_fluid_score = -1.0;
            default_fluid_score = -1.0;
        } else {
            /*
            int depthBelowSurface = surfaceHeightEstimate + 8 - blockY;
            int maxDepth = 64;
            // 1.0 right at the surface, fading to 0.0 at 64 blocks deep (and 0 if the column has no surface fluid).
            double surfaceProximity = originColumnHasSurfaceFluid ? MathHelper.clampedMap(depthBelowSurface, 0.0, 64.0, 1.0, 0.0) : 0.0;
            */
            if (origin_column_has_surface_fluid) {
                surface_proximity =
                    clamped_map((lowest_surface_y + 8 - y).cast(F64), 0.0, 64.0, 1.0, 0.0);
            }
            /*
            double floodednessSample = MathHelper.clamp(this.fluidLevelFloodednessNoise.sample(noisePos), -1.0, 1.0);
            */
            let floodedness_expr = Expression::FunctionCall {
                function: floodedness_func,
                parameters: vec![Expression::Construct {
                    t: Vec3,
                    args: vec![x.cast(F64), y.cast(F64), z.cast(F64)],
                }],
            };
            floodedness_sample = clamp(floodedness_expr, -1.0, 1.0);
            // Thresholds the noise must beat; they are easier to beat (lower) the closer we are to a flooded surface.
            /*
            double defaultFluidThreshold = MathHelper.map(surfaceProximity, 1.0, 0.0, -0.3, 0.8);
            double noiseFluidThreshold = MathHelper.map(surfaceProximity, 1.0, 0.0, -0.8, 0.4);
            noiseBasedFluidScore = floodednessSample - noiseFluidThreshold;
            defaultFluidScore = floodednessSample - defaultFluidThreshold;
             */
            let default_fluid_threshold = map(surface_proximity, 1.0, 0.0, -0.3, 0.8);
            let noise_fluid_threshold = map(surface_proximity, 1.0, 0.0, -0.8, 0.4);
            noise_based_fluid_score = floodedness_sample - noise_fluid_threshold;
            default_fluid_score = floodedness_sample - default_fluid_threshold;
        }

        /*
        int fluidY;
        if (defaultFluidScore > 0.0) {
            fluidY = defaultFluidLevel.y();
        } else if (noiseBasedFluidScore > 0.0) {
            fluidY = this.getNoiseBasedFluidLevel(blockX, blockY, blockZ, surfaceHeightEstimate);
        } else {
            // Sentinel meaning "far below the world": nothing is below this level, so no fluid at all.
            fluidY = DimensionType.field_35479;
        }
        */
        if (default_fluid_score.gt(0.0)) {
            fluid_y = default_fluid_level(y.into());
        } else {
            if (noise_based_fluid_score.gt(0.0)) {
                fluid_y = get_noise_based_fluid_level(
                    &mut builder,
                    spread_func,
                    x,
                    y,
                    z,
                    lowest_surface_y,
                );
            } else {
                // Sentinel meaning "far below the world": nothing is below this level, so no fluid at all.
                fluid_y = min_y << 4;
            }
        }

        /*
        this.getFluidBlockState(blockX, blockY, blockZ, defaultFluidLevel, fluidY)
        private BlockState getFluidBlockState(int blockX, int blockY, int blockZ, AquiferSampler.FluidLevel defaultFluidLevel, int fluidLevel) {
            BlockState blockState = defaultFluidLevel.state;
            if (fluidLevel <= -10 && fluidLevel != DimensionType.field_35479 && defaultFluidLevel.state != Blocks.LAVA.getDefaultState()) {
                int i = 64;
                int j = 40;
                int k = Math.floorDiv(blockX, 64);
                int l = Math.floorDiv(blockY, 40);
                int m = Math.floorDiv(blockZ, 64);
                double d = this.fluidTypeNoise.sample(new DensityFunction.UnblendedNoisePos(k, l, m));
                if (Math.abs(d) > 0.3) {
                    blockState = Blocks.LAVA.getDefaultState();
                }
            }

            return blockState;
        }
         */
        // reuse fluid_type_var
        if (fluid_y
            .le(-10i64)
            .and(!fluid_y.eq_expr((min_y as i64) << 4i64))
            .and(!fluid_type_var.eq_expr(i32::from(BlockState::Lava) as i64)))
        {
            let k = floor_div(x, 64).cast(F64);
            let l = floor_div(y, 40).cast(F64);
            let m = floor_div(z, 64).cast(F64);
            let pos = Expression::Construct {
                t: VariableType::Vec3,
                args: vec![k.cast(F64), l.cast(F64), m.cast(F64)],
            };
            let d = Expression::FunctionCall {
                function: fluid_type_func,
                parameters: vec![pos],
            };
            if (abs(d).gt(0.3)) {
                fluid_type_var = i32::from(BlockState::Lava);
            }
        }
        /*
        return new AquiferSampler.FluidLevel(fluidY, **EXTRACTED**);
        */
        return encode_fluid_level(fluid_y, fluid_type_var);
    }));

    // we need the built density function to get permutation table inputs
    let (a, _, mut bs) = builder.finish(0.0.into());

    let df = DensityFunction {
        canonical_name: Some("water_levels".into()),
        density_inputs: vec![closest_sites],
        permutation_table_inputs: a.permutation_table_inputs,
        host_inputs: vec![
            HostInput::Static(esh_cache),
            HostInput::Static(erosion_cache),
            HostInput::Static(depth_cache),
        ],
        variables: variables,
        helper_functions: vec![
            floodedness_func,
            spread_func,
            fluid_type_func,
            fluid_level_func,
            fluid_block_func,
        ],
        constants: vec![(
            c_surface_sample_chunk_offsets,
            const_literal_array(c_value_vec),
        )],
        source_hash: 0,
        return_type: VariableType::I64,
        body: body,
    };

    let dfr = Interned::new(master_builder.arena.alloc(df));

    let c_density = DensityType::Custom("water_table");
    bs.insert_density_cache(Box::leak(Box::new(c_density)), dfr);
    // dimensions:
    // x: (chunk_size >> 4) + 2
    // y: y_dim
    // z: (chunk_size >> 4) + 2
    let r = (dfr, dimensions);
    master_builder.builder_state = Some(bs);
    r
}
fn generate_fluid_level_sampler<'a, 'm>(
    builder: &mut DensityBuilder<'a, 'm>,
) -> (FunctionRef<'m>, FunctionRef<'m>) {
    /*
        private static AquiferSampler.FluidLevelSampler createFluidLevelSampler(ChunkGeneratorSettings settings) {
        AquiferSampler.FluidLevel fluidLevel = new AquiferSampler.FluidLevel(-54, Blocks.LAVA.getDefaultState());
        int i = settings.seaLevel();
        AquiferSampler.FluidLevel fluidLevel2 = new AquiferSampler.FluidLevel(i, settings.defaultFluid());
        AquiferSampler.FluidLevel fluidLevel3 = new AquiferSampler.FluidLevel(DimensionType.MIN_HEIGHT * 2, Blocks.AIR.getDefaultState());
        return (x, y, z) -> {
            if (SharedConstants.DISABLE_FLUID_GENERATION) {
                return fluidLevel3;
            } else {
                return y < Math.min(-54, i) ? fluidLevel : fluidLevel2;
            }
        };
    }
     */
    let sea_level = builder.noise_generator_settings.sea_level;
    let level = sea_level.min(-54);
    let y = newvar(builder.arena, "y", VariableType::I32);

    let stmts = body!(builder, {
        if (y.lt(Expression::from(level))) {
            return Expression::from(-54i32);
        } else {
            return Expression::from(sea_level);
        }
    });

    let function = Interned::new(builder.arena.alloc(Function {
        canonical_name: Some("default_fluid_level".into()),
        body: stmts,
        parameters: vec![y.into()],
        variables: vec![],
        return_type: VariableType::I32,
    }));

    builder.density_function.helper_functions.push(function);

    let stmts = body!(builder, {
        if (y.lt(Expression::from(level))) {
            return Expression::from(i32::from(BlockState::Lava));
        } else {
            return Expression::from(i32::from(BlockState::Water));
        }
    });
    let function2 = Interned::new(builder.arena.alloc(Function {
        canonical_name: Some("default_fluid_block".into()),
        body: stmts,
        parameters: vec![y.into()],
        variables: vec![],
        return_type: VariableType::I32,
    }));

    builder.density_function.helper_functions.push(function2);
    (function, function2)
}

fn get_noise_based_fluid_level<'a, 'm>(
    builder: &mut DensityBuilder<'a, 'm>,
    fluid_level_spread: FunctionRef<'m>,
    block_x: impl Into<Expression<'m>>,
    block_y: impl Into<Expression<'m>>,
    block_z: impl Into<Expression<'m>>,

    surface_height_estimate: impl Into<Expression<'m>>,
) -> Expression<'m> {
    /*
    int cellSizeXZ = 16;
    int cellSizeY = 40;
    int cellX = Math.floorDiv(blockX, 16);
    int cellY = Math.floorDiv(blockY, 40);
    int cellZ = Math.floorDiv(blockZ, 16);
    */
    let cell_x = floor_div(block_x, 16);
    let cell_y = floor_div(block_y, 40);
    let cell_z = floor_div(block_z, 16);
    /*
    int cellCenterY = cellY * 40 + 20;
    int spreadAmplitude = 10;
    */
    let cell_center_y = cell_y.clone() * 40 + 20;
    /*
    double spreadOffset = this.fluidLevelSpreadNoise.sample(new DensityFunction.UnblendedNoisePos(cellX, cellY, cellZ)) * 10.0;
    int roundedSpreadOffset = MathHelper.roundDownToMultiple(spreadOffset, 3);
    int noiseFluidY = cellCenterY + roundedSpreadOffset;
    */
    let spread_offset = Expression::FunctionCall {
        function: fluid_level_spread,
        parameters: vec![Expression::Construct {
            t: Vec3,
            args: vec![cell_x.cast(F64), cell_y.cast(F64), cell_z.cast(F64)],
        }],
    } * 10.0;

    let rounded_spread_offset = floor(spread_offset / 3.0) * 3.0;
    let noise_fluid_y = cell_center_y + rounded_spread_offset.cast(I32);
    /*
    return Math.min(surfaceHeightEstimate, noiseFluidY);
     */
    min_i32(surface_height_estimate.into(), noise_fluid_y.cast(I32))
}

fn floordiv(a: i32, b: i32) -> i32 {
    if a >= 0 {
        a / b
    } else {
        (a - b + 1) / b
    }
}

fn const_literal_array<'m>(values: Vec<impl Into<Expression<'m>>>) -> Expression<'m> {
    Expression::ArrayLiteral(values.into_iter().map(|v| v.into()).collect())
}

fn encode_fluid_level<'m>(
    fluid_level: impl Into<Expression<'m>>,
    fluid_type: impl Into<Expression<'m>>,
) -> Expression<'m> {
    (fluid_level.into().cast(I64).shl(32i64)).bitor(fluid_type.into())
}

/*
// 2. clamp(normal_y, 0, 1)
let normal_y = Expression::BinaryOp {
    op: BinaryOperator::Divide,
    left: Box::new(Expression::BinaryOp {
        op: BinaryOperator::Subtract,
        left: Box::new(y_expr),
        right: Box::new(Expression::Double(from_y)),
    }),
    right: Box::new(Expression::Double(to_y - from_y)),
};

// 2. clamp(p.y, from_y, to_y)
let clamped_expr = Expression::ExternCall {
    function_name: "clamp".to_string(),
    parameters: vec![normal_y, Expression::Double(0.0), Expression::Double(1.0)],
    parameter_types: vec![VariableType::F64, VariableType::F64, VariableType::F64],
    return_type: VariableType::F64,
};

*/

fn clamped_map<'m>(
    value: impl Into<Expression<'m>>,
    min: f64,
    max: f64,
    out_min: f64,
    out_max: f64,
) -> Expression<'m> {
    let value = value.into();
    // let clamped = Expression::ExternCall {
    //     function_name: "clamp".to_string(),
    //     parameters: vec![value, Expression::Double(min), Expression::Double(max)],
    //     parameter_types: vec![VariableType::F64, VariableType::F64, VariableType::F64],
    //     return_type: VariableType::F64,
    // };
    let clamped = clamp(value, min, max);
    // Expression::BinaryOp {
    //     op: BinaryOperator::Multiply,
    //     left: Box::new(Expression::BinaryOp {
    //         op: BinaryOperator::Subtract,
    //         left: Box::new(clamped),
    //         right: Box::new(Expression::Double(min)),
    //     }),
    //     right: Box::new(Expression::Double((out_max - out_min) / (max - min))),
    // }
    (clamped - min) * ((out_max - out_min) / (max - min)) + out_min
}

fn map<'m>(
    value: impl Into<Expression<'m>>,
    old_start: f64,
    old_end: f64,
    new_start: f64,
    new_end: f64,
) -> Expression<'m> {
    let value = value.into();
    (value - old_start) * ((new_end - new_start) / (old_end - old_start)) + new_start
}
