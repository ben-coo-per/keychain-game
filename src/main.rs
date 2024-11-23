mod constants;
mod feature;
mod map;

use constants::*;
use feature::Feature;
use map::generate_map;
use minifb::{Key, Window, WindowOptions};

fn main() {
    let mut window = Window::new(
        "Procedurally Generated Map with Features",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    // Generate features (e.g., trees)
    let features = Feature::generate_random_features(10, TILE_SIZE);

    // Initialize panning offsets
    let mut offset_x = 0.0;
    let mut offset_y = 0.0;
    let pan_speed = 0.1;

    let mut buffer = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update offsets based on key presses
        if window.is_key_down(Key::Up) {
            offset_y -= pan_speed;
        }
        if window.is_key_down(Key::Down) {
            offset_y += pan_speed;
        }
        if window.is_key_down(Key::Left) {
            offset_x -= pan_speed;
        }
        if window.is_key_down(Key::Right) {
            offset_x += pan_speed;
        }

        // Generate map with current offsets
        generate_map(&mut buffer, offset_x, offset_y);

        // Render features on top of the map
        for feature in &features {
            feature.render(&mut buffer, offset_x, offset_y);
        }

        // Render the map
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
