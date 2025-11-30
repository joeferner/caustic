pub mod cosine_pdf;
pub mod sphere_pdf;

pub use cosine_pdf::CosinePdf;
pub use sphere_pdf::SpherePdf;

use core::f64;

use crate::{Random, Vector3};

pub trait ProbabilityDensityFunction {
    fn value(&self, direction: Vector3) -> f64;
    fn generate(&self, random: &dyn Random) -> Vector3;
}
