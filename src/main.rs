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
    tile_atlas.register_tileset(TerrainType::Dirt, 4, 0);

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
    let mut view_changed = true;
    let mut tiles_to_render: Vec<Vec<[u8; 2]>> = Vec::new();
    loop {
        handle_input(
            &mut renderer.window,
            &mut offset_x,
            &mut offset_y,
            &mut view_changed,
        );

        if view_changed {
            // Generate tiles for the current viewport
            tiles_to_render = viewport.get_tiles_to_render(offset_x, offset_y);

            // Render the map using dynamically generated tiles

            view_changed = false;
        }

        if tiles_to_render.len() == 0 {
            continue;
        }

        renderer.render(&tiles_to_render, &tile_atlas);

        // Sleep for a short duration to avoid busy-waiting
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
