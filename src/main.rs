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
use raytracing_weekend_rust::materials::lambertian::Lambertian;
use raytracing_weekend_rust::materials::metal::Metal;

fn main() {
    //world

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));


    let sphere_floor = Box::new(Sphere::new(DVec3::new(0.0, -100.5, -1.0), 100.0, material_ground));
    let sphere1 = Box::new(Sphere::new(DVec3::new(0.0, 0.0, -1.2), 0.5, material_center));
    let sphere2 = Box::new(Sphere::new(DVec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    let sphere3 = Box::new(Sphere::new(DVec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    let world : Box<HittableList> = Box::new(HittableList::new(vec![sphere_floor, sphere1, sphere2, sphere3]));

    let camera = Camera::new(16.0 / 9.0, 400, 100, 10);

    let (image_data, start_time) = camera.render(&*world);

    let mut file = File::create("image.ppm").expect("Couldn't create file!");
    file.write_all(image_data.as_bytes()).expect("Couldn't write out file");

    // logs
    eprintln!("Done in {}s", start_time.elapsed().as_secs_f64());
}
