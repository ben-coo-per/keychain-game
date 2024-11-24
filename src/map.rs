use crate::constants::map_gen::*;
use crate::constants::tiles::TileType;
use noise::{NoiseFn, Perlin};

#[derive(Clone)]
pub struct Tile {
    pub tile_type: TileType, // Use TileType for each tile
}

/// Maps Perlin noise values to grass-related TileTypes.
fn map_noise_to_tile_type(noise_value: f64) -> TileType {
    if noise_value < -0.2 {
        // 80% of the time return grass1, 10% grass2, 10% grass3
        match rand::random::<u8>() % 10 {
            0..=7 => TileType::Grass1,
            8 => TileType::Grass2,
            _ => TileType::Grass3,
        }
    } else if noise_value < 0.2 {
        // 90% of the time return dirt1, 5% dirt2, 5% dirt3
        match rand::random::<u8>() % 20 {
            0..=18 => TileType::Dirt1,
            19 => TileType::Dirt2,
            _ => TileType::Dirt3,
        }
    } else {
        TileType::Grass1
    }
}

/// Generates the terrain with grass tiles based on Perlin noise.
pub fn generate_terrain(seed: u32) -> Vec<Vec<Tile>> {
    let perlin = Perlin::new(seed);
    let mut terrain = vec![
        vec![
            Tile {
                tile_type: TileType::Grass1
            };
            MAP_SIZE_X
        ];
        MAP_SIZE_Y
    ];

    for y in 0..MAP_SIZE_Y {
        for x in 0..MAP_SIZE_X {
            let nx = x as f64 / MAP_SIZE_X as f64; // Normalize x
            let ny = y as f64 / MAP_SIZE_Y as f64; // Normalize y

            // Generate Perlin noise value
            let noise_value = perlin.get([nx * NOISE_SCALE, ny * NOISE_SCALE, seed as f64]);

            // Map noise to a TileType
            let tile_type = map_noise_to_tile_type(noise_value);

            // Assign the tile type
            terrain[y][x] = Tile { tile_type };
        }
    }

    terrain
}
