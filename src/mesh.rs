use crate::voxel_data::{CHUNK_SIZE, FACE_CHECKS, INDICES, NORMALS, VERTICES};
use crate::voxel_map;
use crate::world::{ChunkCoord, WORLD_HEIGHT, WORLD_SIZE};
use bevy::log::info_span;
use bevy::prelude::{Mesh, ResMut, Vec2, Vec3};
use bevy::render::mesh::{self, PrimitiveTopology};
use itertools::iproduct;

use super::block_types;
use super::world;

pub fn create_mesh(chunk_pos: &ChunkCoord, voxel_map: &mut ResMut<voxel_map::VoxelMap>) -> Mesh {
    let _span = info_span!("Create mesh").entered();
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut index: u32 = 0;
    let shifted_global_x = chunk_pos.x * CHUNK_SIZE as i32 + (WORLD_SIZE / 2) as i32;
    let shifted_global_y = chunk_pos.y * CHUNK_SIZE as i32;
    let shifted_global_z = chunk_pos.z * CHUNK_SIZE as i32 + (WORLD_SIZE / 2) as i32;

    for (x, y, z) in iproduct!((0..CHUNK_SIZE), (0..CHUNK_SIZE), (0..CHUNK_SIZE)) {
        if check_voxel(
            shifted_global_x + x as i32,
            shifted_global_y + y as i32,
            shifted_global_z + z as i32,
            voxel_map,
        ) {
            let block_type = &block_types::BLOCKTYPES[voxel_map.voxels[[
                (x as i32 + shifted_global_x) as usize,
                (y as i32 + shifted_global_y) as usize,
                (z as i32 + shifted_global_z) as usize,
            ]] as usize];

            for i in 0..6 {
                let face_check = FACE_CHECKS[i];

                if !check_voxel(
                    shifted_global_x + face_check.x as i32 + x as i32,
                    shifted_global_y + face_check.y as i32 + y as i32,
                    shifted_global_z + face_check.z as i32 + z as i32,
                    voxel_map,
                ) {
                    for position in VERTICES[i].iter() {
                        positions
                            .push((*position + Vec3::new(x as f32, y as f32, z as f32)).to_array());
                        normals.push(NORMALS[i].to_array());
                    }
                    for triangle_index in INDICES.iter() {
                        indices.push(*triangle_index + index);
                    }
                    for uv in add_texture(block_type.texture_id.unwrap()[i]) {
                        uvs.push(uv.to_array());
                    }
                    index += 4;
                }
            }
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(mesh::Indices::U32(indices)));
    mesh
}

pub fn check_voxel(x: i32, y: i32, z: i32, voxel_map: &mut ResMut<voxel_map::VoxelMap>) -> bool {
    if x > WORLD_SIZE as i32 - 1
        || x < 0
        || y > WORLD_HEIGHT as i32 - 1
        || y < 0
        || z > WORLD_SIZE as i32 - 1
        || z < 0
    {
        return false;
    }

    block_types::BLOCKTYPES[voxel_map.voxels[[x as usize, y as usize, z as usize]] as usize]
        .is_solid
}

fn add_texture(texture_id: u32) -> [Vec2; 4] {
    let mut y = (texture_id / world::TEXTURE_ATLAS_SIZE_IN_BLOCKS as u32) as f32;
    let mut x = (texture_id % world::TEXTURE_ATLAS_SIZE_IN_BLOCKS as u32) as f32;

    x *= world::NORMALIZED_BLOCK_TEXTURE_SIZE;
    y *= world::NORMALIZED_BLOCK_TEXTURE_SIZE;

    [
        Vec2::new(x, y + world::NORMALIZED_BLOCK_TEXTURE_SIZE),
        Vec2::new(x, y),
        Vec2::new(x + world::NORMALIZED_BLOCK_TEXTURE_SIZE, y),
        Vec2::new(
            x + world::NORMALIZED_BLOCK_TEXTURE_SIZE,
            y + world::NORMALIZED_BLOCK_TEXTURE_SIZE,
        ),
    ]
}
