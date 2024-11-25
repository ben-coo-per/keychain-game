mod constants;
mod map;
mod renderer;
mod tileset;

use constants::tiles::TILE_SIZE;
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use platform::pc::*;
use renderer::Renderer;
use tileset::Tileset;

#[cfg(target_os = "macos")]
mod platform {
    pub mod pc;
}

fn main() {
    // Load the tileset
    let tileset = Tileset::new(constants::tiles::TILESET_PATH, TILE_SIZE, TILE_SIZE);

    // Initialize the Fbm noise generator
    let perlin = Fbm::<Perlin>::new(constants::map_gen::MAP_SEED)
        .set_octaves(constants::map_gen::NOISE_OCTAVES)
        .set_frequency(constants::map_gen::NOISE_FREQUENCY);

    // Create the renderer
    let mut renderer = PCRenderer::new();
    let mut offset_x = 0.0;
    let mut offset_y = 0.0;

    loop {
        // Handle user input to adjust the viewport offsets
        handle_input(&mut renderer.window, &mut offset_x, &mut offset_y);

        // Render the map using dynamically generated tiles
        renderer.render(&perlin, &tileset, offset_x, offset_y);
    }
}
