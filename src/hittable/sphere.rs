use super::{Hittable, Record};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Point;

use std::ops::RangeInclusive;
use std::sync::Arc;

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material: material.clone(),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_range: RangeInclusive<f64>) -> Option<Record> {
        let oc = r.origin - self.center;
        let a = r.direction.mag_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.mag_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root within range
        let mut root = (-half_b - sqrtd) / a;
        if !t_range.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !t_range.contains(&root) {
                return None;
            }
        }

        let point = r.at(root);
        let outward_normal = (point - self.center) / self.radius;

        Some(Record::new(
            r,
            point,
            outward_normal,
            root,
            self.material.clone(),
        ))
    }
}
