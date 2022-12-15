use crate::chunk::Chunk;
use crate::voxel_data::{
    CHUNK_SIZE, RENDER_DISTANCE, WORLD_HEIGHT_IN_CHUNKS, WORLD_SIZE_IN_CHUNKS,
};
use crate::voxel_map::VoxelMap;
use bevy::prelude::*;
use itertools::iproduct;
use ndarray::Array3;

pub const WORLD_SIZE: usize = WORLD_SIZE_IN_CHUNKS * CHUNK_SIZE;
pub const WORLD_HEIGHT: usize = WORLD_HEIGHT_IN_CHUNKS * CHUNK_SIZE;
pub const TEXTURE_ATLAS_SIZE_IN_BLOCKS: u8 = 16;
pub const NORMALIZED_BLOCK_TEXTURE_SIZE: f32 = 1.0 / TEXTURE_ATLAS_SIZE_IN_BLOCKS as f32;

pub fn spawn_world(mut chunk_queue: ResMut<ChunkToGenerateQueue>) {
    // spawn chunks in spiral starting from 0.0 https://stackoverflow.com/a/398302
    let render_square = (2 * RENDER_DISTANCE).pow(2);
    let mut x = 0;
    let mut z = 0;
    let mut dx = 0;
    let mut dz = -1;
    let mut chunks = Vec::new();

    for _ in 0..render_square {
        if x > -1 * RENDER_DISTANCE as isize
            && x <= RENDER_DISTANCE as isize
            && z > -1 * RENDER_DISTANCE as isize
            && x <= RENDER_DISTANCE as isize
        {
            chunks.push((x, z));
        }

        if x == z || (x < 0 && x == -z) || (x > 0 && x == 1 - z) {
            (dx, dz) = (-dz, dx);
        }
        (x, z) = (x + dx, z + dz);
    }

    chunks.reverse();

    for (x, z) in chunks {
        for y in 0..WORLD_HEIGHT_IN_CHUNKS {
            let chunk_pos = ChunkCoord {
                x: x as i32,
                y: y as i32,
                z: z as i32,
            };
            chunk_queue.0.push(chunk_pos);
        }
    }
}

