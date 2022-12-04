use crate::voxel_data::{self, CHUNK_SIZE, WORLD_SIZE_IN_CHUNKS};
use crate::voxel_map;
use crate::world::{ChunkCoord, ChunkMap};
use bevy::prelude::*;

pub struct Chunk {
    pub position: ChunkCoord,
}

impl Chunk {
    pub fn new(
        chunk_pos: &ChunkCoord,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        asset_server: &Res<AssetServer>,
        voxel_map: &mut ResMut<voxel_map::VoxelMap>,
        chunk_map: &mut ResMut<ChunkMap>,
        is_chunk_empty: bool,
    ) -> Self {
        if !is_chunk_empty {
            let mesh_handle = meshes.add(voxel_data::create_mesh(chunk_pos, voxel_map));
            let texture_handle: Handle<Image> = asset_server.load("texture_atlas.png");

            let material_handle = materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle.clone()),
                perceptual_roughness: 1.0,
                metallic: 0.0,
                reflectance: 0.1,
                ..default()
            });

            let _span = info_span!("Spawn mesh").entered();
            chunk_map.0[[
                (chunk_pos.x + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
                chunk_pos.y as usize,
                (chunk_pos.z + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
            ]] = Some(
                commands
                    .spawn_bundle(MaterialMeshBundle {
                        mesh: mesh_handle,
                        material: material_handle,
                        transform: Transform::from_xyz(
                            (chunk_pos.x * CHUNK_SIZE as i32) as f32,
                            (chunk_pos.y * CHUNK_SIZE as i32) as f32,
                            (chunk_pos.z * CHUNK_SIZE as i32) as f32,
                        ),
                        ..Default::default()
                    })
                    .insert(Name::new(format!(
                        "Chunk ({}, {}, {})",
                        chunk_pos.x, chunk_pos.y, chunk_pos.z
                    )))
                    .id(),
            );
        }

        Chunk {
            position: *chunk_pos,
        }
    }
}
