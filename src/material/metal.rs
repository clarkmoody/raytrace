use rand::distributions::Uniform;
use rand::rngs::ThreadRng;

use super::{Material, Scatter};
use crate::hittable::Record;
use crate::ray::Ray;
use crate::vec::Color;

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r: &Ray,
        hit: &Record,
        _vec_dist: &Uniform<f64>,
        _rng: &mut ThreadRng,
    ) -> Option<Scatter> {
        let reflected = r.direction.unit().reflect(&hit.normal);

        if reflected.dot(hit.normal) > 0.0 {
            Some(Scatter {
                ray: Ray::new(hit.point, reflected),
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
