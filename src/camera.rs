extern crate approx;
use crate::{canvas::Canvas, matrix::Matrix, point, ray::Ray, tuple::Tuple, world::World};

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
            half_height,
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::color::Color;
    use crate::matrix;
    use crate::vector;
    use approx::relative_eq;
    use core::f64::consts::PI;
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

    #[test]
    fn test_constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;

        let camera = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(camera.hsize, 160);
        assert_eq!(camera.vsize, 120);
        assert_eq!(camera.field_of_view, PI / 2.0);
        assert_eq!(camera.transform, Matrix::identity(4));
    }

    #[test]
    fn test_camera_pixel_size_calculation() {
        let camera = Camera::new(200, 125, FRAC_PI_2);
        relative_eq!(camera.pixel_size(), 0.01, epsilon = matrix::EPSILON);

        let camera = Camera::new(125, 200, FRAC_PI_2);
        relative_eq!(camera.pixel_size(), 0.01, epsilon = matrix::EPSILON);
    }

    #[test]
    fn test_constructing_a_ray_trough_the_center_of_the_canvas() {
        let camera = Camera::new(201, 101, PI / 2.0);
        let ray = camera.ray_for_pixel(100, 50);

        assert_eq!(ray.origin, point!(0.0, 0.0, 0.0));
        assert_eq!(ray.direction, vector!(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_constructing_a_ray_trough_the_corner_of_the_canvas() {
        let camera = Camera::new(201, 101, PI / 2.0);
        let ray = camera.ray_for_pixel(0, 0);

        assert_eq!(ray.origin, point!(0.0, 0.0, 0.0));
        assert_eq!(ray.direction, vector!(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn test_constructing_a_ray_when_the_camera_is_transformed() {
        let mut camera = Camera::new(201, 101, FRAC_PI_2);
        camera.transform = Matrix::identity(4)
            .translation(0.0, -2.0, 5.0)
            .rotation_y(FRAC_PI_4);
        let ray = camera.ray_for_pixel(100, 50);

        assert_eq!(ray.origin, point!(0.0, 2.0, -5.0));
        assert_eq!(
            ray.direction,
            vector!(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn test_rendering_a_world_trough_a_camera() {
        let world = World::default();
        let mut camera = Camera::new(11, 11, FRAC_PI_2);
        let from = point!(0.0, 0.0, -5.0);
        let to = point!(0.0, 0.0, 0.0);
        let up = vector!(0.0, 1.0, 0.0);
        camera.transform = crate::matrix::view_transform(from, to, up);

        let image = render(&camera, &world);

        assert_eq!(image.pixel_at(5, 5), &Color::new(0.38066, 0.47583, 0.2855));
    }
}
