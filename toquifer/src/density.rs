pub mod density_function {
    // Auto-generated inline Rust module from RCL
    #[allow(dead_code)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(warnings)]
    #[allow(clippy::all)]
    
    use super::*;
    use super::orchestration::*;
    
    const c_surface_sample_chunk_offsets: [i32; 26] = [0_i32, 0_i32, -2_i32, -1_i32, -1_i32, -1_i32, 0_i32, -1_i32, 1_i32, -1_i32, -3_i32, 0_i32, -2_i32, 0_i32, -1_i32, 0_i32, 1_i32, 0_i32, -2_i32, 1_i32, -1_i32, 1_i32, 0_i32, 1_i32, 1_i32, 1_i32];
    #[inline]
    fn aquifer_random_d3x35x3(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> i64 {
        let mut rnd: Xoroshiro128PlusPlusRandom;
        let mut r: i64;
        let mut x: i32;
        let mut y: i32;
        let mut z: i32;
        let mut rnd_x: i32;
        let mut rnd_y: i32;
        let mut rnd_z: i32;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        let mut pos3_1: Pos3 = p.pos;
        x = ((((origin.x as i32) - 5_i32) >> 4_i32) + pos3_1.x);
        y = (-7_i32 + pos3_1.y);
        z = ((((origin.z as i32) - 5_i32) >> 4_i32) + pos3_1.z);
        rnd = split_random(host_consts.splitter, (x as i32), (y as i32), (z as i32));
        rnd_x = random_next_int((&mut rnd), 10_i32);
        rnd_y = random_next_int((&mut rnd), 9_i32);
        rnd_z = random_next_int((&mut rnd), 10_i32);
        r = packed_pos((cell_to_block_x((x as i32), (rnd_x as i32)) as i32), (cell_to_block_y((y as i32), (rnd_y as i32)) as i32), (cell_to_block_z((z as i32), (rnd_z as i32)) as i32));
        return r;
    }
    
    #[inline]
    fn water_levels_d3x35x3(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> i64 {
        let mut x: i32;
        let mut y: i32;
        let mut z: i32;
        let mut r: i64;
        let mut cell_site: i64;
        let mut fluid_level_var: i64;
        let mut fluid_type_var: i64;
        let mut lowest_surface_y: i32;
        let mut upper_y: i32;
        let mut lower_y: i32;
        let mut origin_column_has_surface_fluid: bool;
        let mut i: i32;
        let mut sample_x: i32;
        let mut sample_z: i32;
        let mut surface_y: i32;
        let mut aquifer_surface_y: i32;
        let mut surface_fluid_level: i32;
        let mut surface_fluid_block: i64;
        let mut is_origin_column: bool;
        let mut is_near_surface: bool;
        let mut noise_based_fluid_score: f64;
        let mut default_fluid_score: f64;
        let mut fluid_y: i64;
        let mut surface_proximity: f64;
        let mut floodedness_sample: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        cell_site = buffers.aquifer_random_d3x35x3_output[index(p) as usize];
        x = unpack_long_x(cell_site);
        y = unpack_long_y(cell_site);
        z = unpack_long_z(cell_site);
        fluid_level_var = (default_fluid_level(p, config, buffers, auto_constants, host_consts, y) as i64);
        fluid_type_var = (default_fluid_block(p, config, buffers, auto_constants, host_consts, y) as i64);
        lowest_surface_y = 2147483647_i32;
        upper_y = (y + 12_i32);
        lower_y = (y - 12_i32);
        origin_column_has_surface_fluid = false;
        i = 0_i32;
        while (i < 13_i32) {
            sample_x = (x + (c_surface_sample_chunk_offsets[(i * 2_i32) as usize] << 4_i32));
            sample_z = (z + (c_surface_sample_chunk_offsets[((i * 2_i32) + 1_i32) as usize] << 4_i32));
            surface_y = esh(host_consts.estimate_surface_height, (sample_x as i32), (sample_z as i32));
            aquifer_surface_y = (surface_y + 8_i32);
            is_origin_column = ((sample_x == x) && (sample_z == z));
            if (is_origin_column && (lower_y > aquifer_surface_y)) {
                return (((fluid_level_var as i64) << 32_i64) | fluid_type_var);
            }
            is_near_surface = (upper_y > aquifer_surface_y);
            if (is_near_surface || is_origin_column) {
                surface_fluid_block = (default_fluid_block(p, config, buffers, auto_constants, host_consts, aquifer_surface_y) as i64);
                surface_fluid_level = default_fluid_level(p, config, buffers, auto_constants, host_consts, aquifer_surface_y);
                if (surface_fluid_level <= aquifer_surface_y) {
                    if is_near_surface {
                        return (((surface_fluid_level as i64) << 32_i64) | surface_fluid_block);
                    }
                    if is_origin_column {
                        origin_column_has_surface_fluid = true;
                    }
                }
            }
            lowest_surface_y = min_i32((lowest_surface_y as i32), (surface_y as i32));
            i = (i + 1_i32);
        }
        surface_proximity = 0_f64;
        if ((erosion(host_consts.erosion_cache, (x as i32), (y as i32), (z as i32)) < -0.225_f64) && (depth(host_consts.depth_cache, (x as i32), (y as i32), (z as i32)) > 0.9_f64)) {
            noise_based_fluid_score = -1_f64;
            default_fluid_score = -1_f64;
        } else {
            if origin_column_has_surface_fluid {
                surface_proximity = (((clamp((((lowest_surface_y + 8_i32) - y) as f64), 0_f64, 64_f64) - 0_f64) * -0.015625_f64) + 1_f64);
            }
            floodedness_sample = clamp(fluid_level_floodedness(p, config, buffers, auto_constants, host_consts, Vec3::new((x as f64), (y as f64), (z as f64))), -1_f64, 1_f64);
            noise_based_fluid_score = (floodedness_sample - (((surface_proximity - 1_f64) * -1.2000000000000002_f64) + -0.8_f64));
            default_fluid_score = (floodedness_sample - (((surface_proximity - 1_f64) * -1.1_f64) + -0.3_f64));
        }
        if (default_fluid_score > 0_f64) {
            fluid_y = (default_fluid_level(p, config, buffers, auto_constants, host_consts, y) as i64);
        } else {
            if (noise_based_fluid_score > 0_f64) {
                fluid_y = (min_i32((lowest_surface_y as i32), (((((floor_div((y as i32), 40_i32) * 40_i32) + 20_i32) + ((floor(((fluid_level_spread(p, config, buffers, auto_constants, host_consts, Vec3::new((floor_div((x as i32), 16_i32) as f64), (floor_div((y as i32), 40_i32) as f64), (floor_div((z as i32), 16_i32) as f64))) * 10_f64) / 3_f64)) * 3_f64) as i32)) as i32) as i32)) as i64);
            } else {
                fluid_y = (-1024_i32 as i64);
            }
        }
        if (((fluid_y <= -10_i64) && (!(fluid_y == -1024_i64))) && (!(fluid_type_var == 4_i64))) {
            if (abs(fluid_type(p, config, buffers, auto_constants, host_consts, Vec3::new(((floor_div((x as i32), 64_i32) as f64) as f64), ((floor_div((y as i32), 40_i32) as f64) as f64), ((floor_div((z as i32), 64_i32) as f64) as f64)))) > 0.3_f64) {
                fluid_type_var = (4_i32 as i64);
            }
        }
        return (((fluid_y as i64) << 32_i64) | fluid_type_var);
    }
    
    #[inline]
    fn default_fluid_level(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, y: i32) -> i32 {
        if (y < -54_i32) {
            return -54_i32;
        } else {
            return 63_i32;
        }
    }
    
    #[inline]
    fn default_fluid_block(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, y: i32) -> i32 {
        if (y < -54_i32) {
            return 4_i32;
        } else {
            return 3_i32;
        }
    }
    
    #[inline]
    fn fluid_level_floodedness(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        return minecraft_aquifer_fluid_level_floodedness(p, config, buffers, auto_constants, host_consts, (rpos3 * Vec3::new(1_f64, 0.67_f64, 1_f64)));
    }
    
    #[inline]
    fn minecraft_aquifer_fluid_level_floodedness(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.0078125_f64), (&auto_constants.minecraft_aquifer_fluid_level_floodedness_0_octave__7)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), (&auto_constants.minecraft_aquifer_fluid_level_floodedness_1_octave__7)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn fluid_level_spread(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        return minecraft_aquifer_fluid_level_spread(p, config, buffers, auto_constants, host_consts, (rpos3 * Vec3::new(1_f64, 0.7142857142857143_f64, 1_f64)));
    }
    
    #[inline]
    fn minecraft_aquifer_fluid_level_spread(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.03125_f64), (&auto_constants.minecraft_aquifer_fluid_level_spread_0_octave__5)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), (&auto_constants.minecraft_aquifer_fluid_level_spread_1_octave__5)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn fluid_type(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        return minecraft_aquifer_lava(p, config, buffers, auto_constants, host_consts, (rpos3 * Vec3::new(1_f64, 1_f64, 1_f64)));
    }
    
    #[inline]
    fn minecraft_aquifer_lava(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.5_f64), (&auto_constants.minecraft_aquifer_lava_0_octave__1)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.5090634441087614_f64), (&auto_constants.minecraft_aquifer_lava_1_octave__1)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn GeneralAquifer_d16x384x16(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> i32 {
        let mut rpos3: Vec3;
        let mut barrier_noise: f64;
        let mut closest_fluid: i64;
        let mut second_closest_fluid: i64;
        let mut third_closest_fluid: i64;
        let mut density_adjustment_12: f64;
        let mut pair_factor_13: f64;
        let mut density_adjustment_13: f64;
        let mut pair_factor_23: f64;
        let mut density_adjustment_23: f64;
        let mut needs_fluid_tick: bool;
        let mut final_density: f64;
        let mut blockX: i32;
        let mut blockY: i32;
        let mut blockZ: i32;
        let mut default_fluid_block_var: i32;
        let mut default_fluid_level_var: i32;
        let mut start_x: i32;
        let mut start_z: i32;
        let mut end_x: i32;
        let mut end_z: i32;
        let mut closest_distance_sq1: i32;
        let mut closest_distance_sq2: i32;
        let mut closest_distance_sq3: i32;
        let mut closest_distance_sq4: i32;
        let mut closest_index1: i32;
        let mut closest_index2: i32;
        let mut closest_index3: i32;
        let mut closest_index4: i32;
        let mut neighbor_index: i32;
        let mut cell_site: i64;
        let mut cell_offset_x: i32;
        let mut cell_offset_y: i32;
        let mut cell_offset_z: i32;
        let mut delta_x: i32;
        let mut delta_y: i32;
        let mut delta_z: i32;
        let mut distance_sq: i32;
        let mut closest_fluid_level: i64;
        let mut closest_pair_factor: f64;
        let mut result_state: i32;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        final_density = 0_f64;
        if (final_density > 0_f64) {
            return 0_i32;
        }
        blockX = (rpos3.x as i32);
        blockY = (rpos3.y as i32);
        blockZ = (rpos3.z as i32);
        default_fluid_block_var = default_fluid_block(p, config, buffers, auto_constants, host_consts, blockY);
        default_fluid_level_var = default_fluid_level(p, config, buffers, auto_constants, host_consts, blockY);
        if (blockY > host_consts.aquifer_max_y) {
            return (block_state((default_fluid_block_var as i32), (default_fluid_level_var as i32), (blockY as i32)) << 1_i32);
        }
        if (default_fluid_block_var == 4_i32) {
            return 8_i32;
        }
        start_x = (((origin.x - 5_f64) as i32) >> 4_i32);
        start_z = (((origin.z - 5_f64) as i32) >> 4_i32);
        end_x = (((((origin.x + 15_f64) - 5_f64) as i32) >> 4_i32) + 1_i32);
        end_z = (((((origin.z + 15_f64) - 5_f64) as i32) >> 4_i32) + 1_i32);
        closest_distance_sq1 = 2147483647_i32;
        closest_distance_sq2 = 2147483647_i32;
        closest_distance_sq3 = 2147483647_i32;
        closest_distance_sq4 = 2147483647_i32;
        closest_index1 = 0_i32;
        closest_index2 = 0_i32;
        closest_index3 = 0_i32;
        closest_index4 = 0_i32;
        cell_offset_x = 0_i32;
        cell_offset_y = -1_i32;
        cell_offset_z = 0_i32;
        while (cell_offset_x < 2_i32) {
            while (cell_offset_y < 2_i32) {
                while (cell_offset_z < 2_i32) {
                    neighbor_index = aquifer_index((start_x as i32), -7_i32, (start_z as i32), (end_x as i32), (end_z as i32), 35_i32, ((((blockX + -5_i32) >> 4_i32) + cell_offset_x) as i32), ((floor_div(((blockY + 1_i32) as i32), 12_i32) + cell_offset_y) as i32), ((((blockZ + -5_i32) >> 4_i32) + cell_offset_z) as i32));
                    cell_site = buffers.aquifer_random_d3x35x3_output[neighbor_index as usize];
                    delta_x = (unpack_long_x(cell_site) - blockX);
                    delta_y = (unpack_long_y(cell_site) - blockY);
                    delta_z = (unpack_long_z(cell_site) - blockZ);
                    distance_sq = (((delta_x * delta_x) + (delta_y * delta_y)) + (delta_z * delta_z));
                    if (distance_sq <= closest_distance_sq1) {
                        closest_distance_sq4 = closest_distance_sq3;
                        closest_distance_sq3 = closest_distance_sq2;
                        closest_distance_sq2 = closest_distance_sq1;
                        closest_distance_sq1 = distance_sq;
                        closest_index4 = closest_index3;
                        closest_index3 = closest_index2;
                        closest_index2 = closest_index1;
                        closest_index1 = neighbor_index;
                    } else {
                        if (distance_sq <= closest_distance_sq2) {
                            closest_distance_sq4 = closest_distance_sq3;
                            closest_distance_sq3 = closest_distance_sq2;
                            closest_distance_sq2 = distance_sq;
                            closest_index4 = closest_index3;
                            closest_index3 = closest_index2;
                            closest_index2 = neighbor_index;
                        } else {
                            if (distance_sq <= closest_distance_sq3) {
                                closest_distance_sq4 = closest_distance_sq3;
                                closest_distance_sq3 = distance_sq;
                                closest_index4 = closest_index3;
                                closest_index3 = neighbor_index;
                            } else {
                                if (distance_sq <= closest_distance_sq4) {
                                    closest_distance_sq4 = distance_sq;
                                    closest_index4 = neighbor_index;
                                }
                            }
                        }
                    }
                    cell_offset_z = (cell_offset_z + 1_i32);
                }
                cell_offset_y = (cell_offset_y + 1_i32);
                cell_offset_z = 0_i32;
            }
            cell_offset_x = (cell_offset_x + 1_i32);
            cell_offset_y = -1_i32;
        }
        closest_fluid_level = buffers.water_levels_d3x35x3_output[closest_index1 as usize];
        closest_pair_factor = max_distance(closest_distance_sq1, closest_distance_sq2);
        result_state = get_block_state_from_packed(closest_fluid_level, (blockY as i32));
        if (closest_pair_factor <= 0_f64) {
            return ((result_state << 1_i32) | (((closest_pair_factor >= -0.76_f64) && (!(closest_fluid_level == (buffers.water_levels_d3x35x3_output[closest_index2 as usize] as i64)))) as i32));
        }
        if ((result_state == 3_i32) && (block_state((default_fluid_block(p, config, buffers, auto_constants, host_consts, (blockY - 1_i32)) as i32), (default_fluid_level(p, config, buffers, auto_constants, host_consts, (blockY - 1_i32)) as i32), ((blockY - 1_i32) as i32)) == 4_i32)) {
            return ((result_state << 1_i32) | 1_i32);
        }
        barrier_noise = barrier(p, config, buffers, auto_constants, host_consts, rpos3);
        closest_fluid = buffers.water_levels_d3x35x3_output[closest_index1 as usize];
        second_closest_fluid = buffers.water_levels_d3x35x3_output[closest_index2 as usize];
        third_closest_fluid = buffers.water_levels_d3x35x3_output[closest_index3 as usize];
        density_adjustment_12 = (closest_pair_factor * aquifer_calculate_density(p, config, buffers, auto_constants, host_consts, rpos3, barrier_noise, closest_fluid, second_closest_fluid));
        if ((final_density + density_adjustment_12) > 0_f64) {
            return 0_i32;
        }
        pair_factor_13 = max_distance(closest_distance_sq1, closest_distance_sq3);
        if (pair_factor_13 > 0_f64) {
            density_adjustment_13 = ((closest_pair_factor * pair_factor_13) * aquifer_calculate_density(p, config, buffers, auto_constants, host_consts, rpos3, barrier_noise, closest_fluid, third_closest_fluid));
            if ((final_density + density_adjustment_13) > 0_f64) {
                return 0_i32;
            }
        }
        pair_factor_23 = max_distance(closest_distance_sq2, closest_distance_sq3);
        if (pair_factor_23 > 0_f64) {
            density_adjustment_23 = ((closest_pair_factor * pair_factor_23) * aquifer_calculate_density(p, config, buffers, auto_constants, host_consts, rpos3, barrier_noise, second_closest_fluid, third_closest_fluid));
            if ((final_density + density_adjustment_23) > 0_f64) {
                return 0_i32;
            }
        }
        needs_fluid_tick = ((((!(closest_fluid == second_closest_fluid)) || ((pair_factor_23 >= -0.76_f64) && (!(second_closest_fluid == third_closest_fluid)))) || ((pair_factor_13 >= -0.76_f64) && (!(closest_fluid == third_closest_fluid)))) || (((pair_factor_13 > -0.76_f64) && (max_distance(closest_distance_sq1, closest_distance_sq4) >= -0.76_f64)) && (!(closest_fluid == buffers.water_levels_d3x35x3_output[closest_index4 as usize]))));
        return result_state;
    }
    
    #[inline]
    fn barrier(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        return minecraft_aquifer_barrier(p, config, buffers, auto_constants, host_consts, (rpos3 * Vec3::new(1_f64, 0.5_f64, 1_f64)));
    }
    
    #[inline]
    fn minecraft_aquifer_barrier(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.125_f64), (&auto_constants.minecraft_aquifer_barrier_0_octave__3)) * 1_f64));
        n = (n + (perlin((rpos3 * 0.12726586102719034_f64), (&auto_constants.minecraft_aquifer_barrier_1_octave__3)) * 1_f64));
        return (n * 0.8333333333333333_f64);
    }
    
    #[inline]
    fn aquifer_calculate_density(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts, rpos3: Vec3, barrier: f64, fluid_1: i64, fluid_2: i64) -> f64 {
        let mut y: i32;
        let mut block_state_1: i32;
        let mut fluid_level_1: i32;
        let mut block_state_2: i32;
        let mut fluid_level_2: i32;
        let mut pressure: f64;
        let mut shifted_distance: f64;
        let mut divider: f64;
        let mut relative_y: f64;
        let mut half_difference: f64;
        let mut fluid_level_difference: i32;
        let mut barrier_value: f64;
        y = (rpos3.y as i32);
        fluid_level_1 = get_fluid_level_from_packed(fluid_1);
        fluid_level_2 = get_fluid_level_from_packed(fluid_2);
        block_state_1 = get_block_state_from_packed(fluid_1, (y as i32));
        block_state_2 = get_block_state_from_packed(fluid_2, (y as i32));
        if (((block_state_1 == 4_i32) && (block_state_2 == 3_i32)) || ((block_state_1 == 3_i32) && (block_state_2 == 4_i32))) {
            return 2_f64;
        }
        fluid_level_difference = abs_i32(((fluid_level_1 - fluid_level_2) as i32));
        if (fluid_level_difference == 0_i32) {
            return 0_f64;
        }
        relative_y = (((y as f64) + 0.5_f64) - (0.5_f64 * ((fluid_level_1 + fluid_level_2) as f64)));
        half_difference = ((fluid_level_difference as f64) / 2_f64);
        shifted_distance = (half_difference - abs(relative_y));
        divider = 0_f64;
        barrier_value = barrier;
        if ((relative_y > 0_f64) && (shifted_distance > 0_f64)) {
            divider = 1.5_f64;
        }
        if ((relative_y > 0_f64) && (shifted_distance <= 0_f64)) {
            divider = 2.5_f64;
        }
        if (relative_y <= 0_f64) {
            shifted_distance = (3_f64 + shifted_distance);
        }
        if ((relative_y <= 0_f64) && (shifted_distance > 0_f64)) {
            divider = 3_f64;
        }
        if ((relative_y <= 0_f64) && (shifted_distance <= 0_f64)) {
            divider = 10_f64;
        }
        pressure = (shifted_distance / divider);
        if ((pressure < -2_f64) || (pressure > 2_f64)) {
            barrier_value = 0_f64;
        }
        return (2_f64 * (barrier_value + pressure));
    }
    
    #[inline]
    fn aquifer_d16x384x16(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> i32 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.GeneralAquifer_d16x384x16_output[index(p) as usize];
    }
    
    pub fn aquifer_random_d3x35x3_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(1_f64, 1_f64, 1_f64), dimensions: (3_i32, 35_i32, 3_i32) };
        for p in iter_3d((&config)) {
            let r: i64 = aquifer_random_d3x35x3(p, (&config), buffers, auto_constants, host_consts);
            buffers.aquifer_random_d3x35x3_output[index(p) as usize] = r;
        }
    }
    
    pub fn water_levels_d3x35x3_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(1_f64, 1_f64, 1_f64), dimensions: (3_i32, 35_i32, 3_i32) };
        for p in iter_3d((&config)) {
            let r: i64 = water_levels_d3x35x3(p, (&config), buffers, auto_constants, host_consts);
            buffers.water_levels_d3x35x3_output[index(p) as usize] = r;
        }
    }
    
    pub fn GeneralAquifer_d16x384x16_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(1_f64, 1_f64, 1_f64), dimensions: (16_i32, 384_i32, 16_i32) };
        for p in iter_3d((&config)) {
            let r: i32 = GeneralAquifer_d16x384x16(p, (&config), buffers, auto_constants, host_consts);
            buffers.GeneralAquifer_d16x384x16_output[index(p) as usize] = r;
        }
    }
    
    pub fn aquifer_d16x384x16_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(1_f64, 1_f64, 1_f64), dimensions: (16_i32, 384_i32, 16_i32) };
        for p in iter_3d((&config)) {
            let r: i32 = aquifer_d16x384x16(p, (&config), buffers, auto_constants, host_consts);
            buffers.aquifer_d16x384x16_output[index(p) as usize] = r;
        }
    }
    
}


