use std::sync::Arc;

use crate::{
    Interval, Vector3,
    material::Material,
    object::{HitRecord, Node},
    ray::Ray,
};

#[derive(Debug)]
pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Node for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time);
        let oc = current_center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrt_discriminant) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrt_discriminant) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let pt = ray.at(t);
        let outward_normal = (pt - current_center) / self.radius;
        let mut rec = HitRecord {
            pt,
            normal: Vector3::ZERO, // set by set_face_normal
            t,
            front_face: false,
            material: self.material.clone(),
        };
        rec.set_face_normal(ray, outward_normal);

        Some(rec)
    }
}
