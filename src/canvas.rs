use crate::color::Color;

pub struct Canvas {
    pub height: usize,
    pub width: usize,
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn init(width: usize, height: usize) -> Canvas {
        let pixels = vec![vec![Color::new(0.0, 0.0, 0.0); width]; height];

        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> &Color {
        &self.pixels[y][x]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y][x] = color
    }
}
