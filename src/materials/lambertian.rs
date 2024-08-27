use crate::Color;
use crate::materials::{gen_random_vec_on_unit_sphere, Material};
use crate::ray::hit_record::HitRecord;
use crate::ray::Ray;

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self { Self{albedo} }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = hit_record.normal + gen_random_vec_on_unit_sphere();
        let scatter_direction = if {
            scatter_direction.x.abs() < 1e-8 && scatter_direction.y.abs() < 1e-8 && scatter_direction.z.abs() < 1e-8 }
            { hit_record.normal } else { scatter_direction };
        let scattered_ray = Ray::new(hit_record.pos, scatter_direction);
        Some((scattered_ray, self.albedo))
    }
}