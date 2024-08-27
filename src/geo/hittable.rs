use crate::ray::hit_record::HitRecord;
use crate::ray::Ray;
use crate::ray::ray_interval::Interval;

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

