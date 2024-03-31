#[macro_use]
extern crate impl_ops;

pub const EPSILON: f64 = 0.001;

pub mod camera;
pub mod canvas;
pub mod color;
pub mod light;
pub mod material;
pub mod matrix;
pub mod ppm;
pub mod ray;
pub mod tuple;
pub mod world;
