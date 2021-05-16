use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

use super::{Material, Scatter};
use crate::hittable::Record;
use crate::ray::Ray;
use crate::vec::Color;

pub struct Dielectric {
    /// Index of refraction
    refractive_index: RefractiveIndex,
    // TODO: Color
}

impl Dielectric {
    pub fn new(refractive_index: RefractiveIndex) -> Self {
        Self { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r: &Ray,
        hit: &Record,
        vec_dist: &Uniform<f64>,
        rng: &mut ThreadRng,
    ) -> Option<Scatter> {
        // TODO: Refraction across non-vacuum boundaries
        let refraction_ratio = if hit.front_face {
            1.0 / f64::from(self.refractive_index)
        } else {
            self.refractive_index.into()
        };

        let unit_direction = r.direction.unit();
        let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract
            || reflectance(cos_theta, refraction_ratio) > (vec_dist.sample(rng) + 1.0) / 2.0
        {
            // Cannot refract
            unit_direction.reflect(&hit.normal)
        } else {
            unit_direction.refract(&hit.normal, refraction_ratio)
        };

        Some(Scatter {
            ray: Ray::new(hit.point, direction),
            attenuation: Color::ONE,
        })
    }
}

/// [Schlick's Approximation](https://en.wikipedia.org/wiki/Schlick%27s_approximation)
/// for specular reflection.
fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
    // TOOD: Update for non-vacuum interface
    let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

#[derive(Debug, Clone, Copy)]
/// Index of refraction for various materials
pub enum RefractiveIndex {
    /// Earth's atmosphere
    Air,
    /// Solidified tree sap
    Amber,
    /// Pyrex
    BorosilicateGlass,
    /// Low dispersion glass for convex lenses
    CrownGlass,
    /// Diamond alternative
    CubicZirconia,
    /// Pure carbon crystal
    Diamond,
    /// Human eye cornea
    EyeCornea,
    /// Human eye lens
    EyeLens,
    /// Optical glass for concave lenses
    FlintGlass,
    /// Pure glass also known as fused quartz
    FusedSilica,
    /// Water ice
    Ice,
    /// Very cold helium
    LiquidHelium,
    /// Ethylene tetrafluoroethylene
    PlasticEtfe,
    /// Polyethylene terephthalate
    PlasticPet,
    /// Normal window glass
    PlateGlass,
    /// Poly(methyl methacrylate) (PMMA)
    Plexiglass,
    /// Common plastic
    Polycarbonate,
    /// Halite
    RockSalt,
    /// Precios gemstone
    Sapphire,
    /// Table salt
    SodiumChloride,
    /// 25% solution of sugar water
    SugarWater25,
    /// 50% solution of sugar water
    SugarWater50,
    /// 75% solution of sugar water
    SugarWater75,
    /// Nothing
    Vacuum,
    /// Kitchen cooking oil
    VegetableOil,
    /// Hydrogen dioxide liquid
    Water,
    /// User-defined refractive index
    Custom(f64),
}

// TODO: Index of refraction that varies based on frequency of light

impl From<RefractiveIndex> for f64 {
    fn from(ir: RefractiveIndex) -> f64 {
        match ir {
            RefractiveIndex::Air => 1.000273,
            RefractiveIndex::Amber => 1.55,
            RefractiveIndex::BorosilicateGlass => 1.47,
            RefractiveIndex::CrownGlass => 1.52,
            RefractiveIndex::CubicZirconia => 2.165,
            RefractiveIndex::Diamond => 2.417,
            RefractiveIndex::EyeCornea => 1.373,
            RefractiveIndex::EyeLens => 1.386,
            RefractiveIndex::FlintGlass => 1.61,
            RefractiveIndex::FusedSilica => 1.458,
            RefractiveIndex::Ice => 1.31,
            RefractiveIndex::LiquidHelium => 1.025,
            RefractiveIndex::PlasticEtfe => 1.403,
            RefractiveIndex::PlasticPet => 1.575,
            RefractiveIndex::PlateGlass => 1.52,
            RefractiveIndex::Plexiglass => 1.4896,
            RefractiveIndex::Polycarbonate => 1.6,
            RefractiveIndex::RockSalt => 1.516,
            RefractiveIndex::Sapphire => 1.77,
            RefractiveIndex::SodiumChloride => 1.544,
            RefractiveIndex::SugarWater25 => 1.3723,
            RefractiveIndex::SugarWater50 => 1.42,
            RefractiveIndex::SugarWater75 => 1.4774,
            RefractiveIndex::Vacuum => 1.0,
            RefractiveIndex::VegetableOil => 1.47,
            RefractiveIndex::Water => 1.333,
            RefractiveIndex::Custom(ir) => ir,
        }
    }
}
