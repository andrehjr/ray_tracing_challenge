use raytracer::*;
use std::f64::consts::PI;

fn main() {
    let mut canvas = Canvas::init(150, 150);
    let c1 = color!(1.5, 0.0, 0.0);
    let c2 = color!(0.0, 0.5, 0.0);
    let c3 = color!(-0.5, 0.0, 1.0);

    for x in 0..12 {
        let point_a2 = translation!(74.0, 74.0, 0.0)
            * (rotation_z!((PI / 6.0) * (x as f64))
                * (translation!(0.0, 40.0, 0.0) * point!(0.0, 0.0, 0.0)));

        canvas.write_pixel(point_a2.x as usize, point_a2.y as usize, c2);
    }

    let string = canvas_to_ppm(canvas);

    println!("{}", string);
}
