mod constants;
mod map;
mod renderer;

#[cfg(target_os = "macos")]
mod platform {
    pub mod pc;
}

use renderer::Renderer;

fn main() {
    // Initialize terrain and shadow generation
    let terrain = map::generate_terrain(constants::map_gen::MAP_SEED);
    let shadows = map::generate_shadows(
        &terrain,
        constants::map_gen::MAP_SIZE_X,
        constants::map_gen::MAP_SIZE_Y,
        constants::map_gen::MAP_SEED as u64,
    );

    // Create the renderer
    #[cfg(target_os = "macos")]
    let mut renderer = platform::pc::PCRenderer::new();

    // Initialize offsets for panning
    let mut offset_x = 0.0;
    let mut offset_y = 0.0;

    // Main game loop
    loop {
        // Handle user input to adjust panning offsets
        #[cfg(target_os = "macos")]
        platform::pc::handle_input(&mut renderer.window, &mut offset_x, &mut offset_y);

        // Render terrain and shadows to screen
        renderer.render(&terrain, &shadows, offset_x, offset_y);
    }
}
