use bevy::prelude::{Vec3, Vec2};

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 16;
pub const RENDER_DISTANCE: usize = 4;

pub const VERTICES: [[Vec3; 4]; 6] = [
        [
        Vec3::new(0.0, 0.0, 0.0), // front face
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 1.0),
        Vec3::new(0.0, 0.0, 1.0)
        ],[
        Vec3::new(1.0, 0.0, 1.0), //back face
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        ],[
        Vec3::new(0.0, 1.0, 0.0), // top face
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(0.0, 1.0, 1.0),
        ],[
        Vec3::new(1.0, 0.0, 0.0), //bottom face
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 1.0),
        ],[
        Vec3::new(0.0, 0.0, 1.0), // right face
        Vec3::new(0.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 0.0, 1.0),
        ],[
        Vec3::new(1.0, 0.0, 0.0), // left face
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        ]
    ];

pub const UVS: [Vec2; 4] = [
        Vec2::new(0.0, 1.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
];

pub const FACE_CHECKS: [Vec3; 6] = [
        Vec3::new(-1.0, 0.0, 0.0),  // front face
        Vec3::new(1.0, 0.0, 0.0), // back face
        Vec3::new(0.0, 1.0, 0.0),  // top face
        Vec3::new(0.0, -1.0, 0.0), // bottom face
        Vec3::new(0.0, 0.0, 1.0),  // right face
        Vec3::new(0.0, 0.0, -1.0), // left face
];

pub const INDICES: [u32; 6] = [
        0, 2, 1, 0, 3, 2, // front face
    ];

pub const NORMALS: [Vec3; 6] = [
        Vec3::new(-1.0, 0.0, 0.0),   // front face
        Vec3::new(1.0, 0.0, 0.0),  // back face
        Vec3::new(0.0, 1.0, 0.0),   // top face
        Vec3::new(0.0, -1.0, 0.0),  // bottom face
        Vec3::new(0.0, 0.0, -1.0),  // right face
        Vec3::new(0.0, 0.0, 1.0),   // left face
];

