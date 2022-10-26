pub struct BlockType {
    pub name: &'static str,
    pub is_solid: bool,
    pub texture_id: Option<[u32; 6]>, //front, back, top, bottom, right, left
}

pub const BLOCKTYPES: [BlockType; 4] = [
    BlockType {
        name: "air",
        is_solid: false,
        texture_id: None,
    },
    BlockType {
        name: "stone",
        is_solid: true,
        texture_id: Some([0, 0, 0, 0, 0, 0]),
    },
    BlockType {
        name: "bedrock",
        is_solid: true,
        texture_id: Some([9, 9, 9, 9, 9, 9]),
    },
    BlockType {
        name: "grass",
        is_solid: true,
        texture_id: Some([2, 2, 7, 1, 2, 2]),
    },
];
