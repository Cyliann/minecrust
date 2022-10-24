use super::voxel_data;

#[derive(Copy, Clone, Debug)]
pub struct VoxelMap {
    pub voxels: [[[u8; voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH]; voxel_data::CHUNK_HEIGHT]; voxel_data::RENDER_DISTANCE *voxel_data::CHUNK_WIDTH],
}
