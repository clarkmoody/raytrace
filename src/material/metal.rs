use rand::distributions::Uniform;
use rand::rngs::ThreadRng;

use super::{Material, Scatter};
use crate::hittable::Record;
use crate::ray::Ray;
use crate::vec::{Color, Vec3};

pub struct Metal {
    albedo: Color,
    // TODO: Float type clamped to 0.0 to 1.0
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r: &Ray,
        hit: &Record,
        vec_dist: &Uniform<f64>,
        rng: &mut ThreadRng,
    ) -> Option<Scatter> {
        let reflected = r.direction.unit().reflect(&hit.normal);
        let target = reflected + self.fuzz * Vec3::random_inside_unit(vec_dist, rng);

        if reflected.dot(hit.normal) > 0.0 {
            Some(Scatter {
                ray: Ray::new(hit.point, target),
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
