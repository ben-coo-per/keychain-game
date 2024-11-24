use noise::NoiseFn;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::{constants::color::*, constants::map_gen::*};

#[derive(Clone)]
pub struct Tile {
    pub terrain_color: u32,
    pub shadow_color: u32,
    pub texture_color: u32,
}

// Function to generate the terrain using Perlin noise
pub fn generate_terrain(seed: u32) -> Vec<Vec<Tile>> {
    let perlin = noise::Perlin::new(seed);

    let mut terrain = vec![
        vec![
            Tile {
                terrain_color: 0,
                shadow_color: 0,
                texture_color: 0
            };
            MAP_SIZE_X
        ];
        MAP_SIZE_Y
    ];

    for y in 0..MAP_SIZE_Y {
        for x in 0..MAP_SIZE_X {
            let nx = x as f64 / MAP_SIZE_X as f64 * ELEVATION_NOISE_SCALE;
            let ny = y as f64 / MAP_SIZE_Y as f64 * ELEVATION_NOISE_SCALE;

            let noise_value = perlin.get([nx + seed as f64, ny + seed as f64]);
            let terrain_color;
            let shadow_color;
            let texture_color;
            if noise_value < -0.5 {
                // water
                terrain_color = Color::Black.to_u32();
                shadow_color = Color::LightGray.to_u32();
                texture_color = Color::White.to_u32();
            } else if noise_value < 0.0 {
                terrain_color = Color::DarkGray.to_u32();
                shadow_color = Color::Black.to_u32();
                texture_color = Color::LightGray.to_u32();
            } else if noise_value < 0.5 {
                terrain_color = Color::LightGray.to_u32();
                shadow_color = Color::DarkGray.to_u32();
                texture_color = Color::White.to_u32();
            } else {
                terrain_color = Color::White.to_u32();
                shadow_color = Color::LightGray.to_u32();
                texture_color = Color::DarkGray.to_u32();
            };

            terrain[y][x] = Tile {
                terrain_color,
                shadow_color,
                texture_color,
            };
        }
    }

    terrain
}

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

    for l in 0..SHADOW_DEPTH {
        let dots_to_add = (SHADOW_DECAY.powi(l as i32) * SHADOW_DENSITY as f64) as usize;
        for _ in 0..dots_to_add {
            match edge {
                "top" => {
                    let offset_x = rng.gen_range(0..TILE_SIZE);
                    dots.push((x * TILE_SIZE + offset_x, y * TILE_SIZE + l));
                }
                "right" => {
                    let offset_y = rng.gen_range(0..TILE_SIZE);
                    dots.push((x * TILE_SIZE + TILE_SIZE - 1 - l, y * TILE_SIZE + offset_y));
                }
                "bottom" => {
                    let offset_x = rng.gen_range(0..TILE_SIZE);
                    dots.push((x * TILE_SIZE + offset_x, y * TILE_SIZE + TILE_SIZE - 1 - l));
                }
                "left" => {
                    let offset_y = rng.gen_range(0..TILE_SIZE);
                    dots.push((x * TILE_SIZE + l, y * TILE_SIZE + offset_y));
                }
                _ => {}
            }
        }
    }

    dots
}

pub fn generate_textures(
    terrain: &Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    seed: u64,
) -> Vec<Vec<Vec<(usize, usize)>>> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let mut textures = vec![vec![vec![]; width]; height];

    for y in 0..height {
        for x in 0..width {
            // Texture styling parameters based on tile type
            let tile = &terrain[y][x];

            let mut tile_texture = vec![];

            // Generate lines for this tile
            for line in 0..(TILE_SIZE / TEXTURE_SPACING) {
                let y_base = line * TEXTURE_SPACING;

                // Generate dots along the line within the tile
                for _ in 0..TEXTURE_DOT_DENSITY {
                    let x_offset = rng.gen_range(0..TILE_SIZE); // Random horizontal position within the tile
                    let y_jitter = rng.gen_range(0..=TEXTURE_JITTER); // Slight vertical offset for randomness

                    tile_texture.push((x_offset, y_base + y_jitter));
                }
            }

            // Store the texture for the tile
            textures[y][x] = tile_texture;
        }
    }

    textures
}
