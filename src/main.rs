#[allow(dead_code)]
use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use glam::{DVec3};

use raytracing_weekend_rust::geo::{
    hittable_list::HittableList,
    sphere::Sphere
};

use raytracing_weekend_rust::camera::Camera;
use raytracing_weekend_rust::Color;
use raytracing_weekend_rust::materials::dielectric::Dielectric;
use raytracing_weekend_rust::materials::lambertian::Lambertian;
use raytracing_weekend_rust::materials::metal::Metal;

fn main() {
    //world

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_left_bubble = Rc::new(Dielectric::new(1.0/ 1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.2, 0.2), 1.0));


    let sphere_floor = Box::new(Sphere::new(DVec3::new(0.0, -100.5, -1.0), 100.0, material_ground));
    let sphere1 = Box::new(Sphere::new(DVec3::new(0.0, 0.0, -1.2), 0.5, material_center));
    let sphere2 = Box::new(Sphere::new(DVec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    let sphere3 = Box::new(Sphere::new(DVec3::new(-1.0, 0.0, -1.0), 0.4, material_left_bubble));
    let sphere4 = Box::new(Sphere::new(DVec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    let world : Box<HittableList> = Box::new(HittableList::new(vec![sphere_floor, sphere1, sphere2, sphere3, sphere4]));

    let camera = Camera::new(16.0 / 9.0, // aspect_ratio
                             400, // image_width
                             100, // samples_per_pixel
                             10, // max_bounces
                             30.0, // vertical_fov
                             10.0,
                             3.4,
                             DVec3::new(-2.0, 2.0, 1.0), // camera position
                             DVec3::new(0.0, 0.0, -1.0), // camera image plane position
                             DVec3::new(0.0, 1.0, 0.0)); // world up vector

    let (image_data, start_time) = camera.render(&*world);

    let mut file = File::create("image.ppm").expect("Couldn't create file!");
    file.write_all(image_data.as_bytes()).expect("Couldn't write out file");

    // logs
    eprintln!("Done in {}s", start_time.elapsed().as_secs_f64());
}
