use crate::geo::hit_record::HitRecord;
use crate::geo::hittable::Hittable;
use crate::geo::ray::Ray;
use crate::geo::ray_interval::Interval;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self { Self{ objects } }
    pub fn clear(&mut self) { self.objects.clear(); }
    pub fn add(&mut self, object: Box<dyn Hittable>) { self.objects.push(object); }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit: Option<HitRecord>= None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            match object.hit(r, Interval::new(ray_t.min, closest_so_far.clone())) {
                None => {}
                Some(res) => {
                    closest_so_far = res.t;
                    hit = Some(res);
                }
            }
        }

        hit
    }
}