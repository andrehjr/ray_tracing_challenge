use std::f64::consts::{PI, FRAC_PI_3};

use raytracer::*;
// use std::f64::consts::PI;

fn main() {
    let mut floor = Sphere::init();
    floor.transform = Matrix::identity(4).scaling(10.0, 0.01, 10.0);
    let mut floor_material = Material::default();
    floor_material.color = Color::new(1.0, 0.9, 0.9);
    floor_material.specular = 0.0;
    floor.material = floor_material;


    let mut left_wall = Sphere::init();
    left_wall.transform = Matrix::identity(4)
        .scaling(10.0, 0.01, 10.0)
        .rotation_x(PI / 2.0)
        .rotation_y(-PI / 4.0)
        .translation(0.0, 0.0, 5.0);

    let mut right_wall = Sphere::init();
    right_wall.transform = Matrix::identity(4)
        .scaling(10.0, 0.01, 10.0)
        .rotation_x(PI / 2.0)
        .rotation_y(PI / 4.0)
        .translation(0.0, 0.0, 5.0);

    let mut middle = Sphere::init();
    middle.transform = Matrix::identity(4).translation(-0.5, 1.0, 0.5);
    let mut middle_material = Material::default();
    middle_material.color = Color::new(0.1, 1.0, 0.5);
    middle_material.diffuse = 0.7;
    middle_material.specular = 0.3;
    middle.material = middle_material;

    let mut right = Sphere::init();
    right.transform = Matrix::identity(4).scaling(0.5,0.5,0.5).translation(1.5, 0.5, -0.5);
    let mut right_material = Material::default();
    right_material.color = Color::new(0.5, 1.0, 0.1);
    right_material.diffuse = 0.7;
    right_material.specular = 0.3;
    right.material = right_material;

    let mut left = Sphere::init();
    left.transform = Matrix::identity(4).scaling(0.33,0.33,0.33).translation(1.5, 0.33, -0.75);
    let mut left_material = Material::default();
    left_material.color = Color::new(1.0, 0.8, 0.1);
    left_material.diffuse = 0.7;
    left_material.specular = 0.3;
    left.material = left_material;

    let mut world = World::new();
    world.lights.push(Light { intensity: Color { red: 1.0, green: 1.0, blue: 1.0 }, position: point!(-10,10,-10) });
    
    let mut camera = Camera::new(100, 50, FRAC_PI_3);
    camera.transform = view_transform(point!(0,1.5,-5), point!(0,1,0), vector!(0,1,0));

    let canvas = render(&camera, &world);

    let string = canvas_to_ppm(canvas);
    println!("{}", string);
}
