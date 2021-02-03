use raytracer::*;

#[test]
fn test_canvas_to_ppm() {
    let canvas = Canvas::init(5, 3);

    let string = canvas_to_ppm(canvas);

    let mut lines = string.lines();

    assert_eq!(Some("P3"), lines.next());
    assert_eq!(Some("5 3"), lines.next());
    assert_eq!(Some("255"), lines.next());
}

#[test]
fn test_canvas_to_ppm_body() {
    let mut canvas = Canvas::init(5, 3);
    let c1 = color(1.5, 0.0, 0.0);
    let c2 = color(0.0, 0.5, 0.0);
    let c3 = color(-0.5, 0.0, 1.0);

    canvas.write_pixel(0, 0, c1);
    canvas.write_pixel(2, 1, c2);
    canvas.write_pixel(4, 2, c3);

    let string = canvas_to_ppm(canvas);

    let mut lines = string.lines();

    lines.next();
    lines.next();
    lines.next();

    assert_eq!(Some("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"), lines.next());
    assert_eq!(Some("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0"), lines.next());
    assert_eq!(Some("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"), lines.next());
}
