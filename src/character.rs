use image::ImageReader;

pub struct Character {
    pub texture: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

impl Character {
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
            texture,
            width,
            height,
        }
    }
}
