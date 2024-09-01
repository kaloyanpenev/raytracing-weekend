use std::ops::Neg;
use rand::Rng;
use crate::Color;
use crate::materials::{Material, refract_vec, reflect_vec};
use crate::ray::hit_record::HitRecord;
use crate::ray::Ray;

pub struct Dielectric {
    refraction_index: f64
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self { Self{refraction_index} }
    pub fn schlick_reflectance(cosine: f64, refraction_index: f64) -> f64{
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0*r0;
        r0 + (1.0-r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::ONE;
        let ri = if hit_record.front_face { 1.0/self.refraction_index } else {self.refraction_index};

        let cos_theta = ray_in.dir.normalize().neg().dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let new_ray_dir =
            if cannot_refract || Self::schlick_reflectance(cos_theta, ri) > rand::thread_rng().gen_range(0.0..1.0)
            { reflect_vec(&ray_in.dir.normalize(), &hit_record.normal) }
            else { refract_vec(&(ray_in.dir.normalize()), &(hit_record.normal), ri)};

        Some((Ray::new(hit_record.pos, new_ray_dir), attenuation))
    }
}