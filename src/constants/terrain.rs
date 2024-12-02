/// The terrain layer ordering is as follows:
/// Water - Lowest
/// Sand
/// Stone
/// Dirt
/// Grass - Highest
#[derive(Clone, PartialEq, Eq, Hash, Debug)]

pub enum TerrainType {
    // Water,
    // Sand,
    // Stone,
    Dirt,
    Grass,
}

// ❗❗ Update this whenever a terrain type is added or removed‼️‼️❗❗
pub const TERRAIN_TYPE_COUNT: usize = 2;

// Static array of all terrain types
pub const ALL_TERRAIN_TYPES: [TerrainType; TERRAIN_TYPE_COUNT] = [
    // TerrainType::Water,
    // TerrainType::Sand,
    // TerrainType::Stone,
    TerrainType::Dirt,
    TerrainType::Grass,
];
