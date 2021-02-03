use raytracer::*;

fn main() {
    let mut canvas = Canvas::init(5, 3);
    let c1 = color(1.5, 0.0, 0.0);
    let c2 = color(0.0, 0.5, 0.0);
    let c3 = color(-0.5, 0.0, 1.0);

    canvas.write_pixel(0, 0, c1);
    canvas.write_pixel(2, 1, c2);
    canvas.write_pixel(4, 2, c3);

    let string = canvas_to_ppm(canvas);

    println!("{}", string);
}
