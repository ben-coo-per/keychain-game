use image::ImageReader;
use crate::characters::sprite::Sprite;
use crate::characters::Direction;
use crate::constants::device::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::constants::tiles::TILE_SIZE;

pub struct NPC {
    pub sprite: Sprite,
    pub movement_speed: u8,
    pub x: f64,
    pub y: f64,
}

pub type SpriteToRender = (Sprite, usize, usize);

impl NPC {
    pub fn new(path: &str, x: f64, y: f64, scale: u8) -> Self {
        let character_image = ImageReader::open(path)
            .expect("Failed to load character image")
            .decode()
            .expect("Failed to decode image")
            .to_rgba8();

        let width = character_image.width() as usize;
        let height = character_image.height() as usize;

        let texture = character_image
            .pixels()
            .map(|p| {
                // Adjust the byte order to match rendering expectations
                u32::from_le_bytes([
                    p[2], // Blue
                    p[1], // Green
                    p[0], // Red
                    p[3], // Alpha
                ])
            })
            .collect();

        Self {
            sprite: Sprite {
                texture,
                width,
                height,
                direction: Direction::Right,
                scale,
            },
            movement_speed: 1,
            x,
            y,
        }
    }

    pub fn flip(&mut self) {
        self.sprite.flip();
    }

    pub fn get_sprite_to_render(&mut self, offset_x: f64, offset_y: f64) -> Option<SpriteToRender> {
        /// Returns the sprite to render and the screen coordinates in the viewport
        let screen_x = ((self.x as f64 + offset_x) as isize) * TILE_SIZE as isize;
        let screen_y = ((self.y as f64 + offset_y) as isize) * TILE_SIZE as isize;

        // Check if the sprite is within the viewport, accounting for scale
        if screen_x + (self.sprite.width * self.sprite.scale as usize) as isize >= 0
            && screen_x < SCREEN_WIDTH as isize
            && screen_y + (self.sprite.height * self.sprite.scale as usize) as isize >= 0
            && screen_y < SCREEN_HEIGHT as isize
        {
            Some((self.sprite.clone(), screen_x as usize, screen_y as usize))
        } else {
            None
        }
    }
}
