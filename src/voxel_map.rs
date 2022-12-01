use crate::voxel_data::{self, CHUNK_HEIGHT, CHUNK_WIDTH};
use crate::world;
use crate::world::WORLD_SIZE;
use itertools::Itertools;
use ndarray::Array3;
use noise::{BasicMulti, NoiseFn, OpenSimplex, Perlin};
use std::cmp::{Ord, Ordering};

#[derive(Clone, Debug, Default)]
pub struct VoxelMap {
    pub voxels: Array3<u8>,
}

impl VoxelMap {
    pub fn new() -> Self {
        VoxelMap {
            voxels: Array3::<u8>::from_elem((WORLD_SIZE, CHUNK_HEIGHT, WORLD_SIZE), 0),
        }
    }

    pub fn populate_voxel_map(&mut self, chunk_pos: world::ChunkCoord) {
        let noise = Perlin::new();
        let scale = 60.;
        let octave_number = 4;

        let shifted_x = (chunk_pos.x * CHUNK_WIDTH as i32 + (WORLD_SIZE / 2) as i32) as usize;
        let shifted_z = (chunk_pos.z * CHUNK_WIDTH as i32 + (WORLD_SIZE / 2) as i32) as usize;

        for (x, z) in (0 as usize..CHUNK_WIDTH + 1).cartesian_product(0 as usize..CHUNK_WIDTH + 1) {
            let global_x = shifted_x + x;
            let global_z = shifted_z + z;
            let mut frequency = 1.0;
            let mut amplitude = 1.0;

            if global_x < WORLD_SIZE && global_z < WORLD_SIZE && global_x > 0 && global_z > 0 {
                let mut noise_value = 0.0;
                for _ in 0..octave_number {
                    noise_value += amplitude
                       * (noise.get([
                                frequency * global_x as f64 / scale,
                                frequency * global_z as f64 / scale,
                            ]));
                    frequency *= 2.0;
                    amplitude /= 2.0;
                }
                let threshold = ((noise_value / 1.875 + 1.0) / 2.0 * CHUNK_HEIGHT as f64).floor() as usize;
                dbg!(threshold);


                    for y in 0..CHUNK_HEIGHT {
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
    }
}
