use noise::NoiseFn;

use crate::constants;

#[derive(Clone)]
pub struct Tile {
    pub terrain_color: u32,
}

// Function to generate the terrain using Perlin noise
pub fn generate_terrain(seed: u32) -> Vec<Vec<Tile>> {
    let perlin = noise::Perlin::new(seed);

    let mut terrain =
        vec![vec![Tile { terrain_color: 0 }; constants::MAP_SIZE_X]; constants::MAP_SIZE_Y];

    for y in 0..constants::MAP_SIZE_Y {
        for x in 0..constants::MAP_SIZE_X {
            let nx = x as f64 / constants::MAP_SIZE_X as f64 * constants::ELEVATION_NOISE_SCALE;
            let ny = y as f64 / constants::MAP_SIZE_Y as f64 * constants::ELEVATION_NOISE_SCALE;

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

            terrain[y][x] = Tile { terrain_color };
        }
    }

    terrain
}

// Function to generate shadows as edges around tiles
pub fn generate_shadows_as_lines(
    terrain: &Vec<Vec<Tile>>,
    width: usize,
    height: usize,
) -> Vec<Vec<(bool, bool, bool, bool)>> {
    let mut shadows = vec![vec![(false, false, false, false); width]; height];

    for y in 0..height {
        for x in 0..width {
            let center = terrain[y][x].terrain_color;

            // Check neighbors
            let top = if y > 0 {
                terrain[y - 1][x].terrain_color
            } else {
                center
            };
            let right = if x < width - 1 {
                terrain[y][x + 1].terrain_color
            } else {
                center
            };
            let bottom = if y < height - 1 {
                terrain[y + 1][x].terrain_color
            } else {
                center
            };
            let left = if x > 0 {
                terrain[y][x - 1].terrain_color
            } else {
                center
            };

            // Determine shaded edges
            shadows[y][x] = (
                center < top,    // Top edge
                center < right,  // Right edge
                center < bottom, // Bottom edge
                center < left,   // Left edge
            );
        }
    }

    shadows
}
