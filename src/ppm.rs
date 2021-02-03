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
    let mut ppm: Vec<String> = vec![];
    ppm.push("P3".to_string());
    ppm.push(format!("{} {}", canvas.width, canvas.height));
    ppm.push("255".to_string());

    for line in canvas.pixels.iter() {
        let mut ppm_line: Vec<String> = vec![];

        for pixel in line.iter() {
            ppm_line.push(format!("{}", scale_color(pixel.red)));
            ppm_line.push(format!("{}", scale_color(pixel.green)));
            ppm_line.push(format!("{}", scale_color(pixel.blue)));
        }
        ppm.push(ppm_line.join(" "));
    }

    ppm.join("\n")
}
