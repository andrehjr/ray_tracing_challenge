use crate::canvas::Canvas;

fn scale_color(color: f64) -> i32 {
    let rs = (color * 255.0).ceil();

    if rs > 255.0 {
        255
    } else if rs < 0.0 {
        0
    } else {
        rs as i32
    }
}

pub fn canvas_to_ppm(canvas: Canvas) -> String {
    let mut ppm = vec![];
    ppm.push("P3".to_string());
    ppm.push(format!("{} {}", canvas.width, canvas.height));
    ppm.push("255".to_string());

    for line in canvas.pixels.iter() {
        let mut ppm_line = vec![];

        for pixel in line.iter() {
            ppm_line.push(format!("{}", scale_color(pixel.red)));
            ppm_line.push(format!("{}", scale_color(pixel.green)));
            ppm_line.push(format!("{}", scale_color(pixel.blue)));
        }
        ppm.push(ppm_line.join(" "));
    }

    ppm.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

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
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

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
}
