use crate::{
    constants::{
        device::{SCREEN_HEIGHT, SCREEN_WIDTH},
        tiles::TILE_SIZE,
    },
    renderer::Renderer,
    terrain::map::Viewport,
    tileset::Tileset,
};
use minifb::{Key, Window, WindowOptions};

pub fn handle_input(window: &mut Window, offset_x: &mut f64, offset_y: &mut f64) {
    if window.is_key_down(Key::Up) {
        *offset_y -= TILE_SIZE as f64;
    }
    if window.is_key_down(Key::Down) {
        *offset_y += TILE_SIZE as f64;
    }
    if window.is_key_down(Key::Left) {
        *offset_x -= TILE_SIZE as f64;
    }
    if window.is_key_down(Key::Right) {
        *offset_x += TILE_SIZE as f64;
    }

    // Round offset values to one decimal place
    *offset_x = (*offset_x * 10.0).round() / 10.0;
    *offset_y = (*offset_y * 10.0).round() / 10.0;
}

pub struct PCRenderer {
    pub window: Window,
}

impl PCRenderer {
    pub fn new() -> Self {
        let window = Window::new(
            "Tile Renderer",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            WindowOptions::default(),
        )
        .expect("Failed to create window");
        Self { window }
    }
}

impl Renderer for PCRenderer {
    fn render(&mut self, viewport: &Viewport, tileset: &Tileset, offset_x: f64, offset_y: f64) {
        // Create an empty buffer to store the pixel data for the window
        let mut buffer = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

        // Generate tiles for the current viewport
        let tiles_to_render = viewport.get_tiles_to_render(offset_x, offset_y);

        // Loop through the `TileCake` structure to render layers from bottom to top
        for (y, row) in tiles_to_render.iter().enumerate() {
            for (x, tile_cake) in row.iter().enumerate() {
                let tile_screen_x = x * TILE_SIZE;
                let tile_screen_y = y * TILE_SIZE;

                for (terrain_layer, tile_index) in tile_cake.iter().enumerate() {
                    // Get the tile's pixel data from the tileset
                    let tile_pixels = tileset.get_tile_pixels(*tile_index as usize);

                    // Copy tile pixels into the buffer
                    for ty in 0..TILE_SIZE {
                        for tx in 0..TILE_SIZE {
                            let buffer_x = tile_screen_x + tx;
                            let buffer_y = tile_screen_y + ty;
                            if buffer_x < SCREEN_WIDTH && buffer_y < SCREEN_HEIGHT {
                                let buffer_index = buffer_y * SCREEN_WIDTH + buffer_x;

                                // Draw the pixel only if it's part of the current layer
                                let pixel_value = tile_pixels[ty * TILE_SIZE + tx];
                                if pixel_value != 0 {
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
