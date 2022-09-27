use bevy::prelude::Vec3;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 16;
pub const RENDER_WIDTH: usize = 4;

pub const VERTS: [Vec3; 8] = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 1.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 0.0, 1.0),
    ];

pub const FACE_CHECKS: [Vec3; 6] = [
        Vec3::new(-1.0, 0.0, 0.0),  // front face
        Vec3::new(1.0, 0.0, 0.0), // back face
        Vec3::new(0.0, 1.0, 0.0),  // top face
        Vec3::new(0.0, -1.0, 0.0), // bottom face
        Vec3::new(0.0, 0.0, 1.0),  // right face
        Vec3::new(0.0, 0.0, -1.0), // left face
];

pub const INDICES: [[u32; 6]; 6] = [
        [0, 2, 1, 0, 3, 2], // front face
        [7, 5, 6, 7, 4, 5], // back face
        [1, 6, 5, 1, 2, 6], // top face
        [3, 4, 7, 3, 0, 4], // bottom face
        [3, 6, 2, 3, 7, 6], // right face
        [4, 1, 5, 4, 0, 1], // left face
    ];

