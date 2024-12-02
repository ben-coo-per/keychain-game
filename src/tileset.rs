use std::collections::HashMap;

use image::ImageReader;

#[derive(Debug, Eq, PartialEq, Hash)]
enum TerrainTypes {
    Grass=0,
    Dirt=1,
}

// This defines how tiles are laid out in the tileset
enum Corner {
    TopLeft = 0b1000,
    TopRight = 0b0100,
    BottomLeft = 0b0010,
    BottomRight = 0b0001,
}

pub struct TileAtlas  {
    pub texture: Vec<u32>,    // Pixel data for the tileset
    pub tile_width: usize,    // Width of a single tile
    pub tile_height: usize,   // Height of a single tile
    pub tiles_per_row: usize, // Number of tiles in a row
    pub single_tilesets: HashMap<TerrainTypes, &'static Tileset>,
    pub edge_tilesets: HashMap<(TerrainTypes, TerrainTypes), &'static Tileset>,
}
impl TileAtlas {
    pub const fn new(path: &str, tile_width: usize, tile_height: usize) -> Self {
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

    pub fn get_tile_pixels(&self, row: uszize, col: usize) -> Vec<u32> {
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

    pub fn get_tile_pixels(&self, tile_index: usize) -> Vec<u32> {
        let row = tile_index / self.tiles_per_row;
        let col = tile_index % self.tiles_per_row;
        self.get_tile_pixels(row, col)
    }


    pub fn get_tile_pixels(&self, tile: Tile) -> Vec<u32> {
        let tile_index = match self.corners {
            0b0000 => 0,
            0b0001 => 1,
            0b0010 => 2,
            0b0011 => 3,
            0b0100 => 4,
            0b0101 => 5,
            0b0110 => 6,
            0b0111 => 7,
            0b1000 => 8,
            0b1001 => 9,
            0b1010 => 10,
            0b1011 => 11,
            0b1100 => 12,
            0b1101 => 13,
            0b1110 => 14,
            0b1111 => 15,
            _ => panic!("Invalid corner configuration"),
        };
    }
    pub fn register_tileset(&mut self, terrain_type: TerrainTypes, secondary_terrain_type: TerrainTypes, tileset: Tileset) {
        self.single_tilesets.insert(terrain_type, &tileset);
        self.single_tilesets.insert((terrain_type, secondary_terrain_type), &tileset);
    }
}

pub struct Tileset {
    pub atlas: &'static TileAtlas,
    pub tile_offset_x: usize, // Where this tileset starts in the atlas (in tiles along the X-axis)
    pub tile_offset_y: usize, // Where this tileset starts in the atlas (in tiles along the Y-axis)
}

impl Tileset {
    pub fn new(atlas: &'static TileAtlas, tile_offset_x: usize, tile_offset_y: usize) -> Self {
        Self { atlas, tile_offset_x, tile_offset_y }
    }

    pub fn get_tile_pixels(&self, tile_index: usize) -> Vec<u32> {
        self.atlas.get_tile_pixels(tile_index + self.tile_offset_x * self.atlas.tiles_per_row)
    }
}


// Create global Tile Atlas

pub const TILESET_PATH: &str = "assets/tileset.png";
pub const TILE_SIZE: usize = 32;

pub let mut TILE_ATLAS: TileAtlas = TileAtlas::new(TILESET_PATH, TILE_SIZE, TILE_SIZE);




//  Initialize the tilesets

// Register the tilesets


struct Tile {
    terrain_type: TerrainTypes,
    secondary_terrain_type: Option<TerrainTypes>,
    corners: u8
}

impl Tile {
    pub fn new(terrain_type: TerrainTypes, secondary_terrain_type: Option<TerrainTypes>, corners: u8) -> Self {
        Self { terrain_type, secondary_terrain_type, corners }
    }
}