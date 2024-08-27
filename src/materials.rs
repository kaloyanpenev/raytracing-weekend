pub mod metal;
pub mod lambertian;

use glam::DVec3;
use rand::Rng;
use crate::Color;
use crate::ray::hit_record::HitRecord;
use crate::ray::Ray;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}

fn gen_random_vec_on_unit_sphere() -> DVec3 {
    loop {
        let dir = DVec3::new(rand::thread_rng().gen_range(-1.0..1.0),
                             rand::thread_rng().gen_range(-1.0..1.0),
                             rand::thread_rng().gen_range(-1.0..1.0));

        if dir.length_squared() < 1.0 {
            return dir.normalize();
        }
    }
}

fn reflect_vec(v: DVec3, n: &DVec3) -> DVec3 {
    v - 2.0*v.dot(*n)**n
}
