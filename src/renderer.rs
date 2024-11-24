use crate::map::Tile;

pub trait Renderer {
    fn render(
        &mut self,
        terrain: &Vec<Vec<Tile>>,
        shadows: &Vec<Vec<(bool, bool, bool, bool)>>,
        offset_x: f64,
        offset_y: f64,
    );
}
