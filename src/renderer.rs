use crate::tileset::Tileset;
use noise::{Fbm, Perlin};

pub trait Renderer {
    fn render(&mut self, perlin: &Fbm<Perlin>, tileset: &Tileset, offset_x: f64, offset_y: f64);
}
