use image::ImageReader;
use crate::characters::character::Sprite;
use crate::characters::Direction;

pub struct NPC {
    pub sprite: Sprite,
    pub movement_speed: f64,
    pub x_pos: f64,
    pub y_pos: f64,
}

impl NPC {
    pub fn new(path: &str) -> Self {
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
            x_pos: 0.0,
            y_pos: 0.0,
        }
    }

    pub fn flip(&mut self) {
        self.sprite.flip();
    }
}
