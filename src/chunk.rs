use bevy::{prelude::*, render::mesh::{self, PrimitiveTopology}, pbr::wireframe::Wireframe};
use bevy::asset::HandleId;
use bevy::render::render_resource::Texture;
use bevy_inspector_egui::egui::TextureHandle;
use noise::{NoiseFn, Perlin};
use itertools::Itertools;

use super::voxel_data;

#[derive(Copy, Clone, Debug)]
pub struct VoxelMap {
    voxels: [[[bool; voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH]; voxel_data::CHUNK_HEIGHT]; voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH],
}

pub fn spawn_chunk(commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
        create_mesh(commands, meshes, materials, asset_server, populate_voxel_map());

}

fn populate_voxel_map() -> VoxelMap {

    let mut voxel_map = VoxelMap{
        voxels: [[[false; voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH]; voxel_data::CHUNK_HEIGHT]; voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH]
    };

    let noise = Perlin::new();
    let scale = 20.;

    for (x, z) in (0..voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH).cartesian_product(0..voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH) {
            for y in 0..voxel_data::CHUNK_HEIGHT {
                voxel_map.voxels[x][y][z] = y < (voxel_data::CHUNK_HEIGHT as f64 * (noise.get([x as f64 / scale, z as f64 / scale]) + 1.)/2.).floor() as usize;
            }
    }
    voxel_map
}

fn check_voxel(pos: Vec3, voxel_map: VoxelMap) -> bool {
    let x:i32  = pos.x.floor() as i32;
    let y:i32  = pos.y.floor() as i32;
    let z:i32  = pos.z.floor() as i32;

    if  x < 0 || x as usize > voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH - 1
              || y < 0 ||  y as usize > voxel_data::CHUNK_HEIGHT - 1 || z < 0
              || z as usize > voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH - 1 {
         return false;
    }

    voxel_map.voxels[x as usize][y as usize][z as usize]
}

fn create_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    voxel_map: VoxelMap,
    ){

    let mut chunk_pos = Vec3::new(0.0, 0.0, 0.0);
    let texture_handle: Handle<Image> = asset_server.load("grass_side.png");

    for (x, z) in (0..voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH).step_by(voxel_data::CHUNK_WIDTH)
        .cartesian_product(0..voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH).step_by(voxel_data::CHUNK_WIDTH) {
        chunk_pos.x = x as f32;
        chunk_pos.z = z as f32;

        let mesh_handle = meshes.add(create_cube(chunk_pos, voxel_map));
        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            perceptual_roughness: 1.0,
            metallic: 0.0,
            reflectance: 0.1,
            ..default()
        });

        commands
            .spawn_bundle(MaterialMeshBundle {
                mesh: mesh_handle,
                material: material_handle,
                transform: Transform::from_translation(chunk_pos),
                ..Default::default()
            })
            .insert(Name::new(format!("Chunk {} {} {}", chunk_pos.x, chunk_pos.y, chunk_pos.z)));
    }
}

fn create_cube(chunk_pos: Vec3, voxel_map:VoxelMap) -> Mesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut index: u32 = 0;

    for (x, z) in (0..voxel_data::CHUNK_WIDTH).cartesian_product(0..voxel_data::CHUNK_WIDTH) {
        for y in 0..voxel_data::CHUNK_HEIGHT {

            let pos = Vec3::new(x as f32, y as f32, z as f32);

            if check_voxel(chunk_pos + pos, voxel_map) {
                for i in 0..6{
                    if !(check_voxel(chunk_pos + pos + voxel_data::FACE_CHECKS[i], voxel_map)) {
                        for (j, position) in voxel_data::VERTICES[i].iter().enumerate() {
                            positions.push((*position + pos).to_array());
                            normals.push(voxel_data::NORMALS[i].to_array());
                            uvs.push(voxel_data::UVS[j].to_array());
                        }
                        for triangle_index in voxel_data::INDICES.iter() {
                            indices.push(*triangle_index+index);
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

