use rand::distributions::Uniform;
use rand::rngs::ThreadRng;

use super::{Material, Scatter};
use crate::hittable::Record;
use crate::ray::Ray;
use crate::vec::{Color, Vec3};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r: &Ray,
        hit: &Record,
        vec_dist: &Uniform<f64>,
        rng: &mut ThreadRng,
    ) -> Option<Scatter> {
        let mut scatter_direction = hit.normal + Vec3::random_unit(vec_dist, rng);

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        Some(Scatter {
            ray: Ray::new(hit.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}
