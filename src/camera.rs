use rand::distributions::Uniform;
use rand::rngs::ThreadRng;

use crate::ray::Ray;
use crate::vec::{Point, Vec3};

pub struct Camera {
    origin: Point,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point,
    u: Vec3,
    v: Vec3,
    // w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Point,
        look_at: Point,
        up_vector: Vec3,
        vertical_field_of_view: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
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
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_distance * w;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            // w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, vec_dist: &Uniform<f64>, rng: &mut ThreadRng) -> Ray {
        let random_disc = self.lens_radius * Vec3::random_inside_unit_disc(vec_dist, rng);
        let offset = self.u * random_disc.x + self.v * random_disc.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
