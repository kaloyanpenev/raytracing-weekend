use crate::Color;
use crate::materials::{Material, reflect_vec, gen_random_vec_on_unit_sphere};
use crate::ray::hit_record::HitRecord;
use crate::ray::Ray;

pub struct Metal {
    albedo: Color,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self { Self{albedo, fuzz: fuzz.clamp(0.0, 1.0)} }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let fuzz = self.fuzz * gen_random_vec_on_unit_sphere();
        let reflected_dir = reflect_vec(ray_in.dir, &hit_record.normal).normalize() + fuzz;
        let scattered_ray = Ray::new(hit_record.pos, reflected_dir);

        if scattered_ray.dir.dot(hit_record.normal) > 0.0 {
            Some((scattered_ray, self.albedo))
        }
        else {
            None
        }
    }
}