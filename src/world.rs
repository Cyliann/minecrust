use bevy::prelude::*;
use itertools::Itertools;
use crate::chunk;
use crate::voxel_data;
use noise::{NoiseFn, Perlin};
use std::cmp::{Ord, Ordering};

#[derive(Copy, Clone, Debug)]
pub struct VoxelMap {
    pub voxels: [[[u8; voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH]; voxel_data::CHUNK_HEIGHT]; voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH],
}

impl VoxelMap{
    fn populate_voxel_map(&mut self){

        // self.voxels = [[[0; voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH]; voxel_data::CHUNK_HEIGHT]; voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH];

        // print!("Voxel map");
        let noise = Perlin::new();
        let scale = 20.;

        for (x, z) in (0..voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH).cartesian_product(0..voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH) {
            for y in 0..voxel_data::CHUNK_HEIGHT {
                let threshold = (voxel_data::CHUNK_HEIGHT as f64 * (noise.get([x as f64 / scale, z as f64 / scale]) + 1.)/2.).floor() as usize;
                match y.cmp(&threshold) {
                    Ordering::Less =>
                        if y == 0 {
                            self.voxels[x][y][z] = 2;
                        }
                        else if (threshold - y) == 1 {
                            self.voxels[x][y][z] = 4;
                        }
                        else {
                            self.voxels[x][y][z] = 1;
                        },
                    Ordering::Greater => (),
                    Ordering::Equal => self.voxels[x][y][z] = 3,
                }
            }
        }
    }
}

pub const TEXTURE_ATLAS_SIZE_IN_BLOCKS: u8 = 4;
pub const NORMALIZED_BLOCK_TEXTURE_SIZE: f32 = 1.0/TEXTURE_ATLAS_SIZE_IN_BLOCKS as f32;

pub fn spawn_world(mut commands: Commands,
                   mut meshes: ResMut<Assets<Mesh>>,
                   mut materials: ResMut<Assets<StandardMaterial>>,
                   mut asset_server: Res<AssetServer>) {

    let mut voxel_map = VoxelMap {voxels: [[[0; voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH]; voxel_data::CHUNK_HEIGHT]; voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH]};
    voxel_map.populate_voxel_map();

    for (x, z) in (0..voxel_data::RENDER_DISTANCE * voxel_data::CHUNK_WIDTH).step_by(voxel_data::CHUNK_WIDTH)
        .cartesian_product(0..voxel_data::RENDER_DISTANCE * voxel_data::CHUNK_WIDTH).step_by(voxel_data::CHUNK_WIDTH){

        let chunk = chunk::Chunk { position: Vec3::new(x as f32, 0.0, z as f32)};

        chunk.spawn_chunk(&mut commands,
                          &mut meshes,
                          &mut materials,
                          &mut asset_server,
                          voxel_map);
    }
    // print!("Voxel Map: {:?}", voxel_map);
}