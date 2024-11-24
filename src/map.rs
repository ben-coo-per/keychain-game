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
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn generate_shadows(
    terrain: &Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    seed: u64,
) -> Vec<Vec<Vec<(usize, usize)>>> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut shadows = vec![vec![vec![]; width]; height];

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

            // Add scattered shadow dots for each shaded edge
            if center < top {
                shadows[y][x].append(&mut generate_shadow_dots(x, y, "top", &mut rng));
            }
            if center < right {
                shadows[y][x].append(&mut generate_shadow_dots(x, y, "right", &mut rng));
            }
            if center < bottom {
                shadows[y][x].append(&mut generate_shadow_dots(x, y, "bottom", &mut rng));
            }
            if center < left {
                shadows[y][x].append(&mut generate_shadow_dots(x, y, "left", &mut rng));
            }
        }
    }

    shadows
}

/// Generates scattered dots for shadows along a specific edge of a tile
fn generate_shadow_dots(x: usize, y: usize, edge: &str, rng: &mut StdRng) -> Vec<(usize, usize)> {
    let mut dots = vec![];
    let density = 60; // Number of dots to scatter along the edge
    let depth = 7; // Depth of the shadow
    for _ in 0..density {
        match edge {
            "top" => {
                let offset_x = rng.gen_range(0..constants::TILE_SIZE);
                let offset_y = rng.gen_range(0..depth);
                dots.push((
                    x * constants::TILE_SIZE + offset_x,
                    y * constants::TILE_SIZE + offset_y,
                ));
            }
            "right" => {
                let offset_y = rng.gen_range(0..constants::TILE_SIZE);
                let offset_x = rng.gen_range(0..depth);
                dots.push((
                    x * constants::TILE_SIZE + constants::TILE_SIZE - 1 - offset_x,
                    y * constants::TILE_SIZE + offset_y,
                ));
            }
            "bottom" => {
                let offset_x = rng.gen_range(0..constants::TILE_SIZE);
                let offset_y = rng.gen_range(0..depth);
                dots.push((
                    x * constants::TILE_SIZE + offset_x,
                    y * constants::TILE_SIZE + constants::TILE_SIZE - 1 - offset_y,
                ));
            }
            "left" => {
                let offset_y = rng.gen_range(0..constants::TILE_SIZE);
                let offset_x = rng.gen_range(0..depth);
                dots.push((
                    x * constants::TILE_SIZE + offset_x,
                    y * constants::TILE_SIZE + offset_y,
                ));
            }
            _ => {}
        }
    }

    dots
}
