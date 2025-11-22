use std::fmt::Debug;

use crate::{Color, Ray, RenderContext, object::HitRecord};

pub mod lambertian;
pub mod metal;

pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material: Debug + Send + Sync {
    fn scatter(&self, ctx: &RenderContext, r_in: &Ray, hit: &HitRecord) -> Option<ScatterResult>;
}

pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Ray,
}
