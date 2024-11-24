// Map generation
pub const MAP_SEED: u32 = 42; // Seed for procedural map generation
pub const MAP_SIZE_Y: usize = 50; // Size of the map in tiles
pub const MAP_SIZE_X: usize = 50; // Size of the map in tiles

// Terrain generation
pub const ELEVATION_NOISE_SCALE: f64 = 3.0; // higher values make the terrain more varied
pub const TILE_SIZE: usize = 20; // Size of each tile

// Texture generation
pub const TEXTURE_SPACING: usize = TILE_SIZE / 5; // Space between lines within the tile
pub const TEXTURE_DOT_DENSITY: usize = 2; // Number of dots to scatter per line
pub const TEXTURE_JITTER: usize = 2; // Max pixel jitter for randomness

// Shadow generation
pub const SHADOW_DENSITY: usize = 35; // Number of dots to scatter first shadow level
pub const SHADOW_DEPTH: usize = 10; // Depth of the shadow
pub const SHADOW_DECAY: f64 = 0.7; // Decay of the shadow
