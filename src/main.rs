use raytracer::*;
// use std::f64::consts::PI;

fn main() {
    let canvas_size = 100;
    let mut canvas = Canvas::init(canvas_size, canvas_size);

    let ray_origin = point!(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixels_size = wall_size / canvas_size as f64;
    let half = wall_size / 2.0;

    let _color = color!(1.0, 0.0, 0.0); // red
                                        // let blue = color!(0.0, 0.0, 1.0);
    // let transform_skew = matrix![ 1.0, 1.0, 0.0, 0.0;
    //        	                      0.0, 1.0, 0.0, 0.0;
    //     	                      0.0, 0.0, 1.0, 0.0;
    //         	                  0.0, 0.0, 0.0, 1.0];

    let material = Material {
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
        color: color!(1.0, 0.2, 1.0),
    };

    let _sphere = Sphere {
        material: material,
        transform: Matrix::identity(4),
    };

    let light = Light {
        intensity: color!(1.0, 1.0, 1.0),
        position: point!(-10.0, 10.0, -10.0),
    };

    for x in 0..canvas_size {
        let world_x = -half + pixels_size * x as f64;

        for y in 0..canvas_size {
            let world_y = half - pixels_size * y as f64;

            let position = point!(world_x, world_y, wall_z);

            let ray = Ray {
                origin: ray_origin,
                direction: (position - ray_origin).norm(),
            };

            let hits = ray.intersect(&_sphere);

            if !hits.is_empty() {
                let hit = hits.first().unwrap();
                let point = ray.position(hit.t);

                let normal = hit.object.normal_at(point);
                let eye = ray.direction.negate();

                let lightning_color =
                    lightning(hit.object.material, light.clone(), point, eye, normal);

                canvas.write_pixel(x, y, lightning_color);
            }
        }
    }

    let string = canvas_to_ppm(canvas);
    println!("{}", string);
}
