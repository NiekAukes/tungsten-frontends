pub mod density_function {
    // Auto-generated inline Rust module from RCL
    #[allow(dead_code)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(warnings)]
    #[allow(clippy::all)]
    
    use super::*;
    use super::orchestration::*;
    
    #[inline]
    fn preliminary_surface_level_d128x1x128(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return 0_f64;
    }
    
    #[inline]
    fn Beardifier_d16x384x16(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut ret: f64;
        let mut current_piece: Piece;
        let mut current_box: BlockBox;
        let mut current_jigsaw: Jigsaw;
        let mut i: i32;
        let mut m: i32;
        let mut n: i32;
        let mut o: i32;
        let mut p_1: i32;
        let mut q: i32;
        let mut s: i32;
        let mut r: i32;
        let mut l: i32;
        let mut m_1: i32;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        ret = 0_f64;
        if do_beardify(host_consts.base_box, rpos3) {
            i = 0_i32;
            while (i < 500_i32) {
                current_piece = host_consts.piece_array[i as usize];
                current_box = current_piece.block_box;
                if (!valid_box(current_box)) {
                    break;
                }
                m = (max(0_f64, max((current_box.min_x - rpos3.x), (rpos3.x - current_box.max_x))) as i32);
                n = (max(0_f64, max((current_box.min_z - rpos3.z), (rpos3.z - current_box.max_z))) as i32);
                o = ((current_box.min_y + (current_piece.ground_level_delta as f64)) as i32);
                p_1 = ((rpos3.y - (o as f64)) as i32);
                s = current_piece.terrain_adjustment;
                q = 0_i32;
                if ((s == 1_i32) || (s == 2_i32)) {
                    q = p_1;
                }
                if (s == 3_i32) {
                    q = (max(0_f64, max(((o as f64) - rpos3.y), (rpos3.y - current_box.max_y))) as i32);
                }
                if (s == 4_i32) {
                    q = (max(0_f64, max((current_box.min_y - rpos3.y), (rpos3.y - current_box.max_y))) as i32);
                }
                if (s == 1_i32) {
                    ret = (ret + get_magnitude_weight((m as f64), ((q as f64) / 2_f64), (n as f64)));
                }
                if ((s == 2_i32) || (s == 3_i32)) {
                    ret = (ret + (get_structure_weight(m, q, n, p_1) * 0.8_f64));
                }
                if (s == 4_i32) {
                    ret = (ret + (get_magnitude_weight(((m as f64) / 2_f64), ((q as f64) / 2_f64), ((n as f64) / 2_f64)) * 0.8_f64));
                }
                i = (i + 1_i32);
            }
            i = 0_i32;
            while (i < 500_i32) {
                current_jigsaw = host_consts.jigsaw_array[i as usize];
                if (((current_jigsaw.source_x == -1073741824_i32) && (current_jigsaw.source_ground_y == -1073741824_i32)) && (current_jigsaw.source_z == -1073741824_i32)) {
                    break;
                }
                r = ((rpos3.x as i32) - current_jigsaw.source_x);
                l = ((rpos3.y as i32) - current_jigsaw.source_ground_y);
                m_1 = ((rpos3.z as i32) - current_jigsaw.source_z);
                ret = (ret + (get_structure_weight(r, l, m_1, l) * 0.4_f64));
                i = (i + 1_i32);
            }
        }
        return ret;
    }
    
    #[inline]
    fn minecraft_offset_3_d4x1x4(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut n: f64;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + (pos3 * Vec3::new(1_f64, 0_f64, 1_f64)));
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.125_f64), (&auto_constants.minecraft_offset_0_octave__3)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.12726586102719034_f64), (&auto_constants.minecraft_offset_1_octave__3)) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.25_f64), (&auto_constants.minecraft_offset_0_octave__2)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.2545317220543807_f64), (&auto_constants.minecraft_offset_1_octave__2)) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.5_f64), (&auto_constants.minecraft_offset_0_octave__1)) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.5090634441087614_f64), (&auto_constants.minecraft_offset_1_octave__1)) * 0.13333333333333333_f64));
        return (n * 1.25_f64);
    }
    
    #[inline]
    fn density_function_Cache2d_2_d4x1x4(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return (buffers.minecraft_offset_3_d4x1x4_output[flat_y_zero_index(pos3, 4_i32, 1_i32) as usize] * 4_f64);
    }
    
    #[inline]
    fn density_function_Cache2d_1_d4x1x4(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.density_function_Cache2d_2_d4x1x4_output[index(p) as usize];
    }
    
    #[inline]
    fn density_function_FlatCache_0_d16x384x16(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.density_function_Cache2d_1_d4x1x4_output[biome_column_index(pos3, 4_i32, 4_i32) as usize];
    }
    
    #[inline]
    fn final_density_with_beardify_d16x384x16(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return (buffers.density_function_FlatCache_0_d16x384x16_output[flat_y_zero_index(pos3, 16_i32, 16_i32) as usize] + buffers.Beardifier_d16x384x16_output[index(p) as usize]);
    }
    
    #[inline]
    fn final_density_d16x384x16(p: PositionIterator, config: &Config, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) -> f64 {
        let mut rpos3: Vec3;
        let mut origin: Vec3 = config.origin;
        let mut pos3: Pos3 = p.pos;
        let mut origin_scale: Vec3 = config.origin_scale;
        let mut position_scale: Vec3 = config.position_scale;
        rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
        return buffers.density_function_FlatCache_0_d16x384x16_output[flat_y_zero_index(pos3, 16_i32, 16_i32) as usize];
    }
    
    pub fn preliminary_surface_level_d128x1x128_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(4_f64, 8_f64, 4_f64), dimensions: (128_i32, 1_i32, 128_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = preliminary_surface_level_d128x1x128(p, (&config), buffers, auto_constants, host_consts);
            buffers.preliminary_surface_level_d128x1x128_output[index(p) as usize] = r;
        }
    }
    
    pub fn Beardifier_d16x384x16_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(1_f64, 1_f64, 1_f64), dimensions: (16_i32, 384_i32, 16_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = Beardifier_d16x384x16(p, (&config), buffers, auto_constants, host_consts);
            buffers.Beardifier_d16x384x16_output[index(p) as usize] = r;
        }
    }
    
    pub fn minecraft_offset_3_d4x1x4_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(0.25_f64, 0_f64, 0.25_f64), position_scale: Vec3::new(1_f64, 0_f64, 1_f64), dimensions: (4_i32, 1_i32, 4_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = minecraft_offset_3_d4x1x4(p, (&config), buffers, auto_constants, host_consts);
            buffers.minecraft_offset_3_d4x1x4_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Cache2d_2_d4x1x4_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (4_i32, 1_i32, 4_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Cache2d_2_d4x1x4(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Cache2d_2_d4x1x4_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_Cache2d_1_d4x1x4_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 0_f64, 1_f64), position_scale: Vec3::new(4_f64, 0_f64, 4_f64), dimensions: (4_i32, 1_i32, 4_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_Cache2d_1_d4x1x4(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_Cache2d_1_d4x1x4_output[index(p) as usize] = r;
        }
    }
    
    pub fn density_function_FlatCache_0_d16x384x16_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(1_f64, 1_f64, 1_f64), dimensions: (16_i32, 384_i32, 16_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = density_function_FlatCache_0_d16x384x16(p, (&config), buffers, auto_constants, host_consts);
            buffers.density_function_FlatCache_0_d16x384x16_output[index(p) as usize] = r;
        }
    }
    
    pub fn final_density_with_beardify_d16x384x16_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(1_f64, 1_f64, 1_f64), dimensions: (16_i32, 384_i32, 16_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = final_density_with_beardify_d16x384x16(p, (&config), buffers, auto_constants, host_consts);
            buffers.final_density_with_beardify_d16x384x16_output[index(p) as usize] = r;
        }
    }
    
    pub fn final_density_d16x384x16_kernel(origin: Vec3, buffers: &mut InternalBuffers, auto_constants: &AutoConsts, host_consts: &HostConsts) {
        let config: Config = Config { origin: origin, origin_scale: Vec3::new(1_f64, 1_f64, 1_f64), position_scale: Vec3::new(1_f64, 1_f64, 1_f64), dimensions: (16_i32, 384_i32, 16_i32) };
        for p in iter_3d((&config)) {
            let r: f64 = final_density_d16x384x16(p, (&config), buffers, auto_constants, host_consts);
            buffers.final_density_d16x384x16_output[index(p) as usize] = r;
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
        pub base_box: BlockBox,
        pub jigsaw_array: [Jigsaw; 500],
        pub piece_array: [Piece; 500],
    }
    
    #[repr(C)]
    #[derive(Clone)]
    pub struct AutoConsts {
        pub minecraft_offset_0_octave__3: PerlinNoiseSampler,
        pub minecraft_offset_1_octave__3: PerlinNoiseSampler,
        pub minecraft_offset_0_octave__2: PerlinNoiseSampler,
        pub minecraft_offset_1_octave__2: PerlinNoiseSampler,
        pub minecraft_offset_0_octave__1: PerlinNoiseSampler,
        pub minecraft_offset_1_octave__1: PerlinNoiseSampler,
    }
    
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct InternalBuffers {
        pub final_density_with_beardify_d16x384x16_output: [f64; 98304],
        pub final_density_d16x384x16_output: [f64; 98304],
        pub preliminary_surface_level_d128x1x128_output: [f64; 16384],
        pub density_function_FlatCache_0_d16x384x16_output: [f64; 98304],
        pub Beardifier_d16x384x16_output: [f64; 98304],
        pub density_function_Cache2d_1_d4x1x4_output: [f64; 16],
        pub density_function_Cache2d_2_d4x1x4_output: [f64; 16],
        pub minecraft_offset_3_d4x1x4_output: [f64; 16],
    }
    
    #[derive(Clone)]
    pub struct Orchestration {
        pub buffers: Box<InternalBuffers>,
        pub auto_constants: Box<AutoConsts>,
    }
    
    fn make_auto_consts(seed: i64, auto_consts: &mut AutoConsts) {
        auto_consts.minecraft_offset_0_octave__3 = make_permutation_table(seed, 577895406318539652_i64, 4557074653038767061_i64, 0_i64, 6040343492819601496_i64, -4840870798921671874_i64);
        auto_consts.minecraft_offset_1_octave__3 = make_permutation_table(seed, 577895406318539652_i64, 4557074653038767061_i64, 1_i64, 6040343492819601496_i64, -4840870798921671874_i64);
        auto_consts.minecraft_offset_0_octave__2 = make_permutation_table(seed, 577895406318539652_i64, 4557074653038767061_i64, 0_i64, -5430693012044290181_i64, 162122330481997252_i64);
        auto_consts.minecraft_offset_1_octave__2 = make_permutation_table(seed, 577895406318539652_i64, 4557074653038767061_i64, 1_i64, -5430693012044290181_i64, 162122330481997252_i64);
        auto_consts.minecraft_offset_0_octave__1 = make_permutation_table(seed, 577895406318539652_i64, 4557074653038767061_i64, 0_i64, -2307493697404144120_i64, -5064731985740145495_i64);
        auto_consts.minecraft_offset_1_octave__1 = make_permutation_table(seed, 577895406318539652_i64, 4557074653038767061_i64, 1_i64, -2307493697404144120_i64, -5064731985740145495_i64);
    }
    
    impl Orchestration {
        pub fn new(seed: i64) -> Self {
            let mut buffers: Box<InternalBuffers> = make_boxed::<InternalBuffers>();
            let mut auto_constants: Box<AutoConsts> = make_boxed::<AutoConsts>();
            let _init: () = make_auto_consts(seed, (&mut auto_constants));
            return Self { buffers: buffers, auto_constants: auto_constants };
        }
        
        pub fn orchestrate_final_density_with_beardify(&mut self, origin: Vec3, host_consts: &HostConsts) -> &[f64; 98304] {
            let _kernel_call_0_0: () = Beardifier_d16x384x16_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_0_1: () = minecraft_offset_3_d4x1x4_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_0: () = density_function_Cache2d_2_d4x1x4_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_0: () = density_function_Cache2d_1_d4x1x4_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_3_0: () = density_function_FlatCache_0_d16x384x16_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_4_0: () = final_density_with_beardify_d16x384x16_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            return (&self.buffers.final_density_with_beardify_d16x384x16_output);
        }
        
        pub fn orchestrate_final_density(&mut self, origin: Vec3, host_consts: &HostConsts) -> &[f64; 98304] {
            let _kernel_call_0_0: () = minecraft_offset_3_d4x1x4_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_1_0: () = density_function_Cache2d_2_d4x1x4_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_2_0: () = density_function_Cache2d_1_d4x1x4_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_3_0: () = density_function_FlatCache_0_d16x384x16_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            let _kernel_call_4_0: () = final_density_d16x384x16_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            return (&self.buffers.final_density_d16x384x16_output);
        }
        
        pub fn orchestrate_preliminary_surface_level(&mut self, origin: Vec3, host_consts: &HostConsts) -> &[f64; 16384] {
            let _kernel_call_0_0: () = preliminary_surface_level_d128x1x128_kernel(origin, (&mut (*self.buffers)), (&(*self.auto_constants)), host_consts);
            return (&self.buffers.preliminary_surface_level_d128x1x128_output);
        }
    }
    
}
