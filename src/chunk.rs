use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use itertools::Itertools;
use std::cmp::{Ord, Ordering};

use super::world;
use super::voxel_data;
use super::block_types;

pub fn spawn_chunk(commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
        create_mesh(commands, meshes, materials, asset_server, populate_voxel_map());

}

fn populate_voxel_map() -> world::VoxelMap {

    let mut voxel_map = world::VoxelMap{
        voxels: [[[0; voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH]; voxel_data::CHUNK_HEIGHT]; voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH]
    };

    let noise = Perlin::new();
    let scale = 20.;

    for (x, z) in (0..voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH).cartesian_product(0..voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH) {
            for y in 0..voxel_data::CHUNK_HEIGHT {
                let threshold = (voxel_data::CHUNK_HEIGHT as f64 * (noise.get([x as f64 / scale, z as f64 / scale]) + 1.)/2.).floor() as usize;
                match y.cmp(&threshold) {
                    Ordering::Less =>
                        if y == 0 {
                            voxel_map.voxels[x][y][z] = 2;
                        }
                        else if (threshold - y) == 1 {
                            voxel_map.voxels[x][y][z] = 4;
                        }
                        else {
                            voxel_map.voxels[x][y][z] = 1;
                        },
                    Ordering::Greater => (),
                    Ordering::Equal => voxel_map.voxels[x][y][z] = 3,
                }
            }
    }
    voxel_map
}

fn create_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    voxel_map: world::VoxelMap,
    ){

    let mut chunk_pos = Vec3::new(0.0, 0.0, 0.0);
    let texture_handle: Handle<Image> = asset_server.load("texture_atlas.png");

    for (x, z) in (0..voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH).step_by(voxel_data::CHUNK_WIDTH)
        .cartesian_product(0..voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH).step_by(voxel_data::CHUNK_WIDTH) {
        chunk_pos.x = x as f32;
        chunk_pos.z = z as f32;

        let mesh_handle = meshes.add(voxel_data::create_voxel(chunk_pos, voxel_map));
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

pub fn check_voxel(pos: Vec3, voxel_map: world::VoxelMap) -> bool {
    let x:i32  = pos.x.floor() as i32;
    let y:i32  = pos.y.floor() as i32;
    let z:i32  = pos.z.floor() as i32;

    if  x < 0 || x as usize > voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH - 1
        || y < 0 ||  y as usize > voxel_data::CHUNK_HEIGHT - 1 || z < 0
        || z as usize > voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH - 1 {
        return false;
    }

    block_types::BLOCKTYPES[voxel_map.voxels[x as usize][y as usize][z as usize] as usize].is_solid
}
