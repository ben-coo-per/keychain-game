use crate::constants::device::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::constants::terrain::{TerrainType, ALL_TERRAIN_TYPES, TERRAIN_TYPE_COUNT};
use crate::constants::tiles::{get_tile_index_from_bitmap, TILE_SIZE};
use noise::{Fbm, NoiseFn, Perlin};

type TileCake = [u8; TERRAIN_TYPE_COUNT]; // array of img indexes for each terrain type

/// Configuration for noise thresholds to determine tile types
pub struct NoiseCutoffs {
    dirt_threshold: f64,
    grass_threshold: f64,
}
const NOISE_CUTOFFS: NoiseCutoffs = NoiseCutoffs {
    dirt_threshold: 0.0,
    grass_threshold: 0.5,
};

fn generate_terrain_grid(
    perlin: &Fbm<Perlin>,
    offset_x: f64,
    offset_y: f64,
) -> Vec<Vec<TerrainType>> {
    // Creates a 2D Vector of the viewport that holds the terrain type for each tile in the terrain grid

    let mut viewport_terrain_grid = vec![vec![TerrainType::Grass; SCREEN_WIDTH]; SCREEN_HEIGHT];

    let num_x_tiles = SCREEN_WIDTH / TILE_SIZE + 1;
    let num_y_tiles = SCREEN_HEIGHT / TILE_SIZE + 1;

    for x in 0..num_x_tiles {
        for y in 0..num_y_tiles {
            let noise_value = perlin.get([(x as f64 - offset_x), (y as f64 - offset_y)]);

            // Normalize the noise value to be between 0 and 1
            let normalized_noise = (noise_value + 1.0) / 2.0;
            let terrain_type = match normalized_noise {
                n if n > NOISE_CUTOFFS.grass_threshold => TerrainType::Grass,
                n if n > NOISE_CUTOFFS.dirt_threshold => TerrainType::Dirt,
                _ => TerrainType::Dirt,
            };

            viewport_terrain_grid[y][x] = terrain_type;
        }
    }

    viewport_terrain_grid
}

fn get_tile_bitmap(target_terrain: &TerrainType, terrain_tiles: [&TerrainType; 4]) -> u8 {
    let mut tile_bitmap: u8 = 0b0000;
    for (i, terrain_tile) in terrain_tiles.iter().enumerate() {
        if **terrain_tile == *target_terrain {
            tile_bitmap |= 1 << (3 - i);
        }
    }
    tile_bitmap
}

pub fn get_tile_cake(terrain_tiles: [&TerrainType; 4]) -> [u8; TERRAIN_TYPE_COUNT] {
    // Returns a list of img indexes for the tile based on the terrain tiles of the 4 corners of the tile
    let mut tile_cake: [u8; TERRAIN_TYPE_COUNT] =
        [get_tile_index_from_bitmap(0b0000); TERRAIN_TYPE_COUNT];
    for (i, terrain_option) in ALL_TERRAIN_TYPES.iter().enumerate() {
        let bitmap = get_tile_bitmap(terrain_option, terrain_tiles);
        tile_cake[i] = get_tile_index_from_bitmap(bitmap);
    }

    tile_cake
}

pub struct Viewport<'a> {
    noise_fn: &'a Fbm<Perlin>, // Procedural noise generator
}

impl<'a> Viewport<'a> {
    /// Create a new viewport with the given dimensions and tile size
    pub fn new(noise_fn: &'a Fbm<Perlin>) -> Self {
        Self { noise_fn }
    }

    /// Generate the tiles for the current viewport based on offsets
    pub fn get_tiles_to_render(&self, offset_x: f64, offset_y: f64) -> Vec<Vec<TileCake>> {
        // Calculate how many tiles fit in the viewport
        let tiles_across = SCREEN_WIDTH / TILE_SIZE;
        let tiles_down = SCREEN_HEIGHT / TILE_SIZE;

        // Get terrain grid for the current viewport (offset by 1/2 tile size up and left. Extend by 1 tile size down and right)
        let terrain_grid = generate_terrain_grid(
            self.noise_fn,
            offset_x - TILE_SIZE as f64 / 2.0,
            offset_y - TILE_SIZE as f64 / 2.0,
        );

        // Choose the tile type based on the terrain types of the 4 corners of the tile
        let mut tiles: Vec<Vec<TileCake>> = Vec::new();

        // Iterate over the tiles in the viewport
        for y in 0..tiles_down {
            let mut row: Vec<TileCake> = Vec::new();
            for x in 0..tiles_across {
                let terrains = [
                    &terrain_grid[y][x],
                    &terrain_grid[y][x + 1],
                    &terrain_grid[y + 1][x + 1],
                    &terrain_grid[y + 1][x],
                ];
                let tile_cake = get_tile_cake(terrains);

                row.push(tile_cake);
            }
            tiles.push(row);
        }
        tiles
    }
}
