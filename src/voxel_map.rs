use crate::voxel_data::CHUNK_SIZE;
use crate::world;
use crate::world::{WORLD_HEIGHT, WORLD_SIZE};
use itertools::Itertools;
use ndarray::Array3;
use noise::{NoiseFn, Perlin};
use std::cmp::{Ord, Ordering};
use bevy::log::info_span;
use splines::{Interpolation, Key, Spline};

#[derive(Clone, Debug, Default)]
pub struct VoxelMap {
    pub voxels: Array3<u8>,
}

impl VoxelMap {
    pub fn new() -> Self {
        VoxelMap {
            voxels: Array3::<u8>::from_elem((WORLD_SIZE, WORLD_HEIGHT, WORLD_SIZE), 0),
        }
    }

    pub fn populate_voxel_map(&mut self, chunk_pos: world::ChunkCoord) {
        let _span = info_span!("VoxelMap population").entered();
        let noise = Perlin::new();
        let scale = 300.;
        let octave_number = 4;

        let start = Key::new(-1., 5., Interpolation::Linear);
        let point1 = Key::new(-0.8, 10., Interpolation::Linear);
        let point2 = Key::new(-0.5, 10., Interpolation::Linear);
        let point3 = Key::new(-0.6, 40., Interpolation::Linear);
        let point4 = Key::new(-0.5, 40., Interpolation::Linear);
        let point5= Key::new(-0.4, 80., Interpolation::Linear);
        let point6= Key::new(-0.3, 80., Interpolation::Linear);
        let end = Key::new(1., 127., Interpolation::default());
        let spline = Spline::from_vec(vec![start, point1, point2, point3, point4, point5, point6, end]);

        let shifted_x = (chunk_pos.x * CHUNK_SIZE as i32 + (WORLD_SIZE / 2) as i32) as usize;
        let shifted_y = chunk_pos.y as usize * CHUNK_SIZE;
        let shifted_z = (chunk_pos.z * CHUNK_SIZE as i32 + (WORLD_SIZE / 2) as i32) as usize;

        for (x, z) in (0 as usize..CHUNK_SIZE + 1).cartesian_product(0 as usize..CHUNK_SIZE + 1) {
            let global_x = shifted_x + x;
            let global_z = shifted_z + z;
            let mut frequency = 1.0;
            let mut amplitude = 1.0;

            if global_x < WORLD_SIZE && global_z < WORLD_SIZE {
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
                let threshold = spline.sample(noise_value/1.875).unwrap().floor() as usize; //((noise_value / 1.875 + 1.0) / 2.0 * CHUNK_HEIGHT as f64).floor() as usize;

                    for mut y in 0..CHUNK_SIZE {
                        y = y + shifted_y;
                        if y < 50 {
                            self.voxels[[global_x, y, global_z]] = 5;
                        }
                        match y.cmp(&threshold) {
                            Ordering::Less => {
                               if y == 0 {
                                    self.voxels[[global_x, y, global_z]] = 2;
                                }  else if (threshold - y) == 1 {
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
