use crate::constants::biome::Biome;

/// The terrain layer ordering is as follows:
/// Water - Lowest
/// Sand
/// Stone
/// Dirt
/// Grass - Highest
#[derive(Clone, PartialEq, Eq, Hash, Debug)]

pub enum TerrainType {
    Water,
    Sand,
    Stone,
    Dirt,
    Grass,
}

// ❗❗ Update this whenever a terrain type is added or removed‼️‼️❗❗
pub const TERRAIN_TYPE_COUNT: usize = 5;

// Static array of all terrain types
pub const ALL_TERRAIN_TYPES: [TerrainType; TERRAIN_TYPE_COUNT] = [
    TerrainType::Water,
    TerrainType::Sand,
    TerrainType::Stone,
    TerrainType::Dirt,
    TerrainType::Grass,
];

pub struct NoiseCutoffs {
    pub dirt_threshold: f64,
    pub grass_threshold: f64,
    pub sand_threshold: f64,
    pub stone_threshold: f64,
    pub water_threshold: f64,
}

pub fn get_noise_cutoffs(biome: &Biome) -> NoiseCutoffs {
    let noise_cutoffs: NoiseCutoffs = NoiseCutoffs {
        water_threshold: biome.water_percentage as f64 / 100.0,
        sand_threshold: (biome.sand_percentage + biome.water_percentage) as f64 / 100.0,
        stone_threshold: (biome.stone_percentage + biome.sand_percentage + biome.water_percentage)
            as f64
            / 100.0,
        dirt_threshold: (biome.dirt_percentage
            + biome.stone_percentage
            + biome.sand_percentage
            + biome.water_percentage) as f64
            / 100.0,
        grass_threshold: (biome.grass_percentage
            + biome.dirt_percentage
            + biome.stone_percentage
            + biome.sand_percentage
            + biome.water_percentage) as f64
            / 100.0,
    };

    noise_cutoffs
}
