pub struct BlockType {
    pub name: &'static str,
    pub is_solid: bool,
    pub texture_id: Option<[u32; 6]>, //front, back, top, bottom, right, left
}

pub const BLOCKTYPES: [BlockType; 6] = [
    BlockType {
        name: "air",
        is_solid: false,
        texture_id: None,
    },
    BlockType {
        name: "stone",
        is_solid: true,
        texture_id: Some([1, 1, 1, 1, 1, 1]),
    },
    BlockType {
        name: "bedrock",
        is_solid: true,
        texture_id: Some([9, 9, 9, 9, 9, 9]),
    },
    BlockType {
        name: "grass",
        is_solid: true,
        texture_id: Some([3, 3, 0, 1, 2, 3]),
    },
    BlockType {
        name: "dirt",
        is_solid: true,
        texture_id: Some([2, 2, 2, 2, 2, 2]),
    },
    BlockType {
        name: "water",
        is_solid: true,
        texture_id: Some([207, 207, 207, 207, 207, 207]),
    },
];
