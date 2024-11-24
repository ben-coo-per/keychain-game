use crate::{constants::*, map::Tile, renderer::Renderer};
use minifb::{Window, WindowOptions};

pub struct PCRenderer {
    pub window: Window,
}

impl PCRenderer {
    pub fn new() -> Self {
        let window = Window::new(
            "Game Map",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            WindowOptions::default(),
        )
        .expect("Failed to create window");
        Self { window }
    }
}

impl Renderer for PCRenderer {
    fn render(&mut self, map: &Vec<Vec<Tile>>, offset_x: f64, offset_y: f64) {
        let mut buffer = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

        for y in 0..(SCREEN_HEIGHT / TILE_SIZE) {
            for x in 0..(SCREEN_WIDTH / TILE_SIZE) {
                let map_x = (x as f64 + offset_x) as isize;
                let map_y = (y as f64 + offset_y) as isize;

                if map_x < 0
                    || map_y < 0
                    || map_y as usize >= map.len()
                    || map_x as usize >= map[0].len()
                {
                    continue;
                }

                let tile = map[map_y as usize][map_x as usize];

                for ty in 0..TILE_SIZE {
                    for tx in 0..TILE_SIZE {
                        let px = x * TILE_SIZE + tx;
                        let py = y * TILE_SIZE + ty;
                        if px < SCREEN_WIDTH && py < SCREEN_HEIGHT {
                            buffer[py * SCREEN_WIDTH + px] = tile.terrain_color;
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

pub fn handle_input(window: &mut Window, offset_x: &mut f64, offset_y: &mut f64) {
    if window.is_key_down(minifb::Key::Up) && offset_y > &mut 0.0 {
        *offset_y -= MOVE_SPEED;
    }
    if window.is_key_down(minifb::Key::Down)
        && *offset_y < (MAP_SIZE_Y - SCREEN_HEIGHT / TILE_SIZE) as f64
    {
        *offset_y += MOVE_SPEED;
    }
    if window.is_key_down(minifb::Key::Left) && offset_x > &mut 0.0 {
        *offset_x -= MOVE_SPEED;
    }
    if window.is_key_down(minifb::Key::Right)
        && *offset_x < (MAP_SIZE_X - SCREEN_WIDTH / TILE_SIZE) as f64
    {
        *offset_x += MOVE_SPEED;
    }
}
