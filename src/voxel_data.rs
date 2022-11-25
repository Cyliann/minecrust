use crate::world::{ChunkCoord, VoxelMap, WORLD_SIZE};
use bevy::prelude::{Mesh, ResMut, Vec2, Vec3};
use bevy::render::mesh::{self, PrimitiveTopology};
use itertools::Itertools;

use super::block_types;
use super::world;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 32;
pub const WORLD_SIZE_IN_CHUNKS: usize = 16;
pub const RENDER_DISTANCE: usize = 4;

pub const VERTICES: [[Vec3; 4]; 6] = [
    [
        Vec3::new(0.0, 0.0, 0.0), // front face
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 1.0),
        Vec3::new(0.0, 0.0, 1.0),
    ],
    [
        Vec3::new(1.0, 0.0, 1.0), //back face
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
    ],
    [
        Vec3::new(0.0, 1.0, 0.0), // top face
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(0.0, 1.0, 1.0),
    ],
    [
        Vec3::new(1.0, 0.0, 0.0), //bottom face
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 1.0),
    ],
    [
        Vec3::new(0.0, 0.0, 1.0), // right face
        Vec3::new(0.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 0.0, 1.0),
    ],
    [
        Vec3::new(1.0, 0.0, 0.0), // left face
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    ],
];

pub const FACE_CHECKS: [Vec3; 6] = [
    Vec3::new(-1.0, 0.0, 0.0), // front face
    Vec3::new(1.0, 0.0, 0.0),  // back face
    Vec3::new(0.0, 1.0, 0.0),  // top face
    Vec3::new(0.0, -1.0, 0.0), // bottom face
    Vec3::new(0.0, 0.0, 1.0),  // right face
    Vec3::new(0.0, 0.0, -1.0), // left face
];

pub const INDICES: [u32; 6] = [
    0, 2, 1, 0, 3, 2, // front face
];

pub const NORMALS: [Vec3; 6] = [
    Vec3::new(-1.0, 0.0, 0.0), // front face
    Vec3::new(1.0, 0.0, 0.0),  // back face
    Vec3::new(0.0, 1.0, 0.0),  // top face
    Vec3::new(0.0, -1.0, 0.0), // bottom face
    Vec3::new(0.0, 0.0, -1.0), // right face
    Vec3::new(0.0, 0.0, 1.0),  // left face
];

pub fn create_mesh(chunk_pos: &ChunkCoord, voxel_map: &mut ResMut<VoxelMap>) -> Mesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut index: u32 = 0;
    let chunk_pos_in_blocks = Vec3::new(
        (chunk_pos.x * CHUNK_WIDTH as i32 + (WORLD_SIZE/2) as i32) as f32,
        0.0,
        (chunk_pos.z * CHUNK_WIDTH as i32 + (WORLD_SIZE/2) as i32) as f32,
    );

    for (x, z) in (0..CHUNK_WIDTH).cartesian_product(0..CHUNK_WIDTH) {
        for y in 0..CHUNK_HEIGHT {
            let pos = Vec3::new(x as f32, y as f32, z as f32);

            if check_voxel(chunk_pos_in_blocks + pos, voxel_map) {
                let block_type = &block_types::BLOCKTYPES[voxel_map.voxels[[
                    x + chunk_pos_in_blocks.x as usize,
                    y,
                    z + chunk_pos_in_blocks.z as usize,
                ]] as usize];

                for i in 0..6 {
                    if !(check_voxel(
                        Vec3::new(
                            chunk_pos_in_blocks.x as f32,
                            0.0,
                            chunk_pos_in_blocks.z as f32,
                        ) + pos
                            + FACE_CHECKS[i],
                        voxel_map,
                    )) {
                        for position in VERTICES[i].iter() {
                            positions.push((*position + pos).to_array());
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
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(mesh::Indices::U32(indices)));
    mesh
}

pub fn check_voxel(pos: Vec3, voxel_map: &mut ResMut<VoxelMap>) -> bool {
    let x = pos.x as i32;
    let y = pos.y as i32;
    let z = pos.z as i32;

    if x > WORLD_SIZE as i32 - 1
        || x < 0
        || y > CHUNK_HEIGHT as i32 - 1
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
