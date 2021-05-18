use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Color = Vec3;
pub type Point = Vec3;

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    const NEAR_ZERO_EPS: f64 = 1.0e-8;

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn components(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn as_color_u8(&self) -> [u8; 3] {
        let r = (self.x.clamp(0.0, 1.0) * 255.0).floor() as u8;
        let g = (self.y.clamp(0.0, 1.0) * 255.0).floor() as u8;
        let b = (self.z.clamp(0.0, 1.0) * 255.0).floor() as u8;
        [r, g, b]
    }

    pub fn sqrt(&self) -> Self {
        Self {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }

    // Length of the vector
    pub fn mag(&self) -> f64 {
        self.mag_squared().sqrt()
    }

    // Squared length of the vector
    pub fn mag_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    // Get the unit vector in the direction of this vector
    pub fn unit(&self) -> Self {
        *self / self.mag()
    }

    // Transform into unit vector
    pub fn make_unit(&mut self) {
        *self /= self.mag();
    }

    pub fn random_inside_unit(vec_dist: &Uniform<f64>, rng: &mut ThreadRng) -> Self {
        // vec_dist is unifrom over -1.0..=1.0
        loop {
            let v = Self::new(
                vec_dist.sample(rng),
                vec_dist.sample(rng),
                vec_dist.sample(rng),
            );
            if v.mag_squared() <= 1.0 {
                return v;
            }
        }
    }

    pub fn random_unit(vec_dist: &Uniform<f64>, rng: &mut ThreadRng) -> Self {
        Self::random_inside_unit(vec_dist, rng).unit()
    }

    pub fn random_inside_unit_disc(vec_dist: &Uniform<f64>, rng: &mut ThreadRng) -> Self {
        // vec_dist is unifrom over -1.0..=1.0
        loop {
            let v = Self::new(vec_dist.sample(rng), vec_dist.sample(rng), 0.0);
            if v.mag_squared() < 1.0 {
                return v;
            }
        }
    }

    pub fn random(dist: &Uniform<f64>, rng: &mut ThreadRng) -> Self {
        Self::new(dist.sample(rng), dist.sample(rng), dist.sample(rng))
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs() < Self::NEAR_ZERO_EPS
            && self.y.abs() < Self::NEAR_ZERO_EPS
            && self.z.abs() < Self::NEAR_ZERO_EPS
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Hadamard / Schur entrywise product of two vectors
    pub fn schur(&self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - 2.0 * self.dot(*normal) * *normal
    }

    /// Self should be a unit vector. Eta ratio is source eta / target eta
    pub fn refract(&self, normal: &Self, eta_ratio: f64) -> Self {
        let cos_theta = (-*self).dot(*normal).min(1.0);
        let ray_perpendicular = eta_ratio * (*self + cos_theta * *normal);
        let ray_parallel = -((1.0 - ray_perpendicular.mag_squared()).abs()).sqrt() * *normal;
        ray_perpendicular + ray_parallel
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, scalar: f64) {
        *self = Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        };
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Self::Output {
        Self::Output {
            x: vec.x * self,
            y: vec.y * self,
            z: vec.z * self,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scalar: f64) {
        *self = Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        };
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}
