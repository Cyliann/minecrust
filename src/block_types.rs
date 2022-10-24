pub struct BlockType {
    pub name: &'static str,
    pub is_solid: bool
}

pub const BLOCKTYPES: [BlockType; 2] = [
    BlockType{name: "air", is_solid: false},
    BlockType{name: "stone", is_solid: true}
];