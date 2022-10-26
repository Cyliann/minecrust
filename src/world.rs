use super::voxel_data;

#[derive(Copy, Clone, Debug)]
pub struct VoxelMap {
    pub voxels: [[[u8; voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH]; voxel_data::CHUNK_HEIGHT]; voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH],
}

pub const TEXTURE_ATLAS_SIZE_IN_BLOCKS: u8 = 4;
pub const NORMALIZED_BLOCK_TEXTURE_SIZE: f32 = 1.0/TEXTURE_ATLAS_SIZE_IN_BLOCKS as f32;
