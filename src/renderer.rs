use crate::{character::Character, constants::terrain::TERRAIN_TYPE_COUNT, tileset::TileAtlas};

pub trait Renderer {
    fn render(
        &mut self,
        tiles_to_render: &Vec<Vec<[u8; TERRAIN_TYPE_COUNT]>>,
        tile_atlas: &TileAtlas,
        character: &Character,
    );
}
