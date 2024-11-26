#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Grass1 = 0,
    Grass2 = 1,
    Grass3 = 2,
    Dirt1 = 3,
    Dirt2 = 4,
    Dirt3 = 5,

    GrassEdge = 32,
    GrassOuterCorner = 33,
    GrassInnerCorner = 34,
}

impl TileType {
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

pub const TILE_SIZE: usize = 32;
pub const TILESET_PATH: &str = "assets/tileset.png";

#[derive(Clone)]
pub enum TerrainType {
    Grass = 0,
    Dirt = 1,
}
