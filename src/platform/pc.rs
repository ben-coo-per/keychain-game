use crate::{

    characters::{
        Direction,
        sprite::{Sprite},
    },
    constants::{
        device::{SCREEN_HEIGHT, SCREEN_WIDTH},
        experience::MOVE_SPEED,
        terrain::{ALL_TERRAIN_TYPES, TERRAIN_TYPE_COUNT},
        tiles::TILE_SIZE,
    },
    renderer::Renderer,
    tileset::TileAtlas,
};
use minifb::{Key, Window, WindowOptions};
use crate::characters::npc::SpriteToRender;

pub fn handle_input(
    window: &mut Window,
    offset_x: &mut f64,
    offset_y: &mut f64,
    view_changed: &mut bool,
    character: &mut Sprite,
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
        if let Direction::Left = character.direction {
            character.direction = Direction::Right;
            character.flip();
        }
    }
    if window.is_key_down(Key::Right) {
        *offset_x -= MOVE_SPEED as f64;
        *view_changed = true;
        if let Direction::Right = character.direction {
            character.direction = Direction::Left;
            character.flip();
        }
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
            "MORE WILD",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            WindowOptions::default(),
        )
        .expect("Failed to create window");
        Self { window }
    }


   fn render_sprite(sprite: &Sprite, buffer: &mut Vec<u32>, character_x: usize, character_y: usize) {
    for ty in 0..sprite.height {
        for tx in 0..sprite.width {
            let pixel_value = sprite.texture[ty * sprite.width + tx];
            let alpha = (pixel_value >> 24) & 0xFF; // Extract the alpha value
            if alpha != 0 {
                for sy in 0..sprite.scale {
                    for sx in 0..sprite.scale {
                        let buffer_x = character_x as isize + tx as isize * sprite.scale as isize + sx as isize;
                        let buffer_y = character_y as isize + ty as isize * sprite.scale as isize + sy as isize;
                        if buffer_x >= 0 && buffer_y >= 0 && buffer_x < SCREEN_WIDTH as isize && buffer_y < SCREEN_HEIGHT as isize {
                            let buffer_index = buffer_y as usize * SCREEN_WIDTH + buffer_x as usize;
                            buffer[buffer_index] = pixel_value;
                        }
                    }
                }
            }
        }
    }
}

    fn render_tilecake(tiles_to_render: &Vec<Vec<[u8; 5]>>, tile_atlas: &TileAtlas, buffer: &mut Vec<u32>) {
        /// Loops through the `TileCake` structure to render layers from bottom to top

        for (y, row) in tiles_to_render.iter().enumerate() {
            for (x, tile_cake) in row.iter().enumerate() {
                let tile_screen_x = x * TILE_SIZE;
                let tile_screen_y = y * TILE_SIZE;

                for (terrain_layer_index, tile_index) in tile_cake.iter().enumerate() {
                    // Get the world type for the current layer
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
    }
}

impl Renderer for PCRenderer {
    fn render(
        &mut self,
        tiles_to_render: &Vec<Vec<[u8; TERRAIN_TYPE_COUNT]>>,
        tile_atlas: &TileAtlas,
        character: &Sprite,
        sprite_to_render: &Vec<SpriteToRender>,
    ) {
        // Create an empty buffer to store the pixel data for the window
        let mut buffer = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

        Self::render_tilecake(tiles_to_render, tile_atlas, &mut buffer);

        // Render the main character at the center of the viewport
        let character_x = (SCREEN_WIDTH - character.width) / 2;
        let character_y = (SCREEN_HEIGHT - character.height) / 2;

        // Order sprites & main character rendering by the highest point calculated by its position & scale
        let mut sprites_to_render = sprite_to_render.clone();
        sprites_to_render.push((character.clone(), character_x, character_y)); // Include the main character

        sprites_to_render.sort_by(|a, b| {
            let key_a = (a.2 as isize + ((a.0.height as isize * a.0.scale as isize) * 2 / 3));
            let key_b = (b.2 as isize + ((b.0.height as isize * b.0.scale as isize) * 2 / 3));
            key_a.cmp(&key_b)
        });
        // Render sprites in sorted order
        for sprite in &sprites_to_render {
            Self::render_sprite(&sprite.0, &mut buffer, sprite.1, sprite.2);
        }


        // Update the window with the rendered buffer
        self.window
            .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}
