use std::time::Instant;
use glam::{DVec3, FloatExt};
use indicatif::ProgressBar;
use rand::Rng;
use crate::ray::Ray;
use crate::geo::hittable::Hittable;
use crate::ray::ray_interval::Interval;
use crate::Color;

pub struct Camera {
    image_size: (i32, i32),
    camera_center: DVec3,
    pixel_delta_uv: (DVec3, DVec3),
    pixel00_loc: DVec3,
    samples_per_pixel: i32,
    max_bounces: i32
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32, max_bounces: i32) -> Self{
        // image
        let image_size = Self::get_image_size(aspect_ratio, image_width).expect("image_height can't be less than 1");

        //camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_size = Self::get_viewport_size(image_size, viewport_height);
        let viewport_width = viewport_size.0;
        let camera_center = DVec3::ZERO;
        let viewport_u = DVec3::new(viewport_width, 0., 0.);
        let viewport_v = DVec3::new(0., -viewport_height, 0.);

        let pixel_delta_u = viewport_u / image_size.0 as f64;
        let pixel_delta_v = viewport_v / image_size.1 as f64;

        let viewport_upper_left = camera_center - DVec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self{image_size, camera_center, pixel_delta_uv: (pixel_delta_u, pixel_delta_v), pixel00_loc, samples_per_pixel, max_bounces }
    }

    fn ray_color(r: &Ray, bounce_num: i32, world: &impl Hittable) -> Color {

        if bounce_num <= 0
        {
            return Color::ZERO
        }

        // check hit
        if let Some(hit_record) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            // successful hit: scatter ray based on the material of the hit surface
            return match hit_record.material.scatter(r, &hit_record) {
                Some((scattered_ray, attenuation)) => { attenuation * Self::ray_color(&scattered_ray, bounce_num - 1, world) }
                None() => { Color::ZERO } // absorbed
            }

        }

        // no hit: terminate ray with background color
        let unit_direction = r.dir.normalize();
        let alpha = unit_direction.y.remap(-1.0, 1.0, 0.0, 1.0);
        let bottom_color = Color::new(0.5, 0.75, 1.0);
        let top_color = Color::new(1.0, 1.0, 1.0);
        top_color.lerp(bottom_color, alpha)
    }

    pub fn render(&self, world: &impl Hittable) -> (String, Instant) {
        let mut image_data = String::new();

        // render
        image_data += &format!("P3\n{} {}\n255\n", self.image_size.0, self.image_size.1);

        let pb = ProgressBar::new(self.image_size.1 as u64);

        let start_time = Instant::now();
        for v in 0..self.image_size.1 {
            pb.inc(1);
            for u in 0..self.image_size.0 {
                let mut pixel_color = Color::ZERO;
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(u, v);
                    pixel_color += Self::ray_color(&r, self.max_bounces, &*world);
                }

                image_data += &Self::write_color(&(pixel_color / self.samples_per_pixel as f64));
            }
        }
        pb.finish();

        (image_data, start_time)
    }

    fn sample_square() -> DVec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        DVec3::new(rand::thread_rng().gen_range(0.0..1.0) - 0.5, rand::thread_rng().gen_range(0.0..1.0) - 0.5, 0.)
    }

    fn get_ray(&self, u_pixel: i32, v_pixel: i32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc + (u_pixel as f64 + offset.x) * self.pixel_delta_uv.0 + (v_pixel as f64 + offset.y) * self.pixel_delta_uv.1;
        let ray_origin = self.camera_center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn write_color(pixel_color: &Color) -> String {
        let pixel_color = Self::linear_to_gamma(pixel_color);

        let rbyte = (255.999 * pixel_color.x.clamp(0., 0.999)) as i32;
        let gbyte = (255.999 * pixel_color.y.clamp(0., 0.999)) as i32;
        let bbyte = (255.999 * pixel_color.z.clamp(0., 0.999)) as i32;


        format!("{} {} {}\n", rbyte, gbyte, bbyte)
    }

    fn linear_to_gamma(color: &Color) -> Color {
        Color::new(color.x.sqrt().max(0.0), color.y.sqrt().max(0.0), color.z.sqrt().max(0.0))
    }

    fn get_image_size(aspect_ratio: f64, image_width: i32) -> Option<(i32, i32)>
    {
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        if image_height < 1
        {
            None
        }
        else {
            Some((image_width, image_height))
        }
    }

    fn get_viewport_size(image_size: (i32, i32), viewport_height: f64) -> (f64, f64)
    {
        let viewport_width = viewport_height * (image_size.0 as f64 / image_size.1 as f64);
        (viewport_width, viewport_height)
    }

}
