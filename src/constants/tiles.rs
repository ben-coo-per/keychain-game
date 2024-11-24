#[derive(Clone, Copy, Debug)]
pub enum TileType {
    GrassBasic = 0,    // Basic grass
    GrassFlowers1 = 1, // Grass with sparse flowers
    GrassFlowers2 = 2, // Grass with dense flowers
}

impl TileType {
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

pub const TILE_SIZE: usize = 32;
pub const TILESET_PATH: &str = "assets/tileset.png";
