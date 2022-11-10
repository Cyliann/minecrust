use bevy::prelude::*;
use itertools::Itertools;
use crate::chunk;
use crate::voxel_data;
use noise::{NoiseFn, Perlin};
use std::cmp::{Ord, Ordering};
use bevy::utils::HashMap;

#[derive(Copy, Clone, Debug)]
pub struct VoxelMap {
    pub voxels: [[[u8; voxel_data::WORLD_SIZE_IN_CHUNKS *voxel_data::CHUNK_WIDTH]; voxel_data::CHUNK_HEIGHT]; voxel_data::WORLD_SIZE_IN_CHUNKS *voxel_data::CHUNK_WIDTH],
}

impl VoxelMap{
    fn populate_voxel_map(&mut self){

        let noise = Perlin::new();
        let scale = 20.;

        for (x, z) in (0..voxel_data::WORLD_SIZE_IN_CHUNKS *voxel_data::CHUNK_WIDTH).cartesian_product(0..voxel_data::WORLD_SIZE_IN_CHUNKS *voxel_data::CHUNK_WIDTH) {
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

    let mut voxel_map = VoxelMap {voxels: [[[0; voxel_data::WORLD_SIZE_IN_CHUNKS *voxel_data::CHUNK_WIDTH]; voxel_data::CHUNK_HEIGHT]; voxel_data::WORLD_SIZE_IN_CHUNKS *voxel_data::CHUNK_WIDTH]};
    voxel_map.populate_voxel_map();
    let mut chunk_pos_to_entity = HashMap::new();

    for (x, z) in (0..voxel_data::RENDER_DISTANCE).cartesian_product(0..voxel_data::RENDER_DISTANCE) {
        let chunk = chunk::Chunk {
            position: Vec3::new((x * voxel_data::CHUNK_WIDTH) as f32, 0.0, (z * voxel_data::CHUNK_WIDTH) as f32),
            entity_id: chunk::spawn_chunk(
                Vec3::new((x * voxel_data::CHUNK_WIDTH) as f32, 0.0, (z * voxel_data::CHUNK_WIDTH) as f32),
                &mut commands,
                &mut meshes,
                &mut materials,
                &mut asset_server,
                &voxel_map)
        };


        chunk_pos_to_entity.insert(gen_key(x as i32, z as i32), chunk.entity_id);
    }
}

fn gen_key(x: i32, z:i32) -> u32 {
    let a: u32 = (x + i16::MAX as i32) as u32;
    let b: u32 = (z + i16::MAX as i32 + 1) as u32;

    let key: u32 = if a >= b { (a * a + a + b).into() } else { (a + b * b).into() };
    key
}