use crate::map::Tile;

pub trait Renderer {
    fn render(&mut self, map: &Vec<Vec<Tile>>, offset_x: f64, offset_y: f64);
}
