use crate::constants::device::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::constants::tiles::{TileType, TILE_SIZE};
use noise::{Fbm, NoiseFn, Perlin};

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

/// Maps Fbm noise "generic" tile types
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

pub fn generate_tile(x: isize, y: isize, perlin: &Fbm<Perlin>) -> Tile {
    // println!("nx: {}, ny: {}", nx, ny);
    // Get Fbm noise values for the current tile and neighbors
    let noise_value = perlin.get([x as f64, y as f64, 0.0]);
    // let neighbors = [
    //     perlin.get([nx - NOISE_SCALE, ny, 0.0]), // Left
    //     perlin.get([nx + NOISE_SCALE, ny, 0.0]), // Right
    //     perlin.get([nx, ny - NOISE_SCALE, 0.0]), // Top
    //     perlin.get([nx, ny + NOISE_SCALE, 0.0]), // Bottom
    // ];

    // Determine basic tile type
    let tile_type = get_basic_tile_type(noise_value);

    // Get edges and corners
    // get_edges_and_corners(noise_value, &neighbors, tile_type)

    Tile {
        tile_type,
        rotation: 0,
    }
}

fn get_edges_and_corners(noise_value: f64, neighbors: &[f64; 4], tile_type: TileType) -> Tile {
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

pub fn generate_viewport_tiles(perlin: &Fbm<Perlin>, offset_x: f64, offset_y: f64) -> Vec<Tile> {
    /// Generates a vector of Tiles for the current viewport
    /// The size of the vector is equal to the number of tiles that fit in the viewport
    ///
    /// # Arguments
    /// * `perlin` - Fbm noise generator
    /// * `offset_x` - Where the viewport starts horizontally
    /// * `offset_y` - Where the viewport starts vertically
    ///
    /// # Returns
    /// * `Vec<Tile>` - Vector of Tiles for the current viewport
    let mut tiles = Vec::with_capacity(SCREEN_WIDTH / TILE_SIZE * SCREEN_HEIGHT / TILE_SIZE);

    for y in 0..SCREEN_WIDTH / TILE_SIZE {
        for x in 0..SCREEN_WIDTH / TILE_SIZE {
            let tile = generate_tile(
                (x as f64 + offset_x) as isize,
                (y as f64 + offset_y) as isize,
                &perlin,
            );
            tiles.push(tile);
        }
    }

    tiles
}
