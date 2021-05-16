use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{Point, Vec3};

use std::ops::RangeInclusive;
use std::sync::Arc;

pub mod sphere;
pub use sphere::Sphere;

/// Record of the hit
pub struct Record {
    pub point: Point,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub distance: f64,
    pub front_face: bool,
}

/// Something that may be hit by a ray
pub trait Hittable {
    fn hit(&self, r: &Ray, t_range: RangeInclusive<f64>) -> Option<Record>;
}

/// Store a list of hittable objects
#[derive(Default)]
pub struct List(Vec<Box<dyn Hittable>>);

impl List {
    pub fn add<T: 'static>(&mut self, hittable: T)
    where
        T: Hittable,
    {
        self.0.push(Box::new(hittable));
    }
}

impl Hittable for List {
    fn hit(&self, r: &Ray, t_range: RangeInclusive<f64>) -> Option<Record> {
        self.0
            .iter()
            .fold((None, t_range), |(last_hit, t_range), hittable| {
                if let Some(hit) = hittable.hit(r, t_range.clone()) {
                    let reduced_t_range = *t_range.start()..=hit.distance;
                    (Some(hit), reduced_t_range)
                } else {
                    (last_hit, t_range)
                }
            })
            .0
    }
}

impl Record {
    pub fn new(
        r: &Ray,
        point: Point,
        outward_normal: Vec3,
        distance: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = r.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            point,
            normal,
            material: material.clone(),
            distance,
            front_face,
        }
    }
}
