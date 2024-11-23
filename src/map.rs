use crate::constants::*;
use noise::{NoiseFn, Perlin};

pub fn generate_map(buffer: &mut Vec<u32>, offset_x: f64, offset_y: f64) {
    let perlin = Perlin::new(123123123);

    for y in 0..(HEIGHT / TILE_SIZE) {
        for x in 0..(WIDTH / TILE_SIZE) {
            let nx = (x as f64 + offset_x) / (WIDTH / TILE_SIZE) as f64;
            let ny = (y as f64 + offset_y) / (HEIGHT / TILE_SIZE) as f64;

            let noise_value = perlin.get([nx, ny]);
            let tile_color = if noise_value < -0.5 {
                0x000000 // Black: Deep terrain
            } else if noise_value < 0.0 {
                0x555555 // Dark Gray: Low terrain
            } else if noise_value < 0.5 {
                0xAAAAAA // Light Gray: Elevated terrain
            } else {
                0xFFFFFF // White: High terrain
            };

            for ty in 0..TILE_SIZE {
                for tx in 0..TILE_SIZE {
                    let px = x * TILE_SIZE + tx;
                    let py = y * TILE_SIZE + ty;
                    if px < WIDTH && py < HEIGHT {
                        buffer[py * WIDTH + px] = tile_color;
                    }
                }
            }
        }
    }
}
