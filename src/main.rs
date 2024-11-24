mod constants;
mod map;
mod renderer;

#[cfg(target_os = "macos")]
mod platform {
    pub mod pc;
}

use renderer::Renderer;

fn main() {
    // Create the renderer and input handler
    #[cfg(target_os = "macos")]
    let mut renderer = platform::pc::PCRenderer::new();

    // Initialize game logic
    let map = map::generate_map(constants::MAP_SEED); // Procedural map generation with seed
    let mut offset_x = 0.0;
    let mut offset_y = 0.0;

    // Main game loop
    loop {
        #[cfg(target_os = "macos")]
        platform::pc::handle_input(&mut renderer.window, &mut offset_x, &mut offset_y);

        // Render the map
        renderer.render(&map, offset_x, offset_y);
    }
}
