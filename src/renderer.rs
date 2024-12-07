use crate::tileset::TileAtlas;

pub trait Renderer {
    fn render(&mut self, tiles_to_render: &Vec<Vec<[u8; 2]>>, tile_atlas: &TileAtlas);
}
