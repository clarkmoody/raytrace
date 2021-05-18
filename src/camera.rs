use crate::ray::Ray;
use crate::vec::{Point, Vec3};

pub struct Camera {
    origin: Point,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point,
}

impl Camera {
    pub fn new(
        look_from: Point,
        look_at: Point,
        up_vector: Vec3,
        vertical_field_of_view: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = vertical_field_of_view.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        // Unit vector pointing toward the camera on the boresight
        let w = (look_from - look_at).unit();
        // Unit vector to the right in the frame
        let u = up_vector.cross(w).unit();
        // Unit vector pointing up in the frame
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

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
