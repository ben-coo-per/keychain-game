mod constants;
mod map;
mod renderer;
mod tileset;

use map::generate_terrain;
use platform::pc::*;
use renderer::Renderer;
use tileset::Tileset;

#[cfg(target_os = "macos")]
mod platform {
    pub mod pc;
}

fn main() {
    let tileset = Tileset::new(constants::tiles::TILESET_PATH, 32, 32);
    let terrain = generate_terrain(constants::map_gen::MAP_SEED);

    let mut renderer = PCRenderer::new();
    let mut offset_x = 0.0;
    let mut offset_y = 0.0;

    loop {
        handle_input(&mut renderer.window, &mut offset_x, &mut offset_y);
        renderer.render(&terrain, &tileset, offset_x, offset_y);
    }
}
