use crate::voxel_data::{CHUNK_SIZE, WORLD_SIZE_IN_CHUNKS};
use crate::world::{ActiveChunks, ChunkCoord, ChunkMap, ChunkToGenerateQueue, ChunkToSpawnQueue};
use crate::mesh;
use bevy::prelude::*;
use crate::voxel_map::VoxelMap;

#[derive(Default)]
pub struct MaterialHandle(pub Handle<StandardMaterial>);

#[derive(Clone, Debug)]
pub struct Chunk {
    pub position: ChunkCoord,
    pub is_full: bool,
    pub mesh_handle: Option<Handle<Mesh>>,
}

pub fn generate_chunk(
    mut chunk_to_generate_queue: ResMut<ChunkToGenerateQueue>,
    mut chunk_to_spawn_queue: ResMut<ChunkToSpawnQueue>,
    mut voxel_map: ResMut<VoxelMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunk_map: ResMut<ChunkMap>,
    mut active_chunks: ResMut<ActiveChunks>,
) {
    while let Some(chunk_pos) = chunk_to_generate_queue.0.pop() {
        let is_full;
        let chunk= &mut chunk_map.0[[
            (chunk_pos.x + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
            chunk_pos.y as usize,
            (chunk_pos.z + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize]].0;


        if chunk.is_none()
        {
                is_full = voxel_map.populate_voxel_map(chunk_pos);
                let mesh_handle = Some(meshes.add(mesh::create_mesh(&chunk_pos, &mut voxel_map)));

                *chunk = Some(Chunk {
                    position: chunk_pos,
                    is_full,
                    mesh_handle,
                });
        } else {
            is_full = chunk.as_ref().unwrap().is_full;
        }
        chunk_to_spawn_queue.0.push((chunk_pos, is_full));
        active_chunks.0.push(chunk_pos);
    }
}

pub fn spawn_chunk(
    mut commands: Commands,
    mut chunk_map: ResMut<ChunkMap>,
    mut chunk_to_spawn_queue: ResMut<ChunkToSpawnQueue>,
    material_handle: Res<MaterialHandle>,
) {
    while let Some((chunk_pos, is_full)) = chunk_to_spawn_queue.0.pop() {
        let _span = info_span!("Chunk spawn").entered();
        if !is_full {
            let chunk = chunk_map.0[[
            (chunk_pos.x + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
            chunk_pos.y as usize,
            (chunk_pos.z + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize]].0.as_mut().unwrap();

            let mesh_handle = chunk.mesh_handle.clone().unwrap();

            let _span = info_span!("Spawn mesh").entered();
            chunk_map.0[[
                (chunk_pos.x + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
                chunk_pos.y as usize,
                (chunk_pos.z + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
            ]].1 = Some(
                commands
                    .spawn_bundle(MaterialMeshBundle {
                        mesh: mesh_handle,
                        material: material_handle.0.clone(),
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
    }
}

pub fn generate_material(
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut material_handle: ResMut<MaterialHandle>,
) {
    let texture_handle: Handle<Image> = asset_server.load("texture_atlas.png");

    material_handle.0 = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        perceptual_roughness: 1.0,
        metallic: 0.0,
        reflectance: 0.1,
        ..default()
    });
}
