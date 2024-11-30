use crate::{terrain::map::Viewport, tileset::Tileset};

pub trait Renderer {
    fn render(&mut self, viewport: &Viewport, tileset: &Tileset, offset_x: f64, offset_y: f64);
}
