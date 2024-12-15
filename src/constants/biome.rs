/// # Biome Types
/// Biomes are sets of terrain percentages.
/// These percentages are converted into thresholds for the noise function.

pub struct Biome {
    pub grass_percentage: u8,
    pub dirt_percentage: u8,
    pub stone_percentage: u8,
    pub sand_percentage: u8,
    pub water_percentage: u8,
}

pub const BIOME_DEFAULT: Biome = Biome {
    grass_percentage: 50,
    dirt_percentage: 30,
    stone_percentage: 5,
    sand_percentage: 5,
    water_percentage: 5,
};

pub const BIOME_DESERT: Biome = Biome {
    grass_percentage: 5,
    dirt_percentage: 20,
    stone_percentage: 15,
    sand_percentage: 58,
    water_percentage: 2,
};

pub const BIOME_FOREST: Biome = Biome {
    grass_percentage: 70,
    dirt_percentage: 15,
    stone_percentage: 5,
    sand_percentage: 5,
    water_percentage: 5,
};

pub const BIOME_MOUNTAIN: Biome = Biome {
    grass_percentage: 10,
    dirt_percentage: 20,
    stone_percentage: 60,
    sand_percentage: 5,
    water_percentage: 5,
};

pub const BIOME_PLAINS: Biome = Biome {
    grass_percentage: 80,
    dirt_percentage: 10,
    stone_percentage: 5,
    sand_percentage: 5,
    water_percentage: 0,
};

pub fn get_biome_from_noise_value(noise_value: f64) -> Biome {
    // Converts a noise value into a biome
    let biome = match noise_value {
        n if n > 0.8 => BIOME_MOUNTAIN,
        n if n > 0.6 => BIOME_FOREST,
        n if n > 0.4 => BIOME_PLAINS,
        n if n > 0.2 => BIOME_DESERT,
        _ => BIOME_DEFAULT,
    };

    biome
}
