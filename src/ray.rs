use crate::vec::{Point, Vec3};

pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }
}
