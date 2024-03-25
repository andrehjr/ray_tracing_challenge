use std::f64::consts::{FRAC_PI_3, PI};

use raytracer::tuple::Tuple;
use raytracer::vector;
use raytracer::*;
// use std::f64::consts::PI;

fn main() {
    let mut floor = ray::Sphere::init();
    floor.transform = matrix::Matrix::identity(4).scaling(10.0, 0.01, 10.0);
    let mut floor_material = material::Material::default();
    floor_material.color = color::Color::new(1.0, 0.9, 0.9);
    floor_material.specular = 0.0;
    floor.material = floor_material;

    let mut left_wall = ray::Sphere::init();
    left_wall.transform = matrix::Matrix::identity(4)
        .scaling(10.0, 0.01, 10.0)
        .rotation_x(PI / 2.0)
        .rotation_y(-PI / 4.0)
        .translation(0.0, 0.0, 5.0);

    let mut right_wall = ray::Sphere::init();
    right_wall.transform = matrix::Matrix::identity(4)
        .scaling(10.0, 0.01, 10.0)
        .rotation_x(PI / 2.0)
        .rotation_y(PI / 4.0)
        .translation(0.0, 0.0, 5.0);

    let mut middle = ray::Sphere::init();
    middle.transform = matrix::Matrix::identity(4).translation(-0.5, 1.0, 0.5);
    let mut middle_material = material::Material::default();
    middle_material.color = color::Color::new(0.1, 1.0, 0.5);
    middle_material.diffuse = 0.7;
    middle_material.specular = 0.3;
    middle.material = middle_material;

    let mut right = ray::Sphere::init();
    right.transform = matrix::Matrix::identity(4)
        .scaling(0.5, 0.5, 0.5)
        .translation(1.5, 0.5, -0.5);
    let mut right_material = material::Material::default();
    right_material.color = color::Color::new(0.5, 1.0, 0.1);
    right_material.diffuse = 0.7;
    right_material.specular = 0.3;
    right.material = right_material;

    let mut left = ray::Sphere::init();
    left.transform = matrix::Matrix::identity(4)
        .scaling(0.33, 0.33, 0.33)
        .translation(1.5, 0.33, -0.75);
    let mut left_material = material::Material::default();
    left_material.color = color::Color::new(1.0, 0.8, 0.1);
    left_material.diffuse = 0.7;
    left_material.specular = 0.3;
    left.material = left_material;

    let mut world = world::World::new();
    world.lights.push(light::Light {
        intensity: color::Color {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        },
        position: point!(-10, 10, -10),
    });

    let mut camera = camera::Camera::new(100, 50, FRAC_PI_3);
    camera.transform =
        matrix::view_transform(point!(0, 1.5, -5), point!(0, 1, 0), vector!(0, 1, 0));

    let canvas = camera::render(&camera, &world);

    let string = ppm::canvas_to_ppm(canvas);
    println!("{}", string);
}