pub mod orchestration {
    // Auto-generated inline Rust module from RCL
    #[allow(dead_code)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(warnings)]
    #[allow(clippy::all)]
    
    use super::*;
    use super::density_function::*;
    
    #[repr(C)]
    pub struct HostConsts {
        pub aquifer_max_y: i32,
        pub splitter: XoroshiroRandomSplitter,
        pub estimate_surface_height: EstimatedSurfaceHeight,
        pub erosion_cache: Erosion,
        pub depth_cache: Depth,
    }
    
    #[repr(C)]
    #[derive(Clone)]
    pub struct AutoConsts {
        pub minecraft_aquifer_barrier_0_octave__3: PerlinNoiseSampler,
        pub minecraft_aquifer_barrier_1_octave__3: PerlinNoiseSampler,
        pub minecraft_aquifer_fluid_level_floodedness_0_octave__7: PerlinNoiseSampler,
        pub minecraft_aquifer_fluid_level_floodedness_1_octave__7: PerlinNoiseSampler,
        pub minecraft_aquifer_fluid_level_spread_0_octave__5: PerlinNoiseSampler,
        pub minecraft_aquifer_fluid_level_spread_1_octave__5: PerlinNoiseSampler,
        pub minecraft_aquifer_lava_0_octave__1: PerlinNoiseSampler,
        pub minecraft_aquifer_lava_1_octave__1: PerlinNoiseSampler,
    }
    
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct InternalBuffers {
        pub aquifer_d16x384x16_output: [i32; 98304],
        pub GeneralAquifer_d16x384x16_output: [i32; 98304],
        pub aquifer_random_d3x35x3_output: [i64; 315],
        pub water_levels_d3x35x3_output: [i64; 315],
    }
    
