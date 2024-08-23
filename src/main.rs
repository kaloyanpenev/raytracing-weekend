mod hittable;

use std::fs::File;
use std::io::Write;
use std::time::{Instant};
use indicatif::{ProgressBar};
use glam::{DVec3};
use crate::hittable::Sphere;

pub type Color = DVec3;
pub type IColor = glam::IVec3;

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

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_size = get_image_size(aspect_ratio, image_width).expect("image_height can't be less than 1");
    let image_height = image_size.1;

    //world
    let sphere = Box::new(hittable::Sphere::new(DVec3::new(0., 0., -1.), 0.5));
    let sphere2 = Box::new(hittable::Sphere::new(DVec3::new(0., -100.5, -1.), 100.));
    let mut world : Box<hittable::hittable_list> = Box::new(hittable::hittable_list::new());
    world.add(sphere);
    world.add(sphere2);

    //camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_size = get_viewport_size(image_size, viewport_height);
    let viewport_width = viewport_size.0;
    let camera_center = DVec3::ZERO;
    let viewport_u = DVec3::new(viewport_width, 0., 0.);
    let viewport_v = DVec3::new(0., -viewport_height, 0.);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center - DVec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut image_data = String::new();

    // render
    image_data += &format!("P3\n{} {}\n255\n", image_width, image_height);

    let pb = ProgressBar::new(image_height as u64);

    let started = Instant::now();
    for j in 0..image_height {
        pb.inc(1);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i as f64* pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = hittable::Ray::new(camera_center, ray_direction);

            let color_pixel_color = hittable::ray_color(&r, &world);
            image_data += &write_color(&color_pixel_color);
        }
    }
    let _a = DVec3::splat(0.0);

    let mut file = File::create("image.ppm").expect("Couldn't create file!");
    file.write_all(image_data.as_bytes()).expect("Couldn't write out file");

    // logs
    pb.finish();
    eprintln!("Done in {}s", started.elapsed().as_secs_f64());
}
