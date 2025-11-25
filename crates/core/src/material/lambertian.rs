use crate::{
    Color, Ray, RenderContext, Vector3,
    material::{Material, ScatterResult},
    object::HitRecord,
};

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ctx: &RenderContext, r_in: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = hit.normal + Vector3::random_unit(ctx.random);

        if scatter_direction.is_near_near() {
            scatter_direction = hit.normal
        }

        let mut scattered = Ray::new(hit.pt, scatter_direction);
        scattered.time = r_in.time;

        Some(ScatterResult {
            attenuation: self.albedo,
            scattered,
        })
    }
}
