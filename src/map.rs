use crate::constants::map_gen::*;
use crate::constants::tiles::TileType;
use noise::{NoiseFn, Perlin};

#[derive(Clone)]
pub struct Tile {
    pub tile_type: TileType, // Use TileType for each tile
    pub rotation: u16,       // 90, 180, 270 degrees
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

/// Maps Perlin noise "generic" tile types
fn get_basic_tile_type(noise_value: f64) -> TileType {
    let mut tile_type = TileType::Grass1;
    if noise_value < NOISE_CUTOFFS.grass_threshold {
        tile_type = TileType::Grass1;
    } else if noise_value < NOISE_CUTOFFS.dirt_threshold {
        tile_type = TileType::Dirt1;
    }

    tile_type
}

fn is_corner(noise_value: f64, neighbors: &[f64; 4]) -> bool {
    let is_grass = noise_value < NOISE_CUTOFFS.grass_threshold;

    neighbors
        .iter()
        .filter(|&&neighbor| (neighbor < NOISE_CUTOFFS.grass_threshold) == is_grass)
        .count()
        == 2 // Only two neighbors match, forming a corner
}

fn calculate_rotation(
    noise_value: f64,
    neighbors: &[f64; 4], // Top, Bottom, Left, Right
) -> u16 {
    let is_grass = noise_value < NOISE_CUTOFFS.grass_threshold;

    match (
        (neighbors[0] < NOISE_CUTOFFS.grass_threshold) == is_grass, // Top
        (neighbors[1] < NOISE_CUTOFFS.grass_threshold) == is_grass, // Bottom
        (neighbors[2] < NOISE_CUTOFFS.grass_threshold) == is_grass, // Left
        (neighbors[3] < NOISE_CUTOFFS.grass_threshold) == is_grass, // Right
    ) {
        (false, true, false, true) => 0, // Bottom-Right (default orientation)
        (true, false, false, true) => 90, // Top-Right
        (true, false, true, false) => 180, // Top-Left
        (false, true, true, false) => 270, // Bottom-Left
        _ => 0,                          // Default to no rotation
    }
}

pub fn generate_tile(x: usize, y: usize, perlin: &Perlin) -> Tile {
    // Normalize coordinates
    let nx = x as f64 / MAP_SIZE_X as f64;
    let ny = y as f64 / MAP_SIZE_Y as f64;

    // Get Perlin noise values for the current tile and neighbors
    let noise_value = perlin.get([nx * NOISE_SCALE, ny * NOISE_SCALE, 0.0]);
    let neighbors = [
        perlin.get([
            (nx - 1.0 / MAP_SIZE_X as f64) * NOISE_SCALE,
            ny * NOISE_SCALE,
            0.0,
        ]), // Left
        perlin.get([
            (nx + 1.0 / MAP_SIZE_X as f64) * NOISE_SCALE,
            ny * NOISE_SCALE,
            0.0,
        ]), // Right
        perlin.get([
            nx * NOISE_SCALE,
            (ny - 1.0 / MAP_SIZE_Y as f64) * NOISE_SCALE,
            0.0,
        ]), // Top
        perlin.get([
            nx * NOISE_SCALE,
            (ny + 1.0 / MAP_SIZE_Y as f64) * NOISE_SCALE,
            0.0,
        ]), // Bottom
    ];

    // Determine basic tile type
    let tile_type = get_basic_tile_type(noise_value);

    // Determine if the tile is an edge or corner
    if neighbors.iter().any(|&neighbor| {
        (neighbor < NOISE_CUTOFFS.grass_threshold) != (noise_value < NOISE_CUTOFFS.grass_threshold)
    }) {
        let rotation = calculate_rotation(noise_value, &neighbors);
        Tile {
            tile_type: TileType::GrassEdge,
            rotation,
        }
    } else if is_corner(noise_value, &neighbors) {
        let rotation = calculate_rotation(noise_value, &neighbors);
        Tile {
            tile_type: if noise_value < NOISE_CUTOFFS.grass_threshold {
                TileType::GrassOuterCorner
            } else {
                TileType::GrassInnerCorner
            },
            rotation,
        }
    } else {
        Tile {
            tile_type,
            rotation: 0,
        }
    }
}
