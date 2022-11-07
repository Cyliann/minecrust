use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::*;
use super::world;
use super::voxel_data;
use super::block_types;

pub struct Chunk {
    pub position: Vec3,
}

impl Chunk {
    pub fn spawn_chunk(self,
                       commands: &mut Commands,
                       meshes: &mut ResMut<Assets<Mesh>>,
                       materials: &mut ResMut<Assets<StandardMaterial>>,
                       asset_server: &mut Res<AssetServer>,
                       voxel_map: world::VoxelMap,
    ) {
        let mesh_handle = meshes.add(voxel_data::create_voxel(self.position, voxel_map));

        let texture_handle: Handle<Image> = asset_server.load("texture_atlas.png");


        let chunk_pos = self.position;
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
            .insert(Wireframe);
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
