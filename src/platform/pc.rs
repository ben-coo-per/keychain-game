use crate::{
    constants::{
        device::{SCREEN_HEIGHT, SCREEN_WIDTH},
        experience::*,
        tiles::TILE_SIZE,
    },
    map::generate_viewport_tiles,
    renderer::Renderer,
    tileset::Tileset,
};
use minifb::{Key, Window, WindowOptions};
use noise::{Fbm, Perlin};

pub fn handle_input(window: &mut Window, offset_x: &mut f64, offset_y: &mut f64) {
    if window.is_key_down(Key::Up) {
        *offset_y -= MOVE_SPEED;
    }
    if window.is_key_down(Key::Down) {
        *offset_y += MOVE_SPEED;
    }
    if window.is_key_down(Key::Left) {
        *offset_x -= MOVE_SPEED;
    }
    if window.is_key_down(Key::Right) {
        *offset_x += MOVE_SPEED;
    }

    // round offset values to one decimal place
    *offset_x = (*offset_x * 10.0).round() / 10.0;
    *offset_y = (*offset_y * 10.0).round() / 10.0;
}

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
    fn render(&mut self, perlin: &Fbm<Perlin>, tileset: &Tileset, offset_x: f64, offset_y: f64) {
        // Create an empty buffer to store the pixel data for the window
        let mut buffer = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

        let current_view = generate_viewport_tiles(&perlin, offset_x, offset_y);
        assert_eq!(
            current_view.len(),
            (SCREEN_WIDTH / TILE_SIZE) * (SCREEN_HEIGHT / TILE_SIZE),
            "Mismatch in current_view size"
        );

        // loop through tiles in current_view and add them to the buffer
        for (i, tile) in current_view.iter().enumerate() {
            let tile_pixels = tileset.get_tile_pixels(tile.tile_type.to_index());
            let tiles_per_row = SCREEN_WIDTH / TILE_SIZE;
            let tile_x = (i % tiles_per_row) * TILE_SIZE;
            let tile_y = (i / tiles_per_row) * TILE_SIZE;

            assert_eq!(
                tile_pixels.len(),
                TILE_SIZE * TILE_SIZE,
                "Tile pixel data size mismatch"
            );

            // merge the tile pixels with the buffer
            for y in 0..TILE_SIZE {
                for x in 0..TILE_SIZE {
                    if tile_x + x < SCREEN_WIDTH && tile_y + y < SCREEN_HEIGHT {
                        let pixel_index = (tile_y + y) * SCREEN_WIDTH + tile_x + x;
                        buffer[pixel_index] = tile_pixels[y * TILE_SIZE + x];
                    }
                }
            }
        }

        self.window
            .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}