pub fn generate_chunk(
    mut chunk_to_generate_queue: ResMut<ChunkToGenerateQueue>,
    mut chunk_to_spawn_queue: ResMut<ChunkToSpawnQueue>,
    mut voxel_map: ResMut<VoxelMap>,
    chunk_map: ResMut<ChunkMap>,
    mut active_chunks: ResMut<ActiveChunks>,
    mut generated_chunks: ResMut<GeneratedChunks>,
) {
    if chunk_to_generate_queue.0.len() > 0 {
        let chunk_pos = chunk_to_generate_queue.0.pop().unwrap();
        if chunk_map.0[[
            (chunk_pos.x + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
            chunk_pos.y as usize,
            (chunk_pos.z + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
        ]] == None
        {
            let generated_chunk = &mut generated_chunks.chunks
                [(chunk_pos.x + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize][chunk_pos.y as usize]
                [(chunk_pos.z + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize];
            if !generated_chunk.0 {
                (*generated_chunk).1 = voxel_map.populate_voxel_map(chunk_pos);
            }

            chunk_to_spawn_queue.0.push((chunk_pos, generated_chunk.1));
            active_chunks.0.push(chunk_pos);
            (*generated_chunk).0 = true;
        }
    }
}

pub fn spawn_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut voxel_map: ResMut<VoxelMap>,
    mut chunk_map: ResMut<ChunkMap>,
    mut chunk_to_spawn_queue: ResMut<ChunkToSpawnQueue>,
) {
    while let Some((chunk_pos, is_full)) = chunk_to_spawn_queue.0.pop() {
        let _span = info_span!("Chunk spawn").entered();
        Chunk::new(
            &chunk_pos,
            &mut commands,
            &mut meshes,
            &mut materials,
            &asset_server,
            &mut voxel_map,
            &mut chunk_map,
            is_full,
        );
    }
}

pub fn check_render_distance(
    query: Query<(&GlobalTransform, With<super::Player>)>,
    mut commands: Commands,
    mut chunk_map: ResMut<ChunkMap>,
    mut chunk_queue: ResMut<ChunkToGenerateQueue>,
    mut active_chunks: ResMut<ActiveChunks>,
    mut player_last_chunk: ResMut<PlayerLastChunk>,
) {
    let player_pos = query.single().0.translation();
    let chunk_pos = get_chunk_from_player_pos(player_pos);

    if !chunk_pos.equals2d(player_last_chunk.0) {
        for (x, y, z) in iproduct!(
            (chunk_pos.x - RENDER_DISTANCE as i32..chunk_pos.x + RENDER_DISTANCE as i32),
            (0..WORLD_HEIGHT_IN_CHUNKS as i32),
            (chunk_pos.z - RENDER_DISTANCE as i32..chunk_pos.z + RENDER_DISTANCE as i32)
        ) {
            if is_chunk_in_world(&ChunkCoord { x, y, z }) {
                if chunk_map.0[[
                    (x + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
                    y as usize,
                    (z + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
                ]] == None
                {
                    chunk_queue.0.push(ChunkCoord { x, y, z });
                }
            }
        }

        for i in (0..active_chunks.0.len()).rev() {
            let chunk_coord = active_chunks.0[i];

            if chunk_coord.x < chunk_pos.x - RENDER_DISTANCE as i32
                || chunk_coord.x > chunk_pos.x + RENDER_DISTANCE as i32
                || chunk_coord.z < chunk_pos.z - RENDER_DISTANCE as i32
                || chunk_coord.z > chunk_pos.z + RENDER_DISTANCE as i32
            {
                let chunk = chunk_map.0[[
                    (chunk_coord.x + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
                    chunk_coord.y as usize,
                    (chunk_coord.z + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
                ]];

                if chunk.is_some() {
                    commands.entity(chunk.unwrap()).despawn_recursive();
                    active_chunks.0.swap_remove(i);
                    chunk_map.0[[
                        (chunk_coord.x + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
                        chunk_coord.y as usize,
                        (chunk_coord.z + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
                    ]] = None;
                }
            }
        }
        player_last_chunk.0 = chunk_pos;
    }
}

fn get_chunk_from_player_pos(mut pos: Vec3) -> ChunkCoord {
    pos.x = (pos.x / CHUNK_SIZE as f32).floor();
    pos.y = (pos.y / CHUNK_SIZE as f32).floor();
    pos.z = (pos.z / CHUNK_SIZE as f32).floor();

    ChunkCoord {
        x: pos.x as i32,
        y: pos.y as i32,
        z: pos.z as i32,
    }
}

fn is_chunk_in_world(chunk_pos: &ChunkCoord) -> bool {
    return chunk_pos.x + WORLD_SIZE_IN_CHUNKS as i32 / 2 >= 0
        && chunk_pos.x + WORLD_SIZE_IN_CHUNKS as i32 / 2 < WORLD_SIZE_IN_CHUNKS as i32
        && chunk_pos.y >= 0
        && chunk_pos.y < WORLD_HEIGHT_IN_CHUNKS as i32
        && chunk_pos.z + WORLD_SIZE_IN_CHUNKS as i32 / 2 >= 0
        && chunk_pos.z + WORLD_SIZE_IN_CHUNKS as i32 / 2 < WORLD_SIZE_IN_CHUNKS as i32;
}

#[derive(Clone, Debug)]
pub struct ActiveChunks(Vec<ChunkCoord>);
pub struct PlayerLastChunk(ChunkCoord);
pub struct ChunkToGenerateQueue(pub Vec<ChunkCoord>);
pub struct ChunkToSpawnQueue(pub Vec<(ChunkCoord, bool)>);

pub struct GeneratedChunks {
    pub chunks:
        [[[(bool, bool); WORLD_SIZE_IN_CHUNKS]; WORLD_HEIGHT_IN_CHUNKS]; WORLD_SIZE_IN_CHUNKS],
}

#[derive(Clone, Copy, Debug)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Default, Debug)]
pub struct ChunkMap(pub Array3<Option<Entity>>);

impl PlayerLastChunk {
    pub fn new() -> Self {
        PlayerLastChunk(ChunkCoord { x: 0, y: 0, z: 0 })
    }
}

impl ActiveChunks {
    pub fn new() -> Self {
        ActiveChunks(Vec::new())
    }
}

impl ChunkCoord {
    pub fn equals2d(self, other: ChunkCoord) -> bool {
        return other.x == self.x && other.z == self.z;
    }
}

impl ChunkMap {
    pub fn new() -> Self {
        ChunkMap(Array3::<Option<Entity>>::from_elem(
            (
                WORLD_SIZE_IN_CHUNKS,
                WORLD_HEIGHT_IN_CHUNKS,
                WORLD_SIZE_IN_CHUNKS,
            ),
            None,
        ))
    }
}
