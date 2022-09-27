use bevy::{prelude::*, render::mesh::{self, PrimitiveTopology}, pbr::wireframe::Wireframe};
use noise::{NoiseFn, Perlin};
use itertools::Itertools;

use super::voxel_data;

#[derive(Copy, Clone, Debug)]
pub struct VoxelMap {
    voxels: [[[bool; voxel_data::RENDER_WIDTH*voxel_data::CHUNK_WIDTH]; voxel_data::CHUNK_HEIGHT]; voxel_data::RENDER_WIDTH*voxel_data::CHUNK_WIDTH],
}

pub fn spawn_chunk(commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>
) {
        create_mesh(commands, meshes, materials, populate_voxel_map());

}

fn populate_voxel_map() -> VoxelMap {

    let mut voxel_map = VoxelMap{
        voxels: [[[false; voxel_data::RENDER_WIDTH*voxel_data::CHUNK_WIDTH]; voxel_data::CHUNK_HEIGHT]; voxel_data::RENDER_WIDTH*voxel_data::CHUNK_WIDTH]
    };

    let noise = Perlin::new();
    let scale = 20.;

    for (x, z) in (0..voxel_data::RENDER_WIDTH*voxel_data::CHUNK_WIDTH).cartesian_product(0..voxel_data::RENDER_WIDTH*voxel_data::CHUNK_WIDTH) {
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

    if  x < 0 || x as usize > voxel_data::RENDER_WIDTH*voxel_data::CHUNK_WIDTH - 1  || y < 0 ||  y as usize > voxel_data::CHUNK_HEIGHT - 1 || z < 0 || z as usize > voxel_data::RENDER_WIDTH*voxel_data::CHUNK_WIDTH - 1 {
         return false;
    }

    voxel_map.voxels[x as usize][y as usize][z as usize]
}

fn create_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    voxel_map: VoxelMap,
    ){
    // let mesh = create_cube(); //Mesh::from(shape::Cube { size: 1.});
    // let mesh_handle = meshes.add(mesh);
    let mut chunk_pos = Vec3::new(0.0, 0.0, 0.0);

    for (x, z) in (0..voxel_data::RENDER_WIDTH*voxel_data::CHUNK_WIDTH).step_by(voxel_data::CHUNK_WIDTH)
        .cartesian_product(0..voxel_data::RENDER_WIDTH*voxel_data::CHUNK_WIDTH).step_by(voxel_data::CHUNK_WIDTH) {
        chunk_pos.x = x as f32;
        chunk_pos.z = z as f32;
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 0.0 })),
                transform: Transform::from_translation(chunk_pos),
                ..Default::default()
            })
            .with_children(|parent| {
                for (x, z) in (0..voxel_data::CHUNK_WIDTH).cartesian_product(0..voxel_data::CHUNK_WIDTH) {
                    for y in 0..voxel_data::CHUNK_HEIGHT {
                        let pos = Vec3::new(x as f32, y as f32, z as f32);
                            let mesh_handle = meshes.add(create_cube(pos+chunk_pos, voxel_map));
                            let material_handle = materials.add(StandardMaterial {
                                base_color: Color::rgb(0.7, 0.8, 0.9, ).into(),
                                perceptual_roughness: 1.,
                                ..default()
                            });
                            parent
                                .spawn_bundle(PbrBundle {
                                    mesh: mesh_handle,
                                    material: material_handle,
                                    transform: Transform::from_translation(pos),
                                    ..default()
                                })
                                .insert(Name::new(format!("{}, {},{}", x, y, z)))
                                .insert(Wireframe);
                    }
                }
            })
            .insert(Name::new("Chunk"));
    }
}

fn create_cube(pos: Vec3, voxel_map:VoxelMap) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();

            if check_voxel(pos, voxel_map) {
                for position in voxel_data::VERTS.iter() {
                    positions.push((position).to_array());
                    normals.push([0.0, 1.0, 0.0])
                }

                for (i, face) in voxel_data::INDICES.iter().enumerate() {
                    if !(check_voxel(pos + voxel_data::FACE_CHECKS[i], voxel_map)) {
                        for triangle_index in face.iter() {
                            indices.push(*triangle_index);
                        }
                    }
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(mesh::Indices::U32(indices.to_vec())));
    mesh
}

