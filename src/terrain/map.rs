use crate::constants::biome;
use crate::constants::device::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::constants::terrain::{
    get_noise_cutoffs, TerrainType, ALL_TERRAIN_TYPES, TERRAIN_TYPE_COUNT,
};
use crate::constants::tiles::{get_tile_index_from_bitmap, TILE_SIZE};
use noise::{Fbm, NoiseFn, Perlin};

type TileCake = [u8; TERRAIN_TYPE_COUNT]; // array of img indexes for each terrain type

fn generate_terrain_grid(
    perlin: &Fbm<Perlin>,
    biome_noise: &Fbm<Perlin>,
    offset_x: f64,
    offset_y: f64,
) -> Vec<Vec<TerrainType>> {
    // Creates a 2D Vector of the viewport that holds the terrain type for each tile in the terrain grid

    let mut viewport_terrain_grid = vec![vec![TerrainType::Grass; SCREEN_WIDTH]; SCREEN_HEIGHT];

    let num_x_tiles = SCREEN_WIDTH / TILE_SIZE + 1;
    let num_y_tiles = SCREEN_HEIGHT / TILE_SIZE + 1;

    for x in 0..num_x_tiles {
        for y in 0..num_y_tiles {
            let tile_x = x as f64 - offset_x;
            let tile_y = y as f64 - offset_y;

            // Get the biome for the current tile
            let biome_noise_value = biome_noise.get([tile_x, tile_y]);
            let current_biome = biome::get_biome_from_noise_value(biome_noise_value);
            let noise_cutoffs = get_noise_cutoffs(&current_biome);

            let noise_value = perlin.get([tile_x, tile_y]);

            // Normalize the noise value to be between 0 and 1
            let normalized_noise = (noise_value + 1.0) / 2.0;
            let terrain_type = match normalized_noise {
                n if n > noise_cutoffs.grass_threshold => TerrainType::Grass,
                n if n > noise_cutoffs.dirt_threshold => TerrainType::Dirt,
                n if n > noise_cutoffs.stone_threshold => TerrainType::Stone,
                n if n > noise_cutoffs.sand_threshold => TerrainType::Sand,
                n if n > noise_cutoffs.water_threshold => TerrainType::Water,
                _ => TerrainType::Water,
            };

            viewport_terrain_grid[y][x] = terrain_type;
        }
    }

    viewport_terrain_grid
}

fn get_tile_bitmap(
    terrain_tiles: [&TerrainType; 4],
    remaining_terrain_layers: &[TerrainType], // Slice of "remaining terrains including current layer"
) -> u8 {
    let mut tile_bitmap: u8 = 0b0000;

    // Iterate through the tiles and check if any terrain in "remaining_terrain_layers" matches
    for (i, terrain_tile) in terrain_tiles.iter().enumerate() {
        // Check if the current terrain_tile matches any terrain in the remaining layers
        if remaining_terrain_layers.iter().any(|remaining| *terrain_tile == remaining) {
            tile_bitmap |= 1 << (3 - i); // Set the corresponding bit
        }
    }

    tile_bitmap
}

pub fn get_tile_cake(terrain_tiles: [&TerrainType; 4]) -> [u8; TERRAIN_TYPE_COUNT] {
    // Returns a list of img indexes for the tile based on the terrain tiles of the 4 corners of the tile
    let mut tile_cake: [u8; TERRAIN_TYPE_COUNT] =
        [get_tile_index_from_bitmap(0b0000); TERRAIN_TYPE_COUNT];

    for (i, _terrain_option) in ALL_TERRAIN_TYPES.iter().enumerate() {
        // Collect "remaining layers" dynamically from the current layer (i) onwards
        let remaining_layers = &ALL_TERRAIN_TYPES[i..];
        let bitmap = get_tile_bitmap(terrain_tiles, remaining_layers); // Pass the remaining layers
        tile_cake[i] = get_tile_index_from_bitmap(bitmap);
    }

    tile_cake
}

pub struct Viewport<'a> {
    terrain_noise_fn: &'a Fbm<Perlin>,
    biome_noise_fn: &'a Fbm<Perlin>,
}

impl<'a> Viewport<'a> {
    /// Create a new viewport with the given dimensions and tile size
    pub fn new(terrain_noise: &'a Fbm<Perlin>, biome_noise: &'a Fbm<Perlin>) -> Self {
        Self {
            terrain_noise_fn: terrain_noise,
            biome_noise_fn: biome_noise,
        }
    }

    /// Generate the tiles for the current viewport based on offsets
    pub fn get_tiles_to_render(&self, offset_x: f64, offset_y: f64) -> Vec<Vec<TileCake>> {
        // Calculate how many tiles fit in the viewport
        let tiles_across = SCREEN_WIDTH / TILE_SIZE;
        let tiles_down = SCREEN_HEIGHT / TILE_SIZE;

        // Get terrain grid for the current viewport (offset by 1/2 tile size up and left. Extend by 1 tile size down and right)
        let terrain_grid = generate_terrain_grid(
            self.terrain_noise_fn,
            self.biome_noise_fn,
            offset_x - TILE_SIZE as f64 / 2.0,
            offset_y - TILE_SIZE as f64 / 2.0,
        );

        // Choose the tile type based on the terrain types of the 4 corners of the tile
        let mut tiles: Vec<Vec<TileCake>> = Vec::new();
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
