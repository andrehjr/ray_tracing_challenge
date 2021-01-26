#[derive(Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Debug, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Tuple {
    pub fn add(&self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    pub fn sub(&self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }

    pub fn negate(&self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

    pub fn multiply(&self, factor: f64) -> Tuple {
        Tuple {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
            w: self.w,
        }
    }

    pub fn magnitude(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn norm(&self) -> Tuple {
        self.multiply(1.0 / self.magnitude())
    }

    pub fn dot_product(&self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }

    pub fn cross_product(&self, other: Tuple) -> Tuple {
        Tuple {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: 0.0,
        }
    }
}

pub const EPSILON: f64 = 0.001;

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        let x_diff = (self.x - other.x).abs() <= EPSILON;
        let y_diff = (self.y - other.y).abs() <= EPSILON;
        let z_diff = (self.z - other.z).abs() <= EPSILON;

        x_diff && y_diff && z_diff
    }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple {
        x: x,
        y: y,
        z: z,
        w: 1.0,
    }
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple {
        x: x,
        y: y,
        z: z,
        w: 0.0,
    }
}

pub fn color(red: f64, green: f64, blue: f64) -> Color {
    Color {
        red: red,
        green: green,
        blue: blue,
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

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        let red_diff = (self.red - other.red).abs() <= EPSILON;
        let green_diff = (self.green - other.green).abs() <= EPSILON;
        let blue_diff = (self.blue - other.blue).abs() <= EPSILON;

        red_diff && green_diff && blue_diff
    }
}

pub struct Canvas {
    pub height: usize,
    pub width: usize,
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn init(width: usize, height: usize) -> Canvas {
        let pixels = vec![vec![color(0.0, 0.0, 0.0); width]; height];

        Canvas {
            width: width,
            height: height,
            pixels: pixels,
        }
    }
}
