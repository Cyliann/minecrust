use crate::voxel_data::CHUNK_SIZE;
use crate::world;
use crate::world::{WORLD_HEIGHT, WORLD_SIZE};
use bevy::log::info_span;
use bracket_noise::prelude::*;
use itertools::Itertools;
use ndarray::Array3;
use splines::{Interpolation, Key, Spline};
use std::cmp::{Ord, Ordering};

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

    pub fn populate_voxel_map(&mut self, chunk_pos: world::ChunkCoord) -> bool {
        let _span = info_span!("VoxelMap population").entered();
        let mut noise = FastNoise::new();
        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(4);
        noise.set_fractal_gain(0.6);
        noise.set_fractal_lacunarity(2.0);
        noise.set_frequency(2.0);
        let scale = 500.;
        let mut counter = 0;

        let start = Key::new(-1., 5., Interpolation::Linear);
        let point1 = Key::new(-0.8, 10., Interpolation::Linear);
        let point3 = Key::new(-0.4, 40., Interpolation::Linear);
        let point4 = Key::new(-0.3, 40., Interpolation::Linear);
        let point5 = Key::new(-0., 80., Interpolation::Linear);
        let point6 = Key::new(-0.1, 80., Interpolation::Linear);
        let end = Key::new(1., 127., Interpolation::default());
        let spline = Spline::from_vec(vec![start, point1, point3, point4, point5, point6, end]);

        let shifted_x = (chunk_pos.x * CHUNK_SIZE as i32 + (WORLD_SIZE / 2) as i32) as usize;
        let shifted_y = chunk_pos.y as usize * CHUNK_SIZE;
        let shifted_z = (chunk_pos.z * CHUNK_SIZE as i32 + (WORLD_SIZE / 2) as i32) as usize;

        for (x, z) in (-1..CHUNK_SIZE as i32 + 1).cartesian_product(-1..CHUNK_SIZE as i32 + 1) {
            let global_x = shifted_x as i32 + x;
            let global_z = shifted_z as i32 + z;

            if global_x < WORLD_SIZE as i32 && global_z < WORLD_SIZE as i32 && x >= 0 && z >= 0 {
                let noise_value = noise.get_noise(global_x as f32 / scale, global_z as f32 / scale);

                let threshold = (spline.sample(noise_value).unwrap() as f32).floor() as usize;

                for y in shifted_y as i32 - 1..(shifted_y + CHUNK_SIZE) as i32 + 1 {
                    if y < WORLD_HEIGHT as i32 && y >= 0 {
                        if y < 50 {
                            self.voxels
                                [[global_x as usize, y as usize, global_z as usize]] = 5;
                            counter += 1;
                        }
                        match (y as usize).cmp(&threshold) {
                            Ordering::Less => {
                                if y == 0 {
                                    self.voxels
                                        [[global_x as usize, y as usize, global_z as usize]] = 2;
                                    counter += 1;
                                } else if (threshold - y as usize) == 1 {
                                    self.voxels
                                        [[global_x as usize, y as usize, global_z as usize]] = 4;
                                    counter += 1;
                                } else {
                                    self.voxels
                                        [[global_x as usize, y as usize, global_z as usize]] = 1;
                                    counter += 1;
                                }
                            }
                            Ordering::Greater => (),
                            Ordering::Equal => {
                                self.voxels[[global_x as usize, y as usize, global_z as usize]] = 3;
                                counter += 1;
                            }
                        }
                    }
                }
            }
        }
     counter == CHUNK_SIZE*CHUNK_SIZE*CHUNK_SIZE
    }
}
