use crate::{
    constants::{
        device::{SCREEN_HEIGHT, SCREEN_WIDTH},
        experience::*,
        map_gen::*,
        tiles::TILE_SIZE,
    },
    map::generate_tile,
    renderer::Renderer,
    tileset::Tileset,
};
use minifb::{Key, Window, WindowOptions};
use noise::Perlin;

pub struct PCRenderer {
    pub window: Window,
}

impl PCRenderer {
    pub fn new() -> Self {
        let window = Window::new(
            "Grass Terrain",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            WindowOptions::default(),
        )
        .expect("Failed to create window");
        Self { window }
    }
}

impl Renderer for PCRenderer {
    fn render(&mut self, perlin: &Perlin, tileset: &Tileset, offset_x: f64, offset_y: f64) {
        let mut buffer = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

        let tile_offset_x = offset_x as usize;
        let tile_offset_y = offset_y as usize;

        for y in 0..(SCREEN_HEIGHT / tileset.tile_height) {
            for x in 0..(SCREEN_WIDTH / tileset.tile_width) {
                let map_x = tile_offset_x + x;
                let map_y = tile_offset_y + y;

                // Dynamically generate the tile at (map_x, map_y)
                let tile = generate_tile(map_x, map_y, perlin);
                let tile_index = tile.tile_type.to_index();
                let tile_pixels = tileset.get_tile_pixels(tile_index);

                for ty in 0..tileset.tile_height {
                    for tx in 0..tileset.tile_width {
                        let px = x * tileset.tile_width + tx;
                        let py = y * tileset.tile_height + ty;

                        if px < SCREEN_WIDTH && py < SCREEN_HEIGHT {
                            // Apply rotation to tile pixels
                            let rotated_index = match tile.rotation {
                                0 => ty * tileset.tile_width + tx, // No rotation
                                90 => tx * tileset.tile_width + (tileset.tile_height - 1 - ty), // 90 degrees clockwise
                                180 => {
                                    (tileset.tile_height - 1 - ty) * tileset.tile_width
                                        + (tileset.tile_width - 1 - tx)
                                } // 180 degrees clockwise
                                270 => (tileset.tile_width - 1 - tx) * tileset.tile_width + ty, // 270 degrees clockwise
                                _ => ty * tileset.tile_width + tx, // Default: No rotation
                            };

                            buffer[py * SCREEN_WIDTH + px] = tile_pixels[rotated_index];
                        }
                    }
                }
            }
        }

        self.window
            .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}

/// Handles user input to adjust panning offsets
pub fn handle_input(window: &mut Window, offset_x: &mut f64, offset_y: &mut f64) {
    if window.is_key_down(Key::Up) && *offset_y > 0.0 {
        *offset_y -= MOVE_SPEED;
    }
    if window.is_key_down(Key::Down) && *offset_y < (MAP_SIZE_Y - SCREEN_HEIGHT / TILE_SIZE) as f64
    {
        *offset_y += MOVE_SPEED;
    }
    if window.is_key_down(Key::Left) && *offset_x > 0.0 {
        *offset_x -= MOVE_SPEED;
    }
    if window.is_key_down(Key::Right) && *offset_x < (MAP_SIZE_X - SCREEN_WIDTH / TILE_SIZE) as f64
    {
        *offset_x += MOVE_SPEED;
    }
}
