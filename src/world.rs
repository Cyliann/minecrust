use crate::chunk::Chunk;
use crate::voxel_data::{CHUNK_WIDTH, RENDER_DISTANCE, WORLD_SIZE_IN_CHUNKS};
use crate::voxel_map;
use bevy::prelude::*;
use itertools::Itertools;
use ndarray::Array2;

pub const WORLD_SIZE: usize = WORLD_SIZE_IN_CHUNKS * CHUNK_WIDTH;
pub const TEXTURE_ATLAS_SIZE_IN_BLOCKS: u8 = 16;
pub const NORMALIZED_BLOCK_TEXTURE_SIZE: f32 = 1.0 / TEXTURE_ATLAS_SIZE_IN_BLOCKS as f32;

#[derive(Clone, Debug)]
pub struct ActiveChunks(Vec<ChunkCoord>);
pub struct SpawnChunkEvent(ChunkCoord);
pub struct PlayerLastChunk(ChunkCoord);

pub struct GeneratedChunks {
    pub chunks: [[bool; WORLD_SIZE_IN_CHUNKS]; WORLD_SIZE_IN_CHUNKS],
}

#[derive(Clone, Copy, Debug)]
pub struct ChunkCoord {
    pub x: i32,
    pub z: i32,
}

#[derive(Default, Debug)]
pub struct ChunkMap(pub Array2<Option<Entity>>);

impl PlayerLastChunk {
    pub fn new() -> Self {
        PlayerLastChunk(ChunkCoord { x: 0, z: 0 })
    }
}

impl ActiveChunks {
    pub fn new() -> Self {
        ActiveChunks(Vec::new())
    }
}

impl ChunkCoord {
    pub fn equals(self, other: ChunkCoord) -> bool {
        return other.x == self.x && other.z == self.z;
    }
}

impl ChunkMap {
    pub fn new() -> Self {
        ChunkMap(Array2::<Option<Entity>>::from_elem(
            (WORLD_SIZE_IN_CHUNKS, WORLD_SIZE_IN_CHUNKS),
            None,
        ))
    }
}

pub fn spawn_world(mut ev_spawn_chunk: EventWriter<SpawnChunkEvent>) {
    for (x, z) in (RENDER_DISTANCE as isize * -1..RENDER_DISTANCE as isize)
        .cartesian_product(RENDER_DISTANCE as isize * -1..RENDER_DISTANCE as isize)
    {
        let chunk_pos = ChunkCoord {
            x: x as i32,
            z: z as i32,
        };
        ev_spawn_chunk.send(SpawnChunkEvent(chunk_pos))
    }
}

pub fn spawn_chunk(
    mut ev_spawn_chunk: EventReader<SpawnChunkEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut voxel_map: ResMut<voxel_map::VoxelMap>,
    mut chunk_map: ResMut<ChunkMap>,
    mut active_chunks: ResMut<ActiveChunks>,
    mut generated_chunks: ResMut<GeneratedChunks>,
) {
    for ev in ev_spawn_chunk.iter() {
        if chunk_map.0[[
            (&ev.0.x + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
            (&ev.0.z + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
        ]] == None
        {
            if !generated_chunks.chunks[(&ev.0.x + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize]
                [(&ev.0.z + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize]
            {
                voxel_map.populate_voxel_map(ev.0);
            }

            let _span = info_span!("Chunk spawn").entered();
            Chunk::new(
                &ev.0,
                &mut commands,
                &mut meshes,
                &mut materials,
                &asset_server,
                &mut voxel_map,
                &mut chunk_map,
            );
            active_chunks.0.push(ev.0);
            generated_chunks.chunks[(&ev.0.x + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize]
                [(&ev.0.z + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize] = true;
        }
    }
}

pub fn check_render_distance(
    query: Query<(&GlobalTransform, With<super::Player>)>,
    mut commands: Commands,
    mut chunk_map: ResMut<ChunkMap>,
    mut ev_spawn_chunk: EventWriter<SpawnChunkEvent>,
    mut active_chunks: ResMut<ActiveChunks>,
    mut player_last_chunk: ResMut<PlayerLastChunk>,
) {
    let player_pos = query.single().0.translation();
    let chunk_pos = get_chunk_from_player_pos(player_pos);

    if !chunk_pos.equals(player_last_chunk.0) {
        for (x, z) in (chunk_pos.x - RENDER_DISTANCE as i32..chunk_pos.x + RENDER_DISTANCE as i32)
            .cartesian_product(
                chunk_pos.z - RENDER_DISTANCE as i32..chunk_pos.z + RENDER_DISTANCE as i32,
            )
        {
            if is_chunk_in_world(&ChunkCoord { x, z }) {
                if chunk_map.0[[
                    (x + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
                    (z + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
                ]] == None
                {
                    ev_spawn_chunk.send(SpawnChunkEvent(ChunkCoord { x, z }))
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
                    (chunk_coord.z + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
                ]];

                if chunk.is_some() {
                    commands.entity(chunk.unwrap()).despawn_recursive();
                    active_chunks.0.swap_remove(i);
                    chunk_map.0[[
                        (chunk_coord.x + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
                        (chunk_coord.z + WORLD_SIZE_IN_CHUNKS as i32 / 2) as usize,
                    ]] = None;
                }
            }
        }
        player_last_chunk.0 = chunk_pos;
    }
}

fn get_chunk_from_player_pos(mut pos: Vec3) -> ChunkCoord {
    pos.x = (pos.x / CHUNK_WIDTH as f32).floor();
    pos.z = (pos.z / CHUNK_WIDTH as f32).floor();

    ChunkCoord {
        x: pos.x as i32,
        z: pos.z as i32,
    }
}

fn is_chunk_in_world(chunk_pos: &ChunkCoord) -> bool {
    return chunk_pos.x + WORLD_SIZE_IN_CHUNKS as i32 / 2 >= 0
        && chunk_pos.x + WORLD_SIZE_IN_CHUNKS as i32 / 2 < WORLD_SIZE_IN_CHUNKS as i32
        && chunk_pos.z + WORLD_SIZE_IN_CHUNKS as i32 / 2 >= 0
        && chunk_pos.z + WORLD_SIZE_IN_CHUNKS as i32 / 2 < WORLD_SIZE_IN_CHUNKS as i32;
}
