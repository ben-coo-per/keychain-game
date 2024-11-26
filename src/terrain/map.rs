use crate::constants::device::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::constants::tiles::{TerrainType, TileType, TILE_SIZE};
use noise::{Fbm, NoiseFn, Perlin};

#[derive(Clone)]
pub struct Tile {
    pub tile_type: TileType, // Use TileType for each tile
}

/// Configuration for noise thresholds to determine tile types
pub struct NoiseCutoffs {
    grass_threshold: f64,
    dirt_threshold: f64,
}
const NOISE_CUTOFFS: NoiseCutoffs = NoiseCutoffs {
    grass_threshold: 0.2,
    dirt_threshold: 0.4,
};

pub fn generate_terrain_grid(
    perlin: &Fbm<Perlin>,
    offset_x: f64,
    offset_y: f64,
) -> Vec<Vec<TerrainType>> {
    // Creates a 2D Vector of the viewport that holds the terrain type for each tile in the terrain grid

    let mut viewport_terrain_grid = vec![vec![TerrainType::Grass; SCREEN_WIDTH]; SCREEN_HEIGHT];

    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            let noise_value = perlin.get([
                (x as f64 + offset_x) / TILE_SIZE as f64,
                (y as f64 + offset_y) / TILE_SIZE as f64,
            ]);

            let terrain_type = match noise_value {
                n if n < NOISE_CUTOFFS.grass_threshold => TerrainType::Grass,
                n if n < NOISE_CUTOFFS.dirt_threshold => TerrainType::Dirt,
                _ => TerrainType::Dirt,
            };

            viewport_terrain_grid[y][x] = terrain_type;
        }
    }

    viewport_terrain_grid
}
