use std::f64::consts::{FRAC_PI_3, PI};

use raytracer::tuple::Tuple;
use raytracer::vector;
use raytracer::*;
// use std::f64::consts::PI;

fn main() {
    let mut floor = ray::Sphere::init();
    floor.transform = matrix::Matrix::identity(4).scaling(10.0, 0.01, 10.0);
    let floor_material = material::Material {
        color: color::Color::new(1.0, 0.9, 0.9),
        specular: 0.0,
        ..material::Material::default()
    };
    floor.material = floor_material;

    let mut left_wall = ray::Sphere::init();
    left_wall.transform = matrix::Matrix::identity(4)
        .scaling(10.0, 0.01, 10.0)
        .rotation_x(PI / 2.0)
        .rotation_y(-PI / 4.0)
        .translation(0.0, 0.0, 5.0);
    left_wall.material = floor_material;

    let mut right_wall = ray::Sphere::init();
    right_wall.transform = matrix::Matrix::identity(4)
        .scaling(10.0, 0.01, 10.0)
        .rotation_x(PI / 2.0)
        .rotation_y(PI / 4.0)
        .translation(0.0, 0.0, 5.0);
    right_wall.material = floor_material;

    let mut middle = ray::Sphere::init();
    middle.transform = matrix::Matrix::identity(4).translation(-0.5, 1.0, 0.5);
    let middle_material = material::Material {
        color: color::Color::new(0.1, 1.0, 0.5),
        diffuse: 0.7,
        specular: 0.3,
        ..material::Material::default()
    };
    middle.material = middle_material;

    let mut right = ray::Sphere::init();
    right.transform = matrix::Matrix::identity(4)
        .scaling(0.5, 0.5, 0.5)
        .translation(1.5, 0.5, -0.5);
    let right_material = material::Material {
        color: color::Color::new(0.5, 1.0, 0.1),
        diffuse: 0.7,
        specular: 0.3,
        ..material::Material::default()
    };
    right.material = right_material;

    let mut left = ray::Sphere::init();
    left.transform = matrix::Matrix::identity(4)
        .scaling(0.33, 0.33, 0.33)
        .translation(-1.5, 0.33, -0.75);
    let left_material = material::Material {
        color: color::Color::new(1.0, 0.8, 0.1),
        diffuse: 0.7,
        specular: 0.3,
        ..material::Material::default()
    };
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
    world.objects.push(floor);
    world.objects.push(left_wall);
    world.objects.push(right_wall);
    world.objects.push(middle);
    world.objects.push(right);
    world.objects.push(left);

    let mut camera = camera::Camera::new(500, 250, FRAC_PI_3);
    camera.transform =
        matrix::view_transform(point!(0, 1.5, -5), point!(0, 1, 0), vector!(0, 1, 0));

    let canvas = camera::render(&camera, &world);

    let string = ppm::canvas_to_ppm(canvas);
    println!("{}", string);
}
