mod constants;
mod map;
mod renderer;
mod tileset;

use noise::Perlin;
use platform::pc::*;
use renderer::Renderer;
use tileset::Tileset;

#[cfg(target_os = "macos")]
mod platform {
    pub mod pc;
}

fn main() {
    // Load the tileset
    let tileset = Tileset::new(constants::tiles::TILESET_PATH, 32, 32);

    // Initialize the Perlin noise generator
    let perlin = Perlin::new(constants::map_gen::MAP_SEED);

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
