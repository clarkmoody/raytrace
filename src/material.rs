use rand::distributions::Uniform;
use rand::rngs::ThreadRng;

use crate::hittable::Record;
use crate::ray::Ray;
use crate::vec::Color;

pub mod lambertian;
pub mod metal;

pub use lambertian::Lambertian;
pub use metal::Metal;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(
        &self,
        r: &Ray,
        hit: &Record,
        vec_dist: &Uniform<f64>,
        rng: &mut ThreadRng,
    ) -> Option<Scatter>;
}
