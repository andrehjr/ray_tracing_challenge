use crate::{Matrix, Tuple, Ray, point, Canvas, World};

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    pub transform: Matrix,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;
        let half_width: f64;
        let half_height: f64;

        if aspect >= 1.0 {
          half_width = half_view;
          half_height = half_view / aspect;
        } else {
          half_width = half_view * aspect;
          half_height = half_view;
        }
        Self {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::identity(4),
            pixel_size: half_width * 2.0 / hsize as f64,
            half_width,
            half_height
        }
    }

    pub fn pixel_size(&self) -> f64 {
        return self.pixel_size;
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let x_offset = (x as f64 + 0.5) * self.pixel_size;
        let y_offset = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let pixel = self.transform.inverse() * point!(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * point!(0.0, 0.0, 0.0);
        let direction = (pixel - origin).norm();

        Ray::new(origin, direction)
    }
}

pub fn render(camera: &Camera, world: &World) -> Canvas {
    let mut canvas = Canvas::init(camera.hsize, camera.vsize);

    for y in 0..camera.vsize {
        for x in 0..camera.hsize {
            let ray = camera.ray_for_pixel(x, y);
            let color = world.color_at(&ray);
            canvas.write_pixel(x, y, color);
        }
    }

    canvas
}