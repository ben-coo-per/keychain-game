use crate::constants::*;
use rand::Rng;

pub struct Feature {
    pub x: f64,        // World x-coordinate
    pub y: f64,        // World y-coordinate
    pub width: usize,  // Width in pixels
    pub height: usize, // Height in pixels
    pub color: u32,    // Color of the feature
}

impl Feature {
    // Generate random features
    pub fn generate_random_features(count: usize, tile_size: usize) -> Vec<Self> {
        let mut rng = rand::thread_rng();
        let mut features = Vec::new();
        for _ in 0..count {
            features.push(Feature {
                x: rng.gen_range(0.0..10.0),
                y: rng.gen_range(0.0..10.0),
                width: tile_size * 2,
                height: tile_size * 3,
                color: 0x00FF00,
            });
        }
        features
    }

    // Render the feature onto the buffer
    pub fn render(&self, buffer: &mut Vec<u32>, offset_x: f64, offset_y: f64) {
        let feature_x = ((self.x - offset_x) * TILE_SIZE as f64) as isize;
        let feature_y = ((self.y - offset_y) * TILE_SIZE as f64) as isize;

        for fy in 0..self.height {
            for fx in 0..self.width {
                let px = feature_x + fx as isize;
                let py = feature_y + fy as isize;

                if px >= 0 && px < WIDTH as isize && py >= 0 && py < HEIGHT as isize {
                    buffer[py as usize * WIDTH + px as usize] = self.color;
                }
            }
        }
    }
}
