use crate::{terrain::map::Viewport, tileset::TileAtlas};

pub trait Renderer {
    fn render(&mut self, viewport: &Viewport, tile_atlas: &TileAtlas, offset_x: f64, offset_y: f64);
}
