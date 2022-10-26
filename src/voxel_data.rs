use bevy::prelude::{Vec3, Vec2, Mesh};
use bevy::render::mesh::{self, PrimitiveTopology};
use itertools::Itertools;

use super::world;
use super::chunk;
use super::block_types;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 32;
pub const RENDER_DISTANCE: usize = 4;

pub const VERTICES: [[Vec3; 4]; 6] = [
        [
        Vec3::new(0.0, 0.0, 0.0), // front face
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 1.0),
        Vec3::new(0.0, 0.0, 1.0)
        ],[
        Vec3::new(1.0, 0.0, 1.0), //back face
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        ],[
        Vec3::new(0.0, 1.0, 0.0), // top face
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(0.0, 1.0, 1.0),
        ],[
        Vec3::new(1.0, 0.0, 0.0), //bottom face
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 1.0),
        ],[
        Vec3::new(0.0, 0.0, 1.0), // right face
        Vec3::new(0.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 0.0, 1.0),
        ],[
        Vec3::new(1.0, 0.0, 0.0), // left face
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        ]
    ];

pub const FACE_CHECKS: [Vec3; 6] = [
        Vec3::new(-1.0, 0.0, 0.0),  // front face
        Vec3::new(1.0, 0.0, 0.0), // back face
        Vec3::new(0.0, 1.0, 0.0),  // top face
        Vec3::new(0.0, -1.0, 0.0), // bottom face
        Vec3::new(0.0, 0.0, 1.0),  // right face
        Vec3::new(0.0, 0.0, -1.0), // left face
];

pub const INDICES: [u32; 6] = [
        0, 2, 1, 0, 3, 2, // front face
    ];

pub const NORMALS: [Vec3; 6] = [
        Vec3::new(-1.0, 0.0, 0.0),   // front face
        Vec3::new(1.0, 0.0, 0.0),  // back face
        Vec3::new(0.0, 1.0, 0.0),   // top face
        Vec3::new(0.0, -1.0, 0.0),  // bottom face
        Vec3::new(0.0, 0.0, -1.0),  // right face
        Vec3::new(0.0, 0.0, 1.0),   // left face
];

pub fn create_voxel(chunk_pos: Vec3, voxel_map: world::VoxelMap) -> Mesh {
        let mut positions = Vec::new();
        let mut indices = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut index: u32 = 0;

        for (x, z) in (0..CHUNK_WIDTH).cartesian_product(0..CHUNK_WIDTH) {
                for y in 0..CHUNK_HEIGHT {

                        let pos = Vec3::new(x as f32, y as f32, z as f32);

                        if chunk::check_voxel(chunk_pos + pos, voxel_map) {
                                let blocktype = &block_types::BLOCKTYPES[voxel_map.voxels[x + chunk_pos.x as usize][y + chunk_pos.y as usize][z + chunk_pos.z as usize] as usize];
                                for i in 0..6{
                                        if !(chunk::check_voxel(chunk_pos + pos + FACE_CHECKS[i], voxel_map)) {
                                                for position in VERTICES[i].iter() {
                                                        positions.push((*position + pos).to_array());
                                                        normals.push(NORMALS[i].to_array());
                                                }
                                                for triangle_index in INDICES.iter() {
                                                        indices.push(*triangle_index+index);
                                                }
                                                for uv in add_texture(blocktype.texture_id.unwrap()[i]){
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

fn add_texture(texture_id: u32) -> [Vec2; 4]{
        let mut y = (texture_id/world::TEXTURE_ATLAS_SIZE_IN_BLOCKS as u32) as f32;
        let mut x = (texture_id%world::TEXTURE_ATLAS_SIZE_IN_BLOCKS as u32) as f32;

        x *= world::NORMALIZED_BLOCK_TEXTURE_SIZE;
        y *= world::NORMALIZED_BLOCK_TEXTURE_SIZE;

        [Vec2::new(x, y + world::NORMALIZED_BLOCK_TEXTURE_SIZE),
        Vec2::new(x, y),
        Vec2::new(x + world::NORMALIZED_BLOCK_TEXTURE_SIZE, y),
        Vec2::new(x + world::NORMALIZED_BLOCK_TEXTURE_SIZE, y + world::NORMALIZED_BLOCK_TEXTURE_SIZE)]
}