    #[derive(Clone)]
    pub struct Orchestration {
        pub buffers: Box<InternalBuffers>,
        pub auto_constants: Box<AutoConsts>,
    }
    
    fn make_auto_consts(seed: i64, auto_consts: &mut AutoConsts) {
        auto_consts.minecraft_aquifer_barrier_0_octave__3 = make_permutation_table(seed, -2201981325070759617_i64, 1391399305011792652_i64, 0_i64, 6040343492819601496_i64, -4840870798921671874_i64);
        auto_consts.minecraft_aquifer_barrier_1_octave__3 = make_permutation_table(seed, -2201981325070759617_i64, 1391399305011792652_i64, 1_i64, 6040343492819601496_i64, -4840870798921671874_i64);
        auto_consts.minecraft_aquifer_fluid_level_floodedness_0_octave__7 = make_permutation_table(seed, 1971819648764795038_i64, 2506222294934710214_i64, 0_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_aquifer_fluid_level_floodedness_1_octave__7 = make_permutation_table(seed, 1971819648764795038_i64, 2506222294934710214_i64, 1_i64, -1075682932162398897_i64, 2700503254851170474_i64);
        auto_consts.minecraft_aquifer_fluid_level_spread_0_octave__5 = make_permutation_table(seed, 644356645385251490_i64, 3919847117419490415_i64, 0_i64, 7888980432583755018_i64, 3328269827262531447_i64);
        auto_consts.minecraft_aquifer_fluid_level_spread_1_octave__5 = make_permutation_table(seed, 644356645385251490_i64, 3919847117419490415_i64, 1_i64, 7888980432583755018_i64, 3328269827262531447_i64);
        auto_consts.minecraft_aquifer_lava_0_octave__1 = make_permutation_table(seed, 1031642243090684786_i64, 2820617609049802170_i64, 0_i64, -2307493697404144120_i64, -5064731985740145495_i64);
        auto_consts.minecraft_aquifer_lava_1_octave__1 = make_permutation_table(seed, 1031642243090684786_i64, 2820617609049802170_i64, 1_i64, -2307493697404144120_i64, -5064731985740145495_i64);
    }
    
    impl Orchestration {
        pub fn new(seed: i64) -> Self {
            let mut buffers: Box<InternalBuffers> = make_boxed::<InternalBuffers>();
            let mut auto_constants: Box<AutoConsts> = make_boxed::<AutoConsts>();
            let _init: () = make_auto_consts(seed, (&mut auto_constants));
            return Self { buffers: buffers, auto_constants: auto_constants };
        }
        
        pub fn orchestrate_aquifer(&mut self, origin: Vec3, host_consts: &HostConsts) -> &[i32; 98304] {
            let _kernel_call_0_0: () = aquifer_random_d3x35x3_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_0: () = water_levels_d3x35x3_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_0: () = GeneralAquifer_d16x384x16_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_3_0: () = aquifer_d16x384x16_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            return (&self.buffers.aquifer_d16x384x16_output);
        }
    }
    
}
