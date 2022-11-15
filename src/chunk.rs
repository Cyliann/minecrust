use bevy::prelude::*;
use super::world;
use super::voxel_data;

pub struct Chunk {
    pub position: Vec3,
    pub entity_id: Entity,
}

pub fn spawn_chunk(
    pos: Vec3,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &mut Res<AssetServer>,
    voxel_map: &world::VoxelMap,
) -> Entity {

    let mesh_handle = meshes.add(voxel_data::create_mesh(pos, voxel_map));
    let texture_handle: Handle<Image> = asset_server.load("texture_atlas.png");
    let chunk_pos = pos;

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
        .insert(Name::new(format!("Chunk ({}, {}, {})", chunk_pos.x, chunk_pos.y, chunk_pos.z)))
        .id()
}

