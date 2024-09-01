pub mod metal;
pub mod lambertian;
pub mod dielectric;

use std::ops::Neg;
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

fn reflect_vec(v: &DVec3, n: &DVec3) -> DVec3 {
    *v - 2.0*v.dot(*n)**n
}

fn refract_vec(v: &DVec3, n: &DVec3, etai_over_etat: f64) -> DVec3 {
    //     auto cos_theta = std::fmin(dot(-uv, n), 1.0);
    //     vec3 r_out_perp =  etai_over_etat * (uv + cos_theta*n);
    //     vec3 r_out_parallel = -std::sqrt(std::fabs(1.0 - r_out_perp.length_squared())) * n;
    //     return r_out_perp + r_out_parallel;

    let cos_theta = n.dot(v.neg()).min(1.0);
    let r_out_perp = etai_over_etat * ((*v) + cos_theta * (*n));
    let r_out_parallel = (1.0 - r_out_perp.length_squared()).abs().sqrt().neg() * *n;
    r_out_perp + r_out_parallel
}
