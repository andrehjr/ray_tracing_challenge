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
            assert_eq!(*pixel, color(0.0, 0.0, 0.0));
        }
    }
}
