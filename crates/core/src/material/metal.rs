use crate::{
    Color, Ray, RenderContext,
    material::{Material, ScatterResult},
    object::HitRecord,
};

#[derive(Debug)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, _ctx: &RenderContext, r_in: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let reflected = r_in.direction.reflect(hit.normal);
        Some(ScatterResult {
            attenuation: self.albedo,
            scattered: Ray::new(hit.pt, reflected),
        })
    }
}
