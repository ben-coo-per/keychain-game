use crate::{
    constants::{
        device::SCREEN_HEIGHT, device::SCREEN_WIDTH, experience::*, map_gen::*, tiles::TILE_SIZE,
    },
    map::Tile,
    renderer::Renderer,
    tileset::Tileset,
};
use minifb::{Key, Window, WindowOptions};

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
    fn render(
        &mut self,
        terrain: &Vec<Vec<Tile>>,
        tileset: &Tileset,
        offset_x: f64,
        offset_y: f64,
    ) {
        let mut buffer = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

        let tile_offset_x = offset_x as usize;
        let tile_offset_y = offset_y as usize;

        for y in 0..(SCREEN_HEIGHT / tileset.tile_height) {
            for x in 0..(SCREEN_WIDTH / tileset.tile_width) {
                let map_x = tile_offset_x + x;
                let map_y = tile_offset_y + y;

                if map_y >= terrain.len() || map_x >= terrain[0].len() {
                    continue;
                }

                let tile = &terrain[map_y][map_x];
                let tile_index = tile.tile_type.to_index();
                let tile_pixels = tileset.get_tile_pixels(tile_index);

                for ty in 0..tileset.tile_height {
                    for tx in 0..tileset.tile_width {
                        let px = x * tileset.tile_width + tx;
                        let py = y * tileset.tile_height + ty;

                        if px < SCREEN_WIDTH && py < SCREEN_HEIGHT {
                            buffer[py * SCREEN_WIDTH + px] =
                                tile_pixels[ty * tileset.tile_width + tx];
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
