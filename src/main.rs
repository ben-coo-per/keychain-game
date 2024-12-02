mod constants;
mod renderer;
mod terrain;
mod tileset;

use constants::terrain::TerrainType;
use constants::tiles::{TILESET_PATH, TILE_SIZE};
use noise::{Fbm, MultiFractal, Perlin};
use platform::pc::*;
use renderer::Renderer;
use terrain::map::Viewport;
use tileset::TileAtlas;

#[cfg(target_os = "macos")]
mod platform {
    pub mod pc;
}

fn main() {
    // Initialize Tile Atlas
    let mut tile_atlas: TileAtlas = TileAtlas::new(TILESET_PATH, TILE_SIZE, TILE_SIZE);

    // Register tilesets
    tile_atlas.register_tileset(TerrainType::Grass, 0, 0);
    tile_atlas.register_tileset(TerrainType::Dirt, 5, 0);

    // Initialize the Fbm noise generator
    let perlin = Fbm::<Perlin>::new(constants::map_gen::MAP_SEED)
        .set_octaves(constants::map_gen::NOISE_OCTAVES)
        .set_frequency(constants::map_gen::NOISE_FREQUENCY);

    // Create the viewport
    let viewport = Viewport::new(&perlin);

    // Create the renderer
    let mut renderer = PCRenderer::new();
    let mut offset_x = 0.0;
    let mut offset_y = 0.0;

    loop {
        // Handle user input to adjust the viewport offsets
        handle_input(&mut renderer.window, &mut offset_x, &mut offset_y);

        // Generate tiles for the current viewport
        println!("offset_x: {}, offset_y: {}", offset_x, offset_y);
        let tiles_to_render = viewport.get_tiles_to_render(offset_x, offset_y);

        // Render the map using dynamically generated tiles
        renderer.render(&tiles_to_render, &tile_atlas);
    }
}
