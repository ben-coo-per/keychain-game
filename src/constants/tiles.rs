pub const TILE_INDICES: [u8; 16] = [
    12, // 0b0000: Empty (no corners filled)
    0,  // 0b0001: bottom-left corner filled
    13, // 0b0010: bottom-right corner filled
    3,  // 0b0011: bottom-right and bottom-left corners filled
    8,  // 0b0100: top-right corner filled
    14, // 0b0101: top-right and bottom-left corners filled
    1,  // 0b0110: top-right and bottom-right corners filled
    5,  // 0b0111: top-right, bottom-right, and bottom-left corners filled
    15, // 0b1000: top-left corner filled
    11, // 0b1001: top-left and bottom-left corners filled
    4,  // 0b1010: top-left and bottom-right corners filled
    2,  // 0b1011: top-left, bottom-right, and bottom-left corners filled
    9,  // 0b1100: top-left and top-right corners filled
    7,  // 0b1101: top-left, top-right, and bottom-left corners filled
    10, // 0b1110: top-left, top-right, and bottom-right corners filled
    6,  // 0b1111: all corners filled
];

pub fn get_tile_index_from_bitmap(bitmap: u8) -> u8 {
    TILE_INDICES[bitmap as usize]
}

pub const TILE_SIZE: usize = 32;
pub const TILESET_PATH: &str = "assets/tileset.png";
