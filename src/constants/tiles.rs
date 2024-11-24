#[derive(Clone, Copy, Debug)]
pub enum TileType {
    Grass1 = 0,
    Grass2 = 1,
    Grass3 = 2,

    Dirt1 = 32,
    Dirt2 = 33,
    Dirt3 = 34,
}

impl TileType {
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

pub const TILE_SIZE: usize = 32;
pub const TILESET_PATH: &str = "assets/tileset.png";
