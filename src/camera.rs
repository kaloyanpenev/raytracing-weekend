use std::time::Instant;
use glam::DVec3;
use indicatif::ProgressBar;
use crate::geo::ray::Ray;
use crate::Color;
use crate::geo::hittable::Hittable;
use crate::geo::ray_interval::Interval;

pub struct Camera {
    image_size: (i32, i32),
    camera_center: DVec3,
    pixel_delta_uv: (DVec3, DVec3),
    pixel00_loc: DVec3
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self{
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

        Self{image_size, camera_center, pixel_delta_uv: (pixel_delta_u, pixel_delta_v), pixel00_loc}
    }

    fn ray_color(r: &Ray, world: &impl Hittable) -> Color {

        match world.hit(r, Interval::new(0., f64::INFINITY)) {
            None => {}
            Some(hit_record) => { return 0.5 * (hit_record.normal + Color::new(1., 1., 1.)) }
        }


        let unit_direction = r.dir.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        1.0 - a * Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
    }

    pub fn render(&self, world: &impl Hittable) -> (String, Instant) {
        let mut image_data = String::new();

        // render
        image_data += &format!("P3\n{} {}\n255\n", self.image_size.0, self.image_size.1);

        let pb = ProgressBar::new(self.image_size.1 as u64);

        let start_time = Instant::now();
        for j in 0..self.image_size.1 {
            pb.inc(1);
            for i in 0..self.image_size.0 {
                let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_uv.0) + (j as f64 * self.pixel_delta_uv.1);
                let ray_direction = pixel_center - self.camera_center;
                let r = Ray::new(self.camera_center, ray_direction);

                let color_pixel_color = Self::ray_color(&r, &*world);
                image_data += &Self::write_color(&color_pixel_color);
            }
        }
        pb.finish();

        (image_data, start_time)
    }

    fn write_color(col: &Color) -> String {
        let rbyte = (255.999 * col.x) as i32;
        let gbyte = (255.999 * col.y) as i32;
        let bbyte = (255.999 * col.z) as i32;

        format!("{} {} {}\n", rbyte, gbyte, bbyte)
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
