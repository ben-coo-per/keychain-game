// Map generation
pub const MAP_SEED: u32 = 42; // Seed for procedural map generation
pub const MAP_SIZE_Y: usize = 75; // Size of the map in tiles
pub const MAP_SIZE_X: usize = 75; // Size of the map in tiles

// Terrain generation
pub const ELEVATION_NOISE_SCALE: f64 = 3.0; // higher values make the terrain more varied
pub const TILE_SIZE: usize = 20; // Size of each tile

// Texture generation
pub const TEXTURE_SPACING: usize = TILE_SIZE / 10; // Space between lines within the tile
pub const TEXTURE_DOT_DENSITY: usize = 1; // Number of dots to scatter per line
pub const TEXTURE_JITTER: usize = 0; // Max pixel jitter for randomness

// Shadow generation
pub const SHADOW_DENSITY: usize = 45; // Number of dots to scatter first shadow level
pub const SHADOW_DEPTH: usize = 8; // Depth of the shadow
pub const SHADOW_DECAY: f64 = 0.8; // Decay of the shadow
