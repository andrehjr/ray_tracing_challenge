use std::ops::{Add, Mul, Sub};

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

impl Add<Color> for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sub<Color> for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, factor: f64) -> Self {
        Self {
            red: self.red * factor,
            green: self.green * factor,
            blue: self.blue * factor,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Color) -> Self {
        Self {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

#[macro_export]
macro_rules! color {
    ($red:expr, $green: expr, $blue: expr) => {
        color::Color {
            red: $red as f64,
            green: $green as f64,
            blue: $blue as f64,
        }
    };
}
