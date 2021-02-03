#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub const EPSILON: f64 = 0.001;

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        let red_diff = (self.red - other.red).abs() <= EPSILON;
        let green_diff = (self.green - other.green).abs() <= EPSILON;
        let blue_diff = (self.blue - other.blue).abs() <= EPSILON;

        red_diff && green_diff && blue_diff
    }
}

impl Color {
    pub fn add(&self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }

    pub fn sub(&self, other: Color) -> Color {
        Color {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }

    pub fn multiply(&self, factor: f64) -> Color {
        Color {
            red: self.red * factor,
            green: self.green * factor,
            blue: self.blue * factor,
        }
    }

    pub fn hadamard_product(&self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

pub fn color(red: f64, green: f64, blue: f64) -> Color {
    Color { red, green, blue }
}
