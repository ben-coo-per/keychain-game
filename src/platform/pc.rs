use crate::{constants::*, map::Tile, renderer::Renderer};
use minifb::{Key, Window, WindowOptions};

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
    fn render(
        &mut self,
        terrain: &Vec<Vec<Tile>>,
        shadows: &Vec<Vec<(bool, bool, bool, bool)>>,
        offset_x: f64,
        offset_y: f64,
    ) {
        let mut buffer = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

        // Convert offsets to tile coordinates
        let tile_offset_x = offset_x as usize;
        let tile_offset_y = offset_y as usize;

        // Iterate over visible tiles
        for y in 0..(SCREEN_HEIGHT) {
            for x in 0..(SCREEN_WIDTH) {
                // Calculate map coordinates
                let map_x = tile_offset_x + x;
                let map_y = tile_offset_y + y;

                // Bounds check
                if map_y >= terrain.len() || map_x >= terrain[0].len() {
                    continue;
                }

                // Get the tile and shadow data
                let tile = &terrain[map_y][map_x];
                let shadow = shadows[map_y][map_x];

                // Calculate screen-space coordinates for the tile
                let start_x = x * TILE_SIZE;
                let start_y = y * TILE_SIZE;

                // Fill the buffer with the tile's color and shadow edges
                render_with_shadows_into_buffer(
                    tile,
                    shadow,
                    &mut buffer,
                    start_x,
                    start_y,
                    SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                );
            }
        }

        // Render the buffer to the screen
        self.window
            .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}

/// Renders a tile and its shadows into the buffer at the given screen position
fn render_with_shadows_into_buffer(
    tile: &Tile,
    shadow: (bool, bool, bool, bool),
    buffer: &mut Vec<u32>,
    start_x: usize,
    start_y: usize,
    screen_width: usize,
    screen_height: usize,
) {
    let shadow_color = 0x333333; // Dark gray for shadows

    // Fill tile color
    for ty in 0..TILE_SIZE {
        for tx in 0..TILE_SIZE {
            let px = start_x + tx;
            let py = start_y + ty;

            if px < screen_width && py < screen_height {
                buffer[py * screen_width + px] = tile.terrain_color;
            }
        }
    }

    // Draw shadow edges
    let (top, right, bottom, left) = shadow;

    if top {
        for tx in 0..TILE_SIZE {
            let px = start_x + tx;
            let py = start_y;
            if px < screen_width && py < screen_height {
                buffer[py * screen_width + px] = shadow_color;
            }
        }
    }

    if right {
        for ty in 0..TILE_SIZE {
            let px = start_x + TILE_SIZE - 1;
            let py = start_y + ty;
            if px < screen_width && py < screen_height {
                buffer[py * screen_width + px] = shadow_color;
            }
        }
    }

    if bottom {
        for tx in 0..TILE_SIZE {
            let px = start_x + tx;
            let py = start_y + TILE_SIZE - 1;
            if px < screen_width && py < screen_height {
                buffer[py * screen_width + px] = shadow_color;
            }
        }
    }

    if left {
        for ty in 0..TILE_SIZE {
            let px = start_x;
            let py = start_y + ty;
            if px < screen_width && py < screen_height {
                buffer[py * screen_width + px] = shadow_color;
            }
        }
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
