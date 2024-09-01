use std::rc::Rc;
use glam::DVec3;
use crate::ray::hit_record::HitRecord;
use crate::geo::hittable::Hittable;
use crate::materials::Material;
use crate::ray::Ray;
use crate::ray::ray_interval::Interval;

pub struct Sphere {
    center: DVec3,
    radius: f64,
    material: Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self { center, radius: radius.max(0.), material }
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

        let first_root = (h - sqrtd) / a;
        let second_root = (h + sqrtd) / a;
        let first_root_valid = ray_t.surrounds(&first_root);
        let second_root_valid = ray_t.surrounds(&second_root);
        if !first_root_valid && !second_root_valid {
            return None;
        }

        // nearest root to 0 that is above 0
        let t = if first_root_valid { first_root } else { second_root };
        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let (front_face, normal) = HitRecord::get_face_normal(&ray.dir, outward_normal);


        Some(HitRecord::new(p, normal, t, front_face, self.material.clone()))
    }
}


