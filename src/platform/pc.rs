use crate::{
    constants::{
        device::{SCREEN_HEIGHT, SCREEN_WIDTH},
        experience::MOVE_SPEED,
        terrain::ALL_TERRAIN_TYPES,
        tiles::TILE_SIZE,
    },
    renderer::Renderer,
    tileset::TileAtlas,
};
use minifb::{Key, Window, WindowOptions};

pub fn handle_input(
    window: &mut Window,
    offset_x: &mut f64,
    offset_y: &mut f64,
    view_changed: &mut bool,
) {
    if window.is_key_down(Key::Up) {
        *offset_y += MOVE_SPEED as f64;
        *view_changed = true;
    }
    if window.is_key_down(Key::Down) {
        *offset_y -= MOVE_SPEED as f64;
        *view_changed = true;
    }
    if window.is_key_down(Key::Left) {
        *offset_x += MOVE_SPEED as f64;
        *view_changed = true;
    }
    if window.is_key_down(Key::Right) {
        *offset_x -= MOVE_SPEED as f64;
        *view_changed = true;
    }

    // Round offset values to two decimal places
    *offset_x = (*offset_x * 100.0).round() / 100.0;
    *offset_y = (*offset_y * 100.0).round() / 100.0;
}

pub struct PCRenderer {
    pub window: Window,
}

impl PCRenderer {
    pub fn new() -> Self {
        let window = Window::new(
            "Apocalypse Keychain",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            WindowOptions::default(),
        )
        .expect("Failed to create window");
        Self { window }
    }
}

impl Renderer for PCRenderer {
    fn render(&mut self, tiles_to_render: &Vec<Vec<[u8; 2]>>, tile_atlas: &TileAtlas) {
        // Create an empty buffer to store the pixel data for the window
        let mut buffer = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

        // Loop through the `TileCake` structure to render layers from bottom to top
        for (y, row) in tiles_to_render.iter().enumerate() {
            for (x, tile_cake) in row.iter().enumerate() {
                let tile_screen_x = x * TILE_SIZE;
                let tile_screen_y = y * TILE_SIZE;

                for (terrain_layer_index, tile_index) in tile_cake.iter().enumerate() {
                    // Get the terrain type for the current layer
                    let terrain_layer = &ALL_TERRAIN_TYPES[terrain_layer_index];

                    // Get the tile's pixel data from the tileOffset
                    let tile_pixels =
                        tile_atlas.get_tile_pixels(*tile_index as usize, terrain_layer);

                    // Copy tile pixels into the buffer
                    for ty in 0..TILE_SIZE {
                        for tx in 0..TILE_SIZE {
                            let buffer_x = tile_screen_x + tx;
                            let buffer_y = tile_screen_y + ty;
                            if buffer_x < SCREEN_WIDTH && buffer_y < SCREEN_HEIGHT {
                                let buffer_index = buffer_y * SCREEN_WIDTH + buffer_x;

                                // Draw the pixel only if it's part of the current layer and not transparent
                                let pixel_value = tile_pixels[ty * TILE_SIZE + tx];
                                let alpha = (pixel_value >> 24) & 0xFF; // Extract the alpha value
                                if alpha != 0 {
                                    buffer[buffer_index] = pixel_value;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Update the window with the rendered buffer
        self.window
            .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}
