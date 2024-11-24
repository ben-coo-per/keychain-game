use crate::constants::*;
use noise::{NoiseFn, Perlin};

// A structure to represent a tile in the map
#[derive(Clone, Copy)]
pub struct Tile {
    pub terrain_color: u32, // The color of the terrain
}

// Generates a fixed 2D grid of tiles with terrain and features
pub fn generate_map(seed: u32) -> Vec<Vec<Tile>> {
    let perlin = Perlin::new(seed);

    // Initialize the map as a 2D vector of `Tile`s
    let mut map = vec![vec![Tile { terrain_color: 0 }; MAP_SIZE_X]; MAP_SIZE_Y];

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            // Use Perlin noise for terrain generation
            let nx = (x as f64) / (SCREEN_WIDTH / TILE_SIZE) as f64;
            let ny = (y as f64) / (SCREEN_HEIGHT / TILE_SIZE) as f64;

            // Generate noise value for terrain
            let noise_value = perlin.get([nx + seed as f64, ny + seed as f64]);
            let terrain_color = if noise_value < -0.5 {
                0x000000 // Black: Deep terrain
            } else if noise_value < 0.0 {
                0x555555 // Dark Gray: Low terrain
            } else if noise_value < 0.5 {
                0xAAAAAA // Light Gray: Elevated terrain
            } else {
                0xFFFFFF // White: Snow terrain
            };

            // Store the tile in the map
            map[y][x] = Tile { terrain_color };
        }
    }

    map
}
