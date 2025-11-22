use crate::{
    Interval,
    object::{HitRecord, Node},
    ray::Ray,
    vector::Vector3,
};

pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
}

impl Node for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
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
        let normal = (pt - self.center) / self.radius;
        let outward_normal = (pt - self.center) / self.radius;
        let mut rec = HitRecord {
            pt,
            normal,
            t,
            front_face: false,
        };
        rec.set_face_normal(ray, outward_normal);

        Some(rec)
    }
}
