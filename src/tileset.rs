use image::ImageReader;

pub struct Tileset {
    pub texture: Vec<u32>,    // Pixel data for the tileset
    pub tile_width: usize,    // Width of a single tile
    pub tile_height: usize,   // Height of a single tile
    pub tiles_per_row: usize, // Number of tiles in a row
}

impl Tileset {
    pub fn new(path: &str, tile_width: usize, tile_height: usize) -> Self {
        let tileset_image = ImageReader::open(path)
            .expect("Failed to open tileset")
            .decode()
            .expect("Failed to decode image")
            .to_rgba8();

        let tileset_width = tileset_image.width() as usize;
        let tiles_per_row = tileset_width / tile_width;

        let texture = tileset_image
            .pixels()
            .map(|p| u32::from_le_bytes([p[0], p[1], p[2], p[3]]))
            .collect();

        Self {
            texture,
            tile_width,
            tile_height,
            tiles_per_row,
        }
    }

    pub fn get_tile_pixels(&self, tile_index: usize) -> Vec<u32> {
        let row = tile_index / self.tiles_per_row;
        let col = tile_index % self.tiles_per_row;

        let mut tile_pixels = vec![];

        for y in 0..self.tile_height {
            for x in 0..self.tile_width {
                let tileset_x = col * self.tile_width + x;
                let tileset_y = row * self.tile_height + y;
                let pixel_index = tileset_y * (self.tile_width * self.tiles_per_row) + tileset_x;

                tile_pixels.push(self.texture[pixel_index]);
            }
        }

        tile_pixels
    }
}
