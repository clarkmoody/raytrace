use crate::ray::Ray;
use crate::vec::{Point, Vec3};

pub struct Camera {
    origin: Point,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point,
}

impl Camera {
    pub fn new(viewport_height: f64, aspect_ratio: f64, focal_length: f64) -> Self {
        let viewport_width = viewport_height * aspect_ratio;

        let origin = Point::ZERO;
        // +x to the right
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        // +y up
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        // +z is out of the frame
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
