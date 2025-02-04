use image::ImageReader;
use std::collections::HashMap;

use crate::constants::terrain::TerrainType;

pub struct TileAtlas {
    pub texture: Vec<u32>,    // Pixel data for the tileset
    pub tile_width: usize,    // Width of a single tile
    pub tile_height: usize,   // Height of a single tile
    pub tiles_per_row: usize, // Number of tiles in a row
    pub tilesets: HashMap<TerrainType, TileOffsets>,
}

impl TileAtlas {
    pub fn new(path: &str, tile_width: usize, tile_height: usize) -> Self {
        let tileset_image = ImageReader::open(path)
            .expect("Failed to load tileset image")
            .decode()
            .expect("Failed to decode image")
            .to_rgba8();

        let tileset_width = tileset_image.width() as usize;
        let tiles_per_row = tileset_width / tile_width;

        let texture = tileset_image
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
            tile_width,
            tile_height,
            tiles_per_row,
            tilesets: HashMap::new(),
        }
    }

    pub fn get_tile_pixels(&self, tile_index: usize, terrain: &TerrainType) -> Vec<u32> {
        let tileset = self.tilesets.get(terrain).unwrap();
        let offset_y = tileset.starting_offset_y;
        let offset_x = tileset.starting_offset_x;

        let row = (tile_index / self.tiles_per_row) + offset_y;
        let col = (tile_index % self.tiles_per_row) + offset_x;

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

    pub fn register_tileset(
        &mut self,
        terrain_type: TerrainType,
        offset_y: usize,
        offset_x: usize,
    ) {
        self.tilesets
            .insert(terrain_type, TileOffsets::new(offset_x, offset_y));
    }
}

pub struct TileOffsets {
    pub starting_offset_x: usize, // Where this tileset starts in the atlas (in tiles along the X-axis)
    pub starting_offset_y: usize, // Where this tileset starts in the atlas (in tiles along the Y-axis)
}

impl TileOffsets {
    pub fn new(starting_offset_x: usize, starting_offset_y: usize) -> Self {
        Self {
            starting_offset_x,
            starting_offset_y,
        }
    }
}
