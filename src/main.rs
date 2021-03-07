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
    let blue = color!(0.0, 0.0, 1.0);

    let transform_skew = matrix![ 1.0, 1.0, 0.0, 0.0;
           	                      0.0, 1.0, 0.0, 0.0;
        	                      0.0, 0.0, 1.0, 0.0;
            	                  0.0, 0.0, 0.0, 1.0];

    let _transform_scaling = matrix![ 0.5, 0.0, 0.0, 0.0;
     	                             0.0, 0.5, 0.0, 0.0;
       	                             0.0, 0.0, 0.5, 0.0;
       	                             0.0, 0.0, 0.0, 1.0];

    let _sphere = Sphere { /*transform: transform_skew */};
    let _sphere_two = Sphere { /*transform: transform_scaling */};

    for x in 0..canvas_size {
        let world_x = -half + pixels_size * x as f64;

        for y in 0..canvas_size {
            let world_y = half - pixels_size * y as f64;

            let position = point!(world_x, world_y, wall_z);

            let ray = Ray {
                origin: ray_origin,
                direction: (position - ray_origin).norm(),
            };

            let transformed_ray = ray.transform(transform_skew.clone());

            // let hits = ray.intersect(&sphere);

            // if !hits.is_empty() {
            //	canvas.write_pixel(x, y, color);
            // }

            let hits = transformed_ray.intersect(&_sphere);

            if !hits.is_empty() {
                canvas.write_pixel(x, y, blue);
            }
        }
    }

    let string = canvas_to_ppm(canvas);
    println!("{}", string);
}
