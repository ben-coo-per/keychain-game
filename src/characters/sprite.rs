use image::ImageReader;
use crate::characters::Direction;

#[derive(Clone, Debug)]
pub struct Sprite {
    pub texture: Vec<u32>,
    pub width: usize,
    pub height: usize,
    pub direction: Direction,
    pub scale: u8,
}

impl Sprite {
    pub fn new(path: &str, scale: u8 ) -> Self {
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
            texture,
            width,
            height,
            direction: Direction::Right,
            scale,
        }
    }

    pub fn flip(&mut self) {
        let mut flipped_texture = vec![0; self.texture.len()];
        for y in 0..self.height {
            for x in 0..self.width {
                let src_index = y * self.width + x;
                let dest_index = y * self.width + (self.width - 1 - x);
                flipped_texture[dest_index] = self.texture[src_index];
            }
        }
        self.texture = flipped_texture;
    }
}