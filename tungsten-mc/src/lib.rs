#![allow(warnings)]

use crate::mathf64::*;
use crate::utilsf64::*;

// include!(concat!(env!("OUT_DIR"), "/generated_worldgen.rs"));
include!("density.rs");

//pub mod density_function;
pub mod math;
pub mod mathf64;
//mod orchestration;
pub mod perlin;
pub mod random;
pub mod utils;
pub mod utilsf64;
pub mod xoroshiro;

pub use mathf64::{Pos3, Vec3};

use crate::orchestration::HostConsts;

/// Deserialize HostConsts from binary data
/// The binary format is a direct representation: BlockBox (48 bytes) + 150 Jigsaws (2400 bytes) + 150 Pieces (8400 bytes)
pub fn deserialize_host_consts(data: &[u8]) -> Result<HostConsts, String> {
    if data.len() < 10848 {
        return Err(format!(
            "Data too small: {} bytes, expected 10848",
            data.len()
        ));
    }

    let mut offset = 0;

    // Read BlockBox (6 f64 values = 48 bytes)
    let base_box = {
        let min_x = f64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);
        offset += 8;
        let min_y = f64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);
        offset += 8;
        let min_z = f64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);
        offset += 8;
        let max_x = f64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);
        offset += 8;
        let max_y = f64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);
        offset += 8;
        let max_z = f64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);
        offset += 8;

        BlockBox {
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
        }
    };

    // Read Jigsaw array (150 items, each 4 i32 = 16 bytes, total 2400 bytes)
    let mut jigsaw_array = [Jigsaw::default(); 500];
    for i in 0..jigsaw_array.len() {
        let source_x = i32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);
        offset += 4;
        let source_ground_y = i32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);
        offset += 4;
        let source_z = i32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);
        offset += 4;
        let delta_y = i32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);
        offset += 4;

        jigsaw_array[i] = Jigsaw {
            source_x,
            source_ground_y,
            source_z,
            delta_y,
        };
    }

    // Read Piece array (150 items, each BlockBox + 2 i32 = 56 bytes, total 8400 bytes)
    let mut piece_array = [Piece::default(); 500];
    for i in 0..piece_array.len() {
        // Read BlockBox
        let min_x = f64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);
        offset += 8;
        let min_y = f64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);
        offset += 8;
        let min_z = f64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);
        offset += 8;
        let max_x = f64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);
        offset += 8;
        let max_y = f64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);
        offset += 8;
        let max_z = f64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);
        offset += 8;

        let block_box = BlockBox {
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
        };

        // Read terrain_adjustment and ground_level_delta
        let terrain_adjustment = i32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);
        offset += 4;
        let ground_level_delta = i32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);
        offset += 4;

        piece_array[i] = Piece {
            block_box,
            terrain_adjustment,
            ground_level_delta,
        };
    }

    Ok(HostConsts {
        base_box,
        jigsaw_array,
        piece_array,
    })
}

/// Compute a full 16x256x16 density chunk with the given seed and origin.
/// Returns 13 output arrays (one for each density function output).
// pub fn compute_density_chunk(
//     seed: u32,
//     origin: Vec3,
// ) -> [Box<[f32; 65536]>; 13] {
//     let (a, b, c, d, e, f, g, h, i, j, k, l, m) = orchestration_seeded(seed, origin);
//     [a, b, c, d, e, f, g, h, i, j, k, l, m]
// }

impl Default for HostConsts {
    fn default() -> Self {
        HostConsts {
            base_box: BlockBox::default(),
            jigsaw_array: [Jigsaw::default(); 500],
            piece_array: [Piece::default(); 500],
        }
    }
}
