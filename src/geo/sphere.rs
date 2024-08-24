use glam::DVec3;
use crate::geo::hit_record::HitRecord;
use crate::geo::hittable::Hittable;
use crate::geo::ray::Ray;
use crate::geo::ray_interval::Interval;

pub struct Sphere {
    center: DVec3,
    radius: f64
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64) -> Self {
        Self { center, radius: radius.max(0.) }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.dir.length_squared();
        let h= ray.dir.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h*h - a*c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // nearest root that lies in the acceptable range w.r.t to `t`
        let root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            let second_root = (h + sqrtd) / a;
            if !ray_t.surrounds(second_root) {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let (front_face, normal) = HitRecord::get_face_normal(ray, outward_normal);


        Some(HitRecord::new(p, normal, t, front_face))
    }
}


