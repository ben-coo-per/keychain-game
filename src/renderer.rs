use crate::tileset::Tileset;
use noise::Perlin;

pub trait Renderer {
    fn render(&mut self, perlin: &Perlin, tileset: &Tileset, offset_x: f64, offset_y: f64);
}
