use crate::chunk::Chunk;
use crate::voxel_data::{
    CHUNK_SIZE, RENDER_DISTANCE, WORLD_HEIGHT_IN_CHUNKS, WORLD_SIZE_IN_CHUNKS,
};
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

    for (x, z) in chunks.iter().rev() {
        for y in 0..WORLD_HEIGHT_IN_CHUNKS {
            let chunk_pos = ChunkCoord {
                x: *x as i32,
                y: y as i32,
                z: *z as i32,
            };
            chunk_queue.0.push(chunk_pos);
        }
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
    let player_chunk_pos = get_chunk_from_player_pos(player_pos);

    if !player_chunk_pos.equals2d(player_last_chunk.0) {
        for (x, y, z) in iproduct!(
            (player_chunk_pos.x - RENDER_DISTANCE as i32..player_chunk_pos.x + RENDER_DISTANCE as i32),
            (0..WORLD_HEIGHT_IN_CHUNKS as i32),
            (player_chunk_pos.z - RENDER_DISTANCE as i32..player_chunk_pos.z + RENDER_DISTANCE as i32)
        ) {
            if is_chunk_in_world(&ChunkCoord { x, y, z }) {
                if chunk_map.0[[
                    (x + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
                    y as usize,
                    (z + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
                ]].1 == None
                {
                    chunk_queue.0.push(ChunkCoord { x, y, z });
                }
            }
        }

        for i in (0..active_chunks.0.len()).rev() {
            let chunk_coord = active_chunks.0[i];

            if chunk_coord.x < player_chunk_pos.x - RENDER_DISTANCE as i32
                || chunk_coord.x > player_chunk_pos.x + RENDER_DISTANCE as i32
                || chunk_coord.z < player_chunk_pos.z - RENDER_DISTANCE as i32
                || chunk_coord.z > player_chunk_pos.z + RENDER_DISTANCE as i32
            {
                let chunk_entity = chunk_map.0[[
                    (chunk_coord.x + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
                    chunk_coord.y as usize,
                    (chunk_coord.z + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
                ]].1;

                if chunk_entity.is_some() {
                    commands.entity(chunk_entity.unwrap()).despawn_recursive();
                    active_chunks.0.swap_remove(i);
                    chunk_map.0[[
                        (chunk_coord.x + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
                        chunk_coord.y as usize,
                        (chunk_coord.z + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
                    ]].1 = None;
                }
            }
        }
        player_last_chunk.0 = player_chunk_pos;
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
pub struct ActiveChunks(pub Vec<ChunkCoord>);
pub struct PlayerLastChunk(ChunkCoord);
pub struct ChunkToGenerateQueue(pub Vec<ChunkCoord>);
pub struct ChunkToSpawnQueue(pub Vec<(ChunkCoord, bool)>);

#[derive(Clone, Copy, Debug)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Default, Debug)]
pub struct ChunkMap(pub Array3<(Option<Chunk>, Option<Entity>)>);

impl PlayerLastChunk {
    pub fn new() -> Self {
        PlayerLastChunk(ChunkCoord { x: 0, y: 0, z: 0 })
    }
}

impl ChunkCoord {
    pub fn equals2d(self, other: ChunkCoord) -> bool {
        return other.x == self.x && other.z == self.z;
    }
}

impl ChunkMap {
    pub fn new() -> Self {
        ChunkMap(Array3::<(Option<Chunk>, Option<Entity>)>::from_elem(
            (
                WORLD_SIZE_IN_CHUNKS,
                WORLD_HEIGHT_IN_CHUNKS,
                WORLD_SIZE_IN_CHUNKS,
            ),
            (None, None),
        ))
    }
}
