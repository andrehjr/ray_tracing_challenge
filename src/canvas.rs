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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn test_canvas_new() {
        let canvas = Canvas::init(10, 20);

        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);

        assert_eq!(canvas.pixels.len(), 20);

        for line in canvas.pixels.iter() {
            assert_eq!(line.len(), 10);

            for pixel in line.iter() {
                assert_eq!(*pixel, Color::new(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn test_canvas_pixel_at() {
        let canvas = Canvas::init(10, 20);
        assert_eq!(*canvas.pixel_at(0, 0), Color::new(0.0, 0.0, 0.0))
    }

    #[test]
    fn test_canvas_write_pixel() {
        let mut canvas = Canvas::init(10, 20);
        let black = Color::new(0.0, 0.0, 0.0);
        let red = Color::new(1.0, 0.0, 0.0);

        assert_eq!(*canvas.pixel_at(0, 0), black);
        canvas.write_pixel(0, 0, red.clone());
        assert_eq!(*canvas.pixel_at(0, 0), red);
    }

    // #[test]
    // fn test_canvas_pixel_at_out_of_bounds() {
    //  let canvas = Canvas::init(10, 20);
    //   canvas.pixel_at(100, 100);
    // }
}
