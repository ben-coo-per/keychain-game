use crate::map::Tile;
use crate::tileset::Tileset;

pub trait Renderer {
    fn render(&mut self, terrain: &Vec<Vec<Tile>>, tileset: &Tileset, offset_x: f64, offset_y: f64);
}
