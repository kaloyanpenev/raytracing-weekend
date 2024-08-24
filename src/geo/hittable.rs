use crate::geo::hit_record::HitRecord;
use crate::geo::ray::Ray;
use crate::geo::ray_interval::Interval;

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

