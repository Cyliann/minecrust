use crate::voxel_data::{self, CHUNK_WIDTH};
use itertools::iproduct;
use ndarray::Array3;
use noise::{BasicMulti, NoiseFn, Perlin};
use std::cmp::{Ord, Ordering};
use bevy::utils::default;
use crate::world;
use crate::world::WORLD_SIZE;

#[derive(Clone, Debug, Default)]
pub struct VoxelMap {
    pub voxels: Array3<u8>,
}

impl VoxelMap {
    pub fn new() -> Self {
        VoxelMap {
            voxels: Array3::<u8>::from_elem((WORLD_SIZE, voxel_data::CHUNK_HEIGHT, WORLD_SIZE), 0),
        }
    }

    pub fn populate_voxel_map(&mut self, chunk_pos: world::ChunkCoord) {
        let noise = BasicMulti::new();
        let scale = 100.;

        let shifted_x = (chunk_pos.x * CHUNK_WIDTH as i32 + (WORLD_SIZE / 2) as i32) as usize;
        let shifted_z = (chunk_pos.z * CHUNK_WIDTH as i32 + (WORLD_SIZE / 2) as i32) as usize;

        for (x, y, z) in iproduct!(
            (0 as usize..CHUNK_WIDTH),
            (0..voxel_data::CHUNK_HEIGHT),
            (0 as usize..CHUNK_WIDTH)
        ) {
            let global_x = shifted_x + x;
            let global_z = shifted_z + z;

            let threshold = (voxel_data::CHUNK_HEIGHT as f64
                * (noise.get([global_x as f64 / scale, global_z as f64 / scale]) + 1.)
                / 2.)
                .floor() as usize;
            match y.cmp(&threshold) {
                Ordering::Less => {
                    if y == 0 {
                        self.voxels[[global_x, y, global_z]] = 2;
                    } else if (threshold - y) == 1 {
                        self.voxels[[global_x, y, global_z]] = 4;
                    } else {
                        self.voxels[[global_x, y, global_z]] = 1;
                    }
                }
                Ordering::Greater => (),
                Ordering::Equal => self.voxels[[global_x, y, global_z]] = 3,
            }
        }
    }
}
