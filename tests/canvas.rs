use raytracer::*;

#[test]
fn test_canvas_new() {
    let canvas = Canvas::init(10, 20);

    assert_eq!(canvas.width, 10);
    assert_eq!(canvas.height, 20);

    assert_eq!(canvas.pixels.len(), 20);

    for line in canvas.pixels.iter() {
        assert_eq!(line.len(), 10);

        for pixel in line.iter() {
            assert_eq!(*pixel, Color::new(0.0, 0.0, 0.0));
        }
    }
}

#[test]
fn test_canvas_pixel_at() {
    let canvas = Canvas::init(10, 20);
    assert_eq!(*canvas.pixel_at(0, 0), Color::new(0.0, 0.0, 0.0))
}

#[test]
fn test_canvas_write_pixel() {
    let mut canvas = Canvas::init(10, 20);
    let black = Color::new(0.0, 0.0, 0.0);
    let red = Color::new(1.0, 0.0, 0.0);

    assert_eq!(*canvas.pixel_at(0, 0), black);
    canvas.write_pixel(0, 0, red.clone());
    assert_eq!(*canvas.pixel_at(0, 0), red);
}

// #[test]
// fn test_canvas_pixel_at_out_of_bounds() {
//  let canvas = Canvas::init(10, 20);
//   canvas.pixel_at(100, 100);
// }
