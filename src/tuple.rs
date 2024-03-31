use crate::matrix::*;
use crate::{matrix, EPSILON};
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl_op_ex!(+|a: &Tuple, b: &Tuple| -> Tuple {
    Tuple {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
        w: a.w + b.w,
    }
});

impl_op_ex!(-|a: &Tuple, b: &Tuple| -> Tuple {
    Tuple {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
        w: a.w - b.w,
    }
});

impl_op_ex!(*|a: &Tuple, b: &f64| -> Tuple {
    Tuple {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
        w: a.w,
    }
});

impl_op_ex!(*|a: &Tuple, b: &Tuple| -> f64 { a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w });

impl Tuple {
    pub fn negate(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

    pub fn magnitude(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn norm(&self) -> Self {
        self * (1.0 / self.magnitude())
    }

    pub fn reflect(&self, normal: Tuple) -> Tuple {
        self - normal * 2.0 * (self * normal)
    }

    pub fn cross_product(&self, other: Tuple) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: 0.0,
        }
    }

    pub fn translation(&self, x: f64, y: f64, z: f64) -> Self {
        let transform = matrix![ 1.0, 0.0, 0.0, x;
                                 0.0, 1.0, 0.0, y;
                                 0.0, 0.0, 1.0, z;
                                 0.0, 0.0, 0.0, 1.0];
        transform * self
    }

    pub fn scaling(&self, x: f64, y: f64, z: f64) -> Self {
        let transform = matrix![   x, 0.0, 0.0, 0.0;
                                 0.0,   y, 0.0, 0.0;
                                 0.0, 0.0,   z, 0.0;
                                 0.0, 0.0, 0.0, 1.0];
        transform * self
    }

    pub fn rotation_x(&self, x: f64) -> Self {
        let transform = matrix![ 1.0, 0.0, 0.0, 0.0;
                                 0.0, x.cos(), -x.sin(), 0.0;
                                 0.0, x.sin(),  x.cos(), 0.0;
                                 0.0, 0.0, 0.0, 1.0];
        transform * self
    }

    pub fn rotation_y(&self, x: f64) -> Self {
        let transform = matrix![ x.cos(), 0.0, x.sin(), 0.0;
                                 0.0, 1.0, 0.0, 0.0;
                                 -x.sin(), 0.0, x.cos(), 0.0;
                                 0.0, 0.0, 0.0, 1.0];
        transform * self
    }

    pub fn rotation_z(&self, x: f64) -> Self {
        let transform = matrix![ x.cos(), -x.sin(), 0.0, 0.0;
                                 x.sin(), x.cos(), 0.0, 0.0;
                                 0.0, 0.0, 1.0, 0.0;
                                 0.0, 0.0, 0.0, 1.0];
        transform * self
    }

    pub fn skew(&self, x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Self {
        let transform = matrix![ 1.0, x_y, x_z, 0.0;
                                  y_x, 1.0, y_z, 0.0;
                                  z_x, z_y, 1.0, 0.0;
                                  0.0, 0.0, 0.0, 1.0];

        transform * self
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        let x_diff = (self.x - other.x).abs() <= EPSILON;
        let y_diff = (self.y - other.y).abs() <= EPSILON;
        let z_diff = (self.z - other.z).abs() <= EPSILON;

        x_diff && y_diff && z_diff && self.w == other.w
    }
}

#[macro_export]
macro_rules! point {
    ($x:expr, $y: expr, $z: expr) => {
        Tuple {
            x: $x as f64,
            y: $y as f64,
            z: $z as f64,
            w: 1.0,
        }
    };
}

#[macro_export]
macro_rules! vector {
    ($x: expr, $y: expr, $z: expr) => {
        Tuple {
            x: $x as f64,
            y: $y as f64,
            z: $z as f64,
            w: 0.0,
        }
    };
}
