use crate::{characters::{sprite::Sprite}, constants::terrain::TERRAIN_TYPE_COUNT, tileset::TileAtlas};
use crate::characters::npc::SpriteToRender;

pub trait Renderer {
    fn render(
        &mut self,
        tiles_to_render: &Vec<Vec<[u8; TERRAIN_TYPE_COUNT]>>,
        tile_atlas: &TileAtlas,
        character: &Sprite,
        sprite_to_render: &Vec<SpriteToRender>,
    );
}
