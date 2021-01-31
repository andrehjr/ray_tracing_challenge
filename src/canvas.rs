use crate::color::{color, Color};

pub struct Canvas {
    pub height: usize,
    pub width: usize,
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn init(width: usize, height: usize) -> Canvas {
        let pixels = vec![vec![color(0.0, 0.0, 0.0); width]; height];

        Canvas {
            width,
            height,
            pixels,
        }
    }
}
