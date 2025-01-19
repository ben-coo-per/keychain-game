use image::ImageReader;
use crate::characters::sprite::Sprite;
use crate::characters::Direction;
use crate::constants::device::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::constants::tiles::TILE_SIZE;

pub struct NPC {
    pub sprite: Sprite,
    pub movement_speed: f64,
    pub x: f64,
    pub y: f64,
}

pub type SpriteToRender = (Sprite, usize, usize);

impl NPC {
    pub fn new(path: &str, x: f64, y: f64) -> Self {
        let character_image = ImageReader::open(path)
            .expect("Failed to load character image")
            .decode()
            .expect("Failed to decode image")
            .to_rgba8();

        let width = character_image.width() as usize;
        let height = character_image.height() as usize;

        let texture = character_image
            .pixels()
            .map(|p| u32::from_le_bytes([p[0], p[1], p[2], p[3]]))
            .collect();

        Self {
            sprite: Sprite {
                texture,
                width,
                height,
                direction: Direction::Right,
            },
            movement_speed: 1.0,
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

        // Check if the sprite is within the viewport
        if screen_x >= 0
            && (screen_x as usize) < SCREEN_WIDTH
            && screen_y >= 0
            && (screen_y as usize) < SCREEN_HEIGHT
        {
            Some((self.sprite.clone(), screen_x as usize, screen_y as usize))
        } else {
            None
        }
    }
}
