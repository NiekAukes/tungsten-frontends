use std::collections::HashMap;

use crate::{
    mathf64::Vec3,
    orchestration::{HostConsts, Orchestration},
};

/// This module provides functionality for caching the preliminary surface level of the world.
/// It is sampled 32x32 chunks at a time to improve performance by reducing redundant calculations.
/// Implemented as both a single value cache, and a chunk-based cache for the aquifer sampling.

pub struct PreliminarySurfaceCache {
    cache: HashMap<(i32, i32), i32>,
    orchestration: Orchestration,
}

impl PreliminarySurfaceCache {
    pub fn new(orchestration: Orchestration) -> Self {
        Self {
            cache: HashMap::new(),
            orchestration,
        }
    }

    fn compute_and_cache(&mut self, biome_x: i32, biome_z: i32) {
        let origin_x = biome_x & !31;
        let origin_z = biome_z & !31;

        let origin = Vec3::new(origin_x as f64, -64.0, origin_z as f64);
        let host_consts = HostConsts::default();
        let result = self
            .orchestration
            .orchestrate_preliminary_surface_level(origin, &host_consts);
        for dz in 0..32 {
            for dx in 0..32 {
                let key = (origin_x + dx, origin_z + dz);
                self.cache
                    .insert(key, result[dx as usize + dz as usize * 32] as i32);
            }
        }
    }

    pub fn get(&mut self, biome_x: i32, biome_z: i32) -> i32 {
        let key = (biome_x, biome_z);
        if let Some(&value) = self.cache.get(&key) {
            return value;
        }

        // Compute the preliminary surface level using the orchestration instance
        self.compute_and_cache(biome_x, biome_z);
        let value = self.cache.get(&key).copied().unwrap();
        value
    }

    pub fn fill_cache_for_chunk(&mut self, chunk_x: i32, chunk_z: i32) -> [i32; 25] {
        // from the original chunk coordinates
        // -4 to +1 in z direction
        // -3 to +2 in x direction
        let mut result = [0; 25];
        let start_x = chunk_x - 3;
        let start_z = chunk_z - 4;
        let mut index = 0;
        for dz in 0..5 {
            for dx in 0..5 {
                result[index] = self.get((start_x + dx) << 2, (start_z + dz) << 2);
                index += 1;
            }
        }
        result
    }
}
