use crate::{
    constants::{
        device::{SCREEN_HEIGHT, SCREEN_WIDTH},
        experience::*,
        tiles::{TerrainType, TILE_SIZE},
    },
    renderer::Renderer,
    terrain::map::generate_terrain_grid,
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
        // // Create an empty buffer to store the pixel data for the window
        let mut buffer = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

        // self.window
        //     .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
        //     .unwrap();

        // render Grid A that has red lines spaced 32 pixels apart in the x and y directions
        for x in (0..SCREEN_WIDTH).step_by(TILE_SIZE) {
            for y in (0..SCREEN_HEIGHT).step_by(1) {
                let pixel_index = y * SCREEN_WIDTH + x;
                buffer[pixel_index] = 0xFF0000;
            }
        }
        for y in (0..SCREEN_HEIGHT).step_by(TILE_SIZE) {
            for x in (0..SCREEN_WIDTH).step_by(1) {
                let pixel_index = y * SCREEN_WIDTH + x;
                buffer[pixel_index] = 0xFF0000;
            }
        }

        let terrain = generate_terrain_grid(&perlin, offset_x, offset_y);

        // render terrain numerical values within the grid
        // for Grass (0), render 1 dot in white
        // for Dirt (1), render 3 dots in white

        for y in (6..SCREEN_HEIGHT).step_by(TILE_SIZE) {
            for x in (6..SCREEN_WIDTH).step_by(TILE_SIZE) {
                let pixel_index = y * SCREEN_WIDTH + x;
                match terrain[y][x] {
                    TerrainType::Grass => {
                        buffer[pixel_index] = 0xFFFFFF;
                    }
                    TerrainType::Dirt => {
                        buffer[pixel_index] = 0xFFFFFF;
                        buffer[pixel_index + 1] = 0xFFFFFF;
                        buffer[pixel_index + 2] = 0xFFFFFF;
                    }
                    _ => {}
                }
            }
        }

        // render Grid B that has blue lines spaced 32 pixels apart in the x and y directions and is offset by 16 pixels in both directions
        for x in (16..SCREEN_WIDTH).step_by(TILE_SIZE) {
            for y in (0..SCREEN_HEIGHT).step_by(1) {
                let pixel_index = y * SCREEN_WIDTH + x;
                buffer[pixel_index] = 0x0000FF;
            }
        }
        for y in (16..SCREEN_HEIGHT).step_by(TILE_SIZE) {
            for x in (0..SCREEN_WIDTH).step_by(1) {
                let pixel_index = y * SCREEN_WIDTH + x;
                buffer[pixel_index] = 0x0000FF;
            }
        }

        self.window
            .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}
