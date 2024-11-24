use crate::map::Tile;

pub trait Renderer {
    fn render(
        &mut self,
        terrain: &Vec<Vec<Tile>>,
        shadows: &Vec<Vec<Vec<(usize, usize)>>>,
        textures: &Vec<Vec<Vec<(usize, usize)>>>,
        offset_x: f64,
        offset_y: f64,
    );
}
