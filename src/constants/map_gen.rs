// Map generation
pub const MAP_SEED: u32 = 42; // Seed for procedural map generation
pub const MAP_SIZE_Y: usize = 50; // Size of the map in tiles
pub const MAP_SIZE_X: usize = 50; // Size of the map in tiles

// Terrain generation
pub const ELEVATION_NOISE_SCALE: f64 = 3.0; // higher values make the terrain more varied
pub const TILE_SIZE: usize = 20; // Size of each tile

// Shadow generation
pub const SHADOW_DENSITY: usize = 60; // Number of dots to scatter along the edge
pub const SHADOW_DEPTH: usize = 7; // Depth of the shadow
pub const SHADOW_DECAY: f64 = 0.8; // Decay of the shadow
