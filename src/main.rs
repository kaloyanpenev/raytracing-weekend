#[allow(dead_code)]
mod geo;
mod camera;

use std::fs::File;
use std::io::Write;
use glam::{DVec3};

use crate::geo::{
    hittable_list::HittableList,
    sphere::Sphere
};

use crate::camera::Camera;

pub type Color = DVec3;
pub type IColor = glam::IVec3;

fn main() {
    //world
    let sphere = Box::new(Sphere::new(DVec3::new(0., 0., -1.), 0.5));
    let sphere2 = Box::new(Sphere::new(DVec3::new(0., -100.5, -1.), 100.));
    let world : Box<HittableList> = Box::new(HittableList::new(vec![sphere, sphere2]));

    let camera = Camera::new(16.0 / 9.0, 400, 100, 10);

    let (image_data, start_time) = camera.render(&*world);

    let mut file = File::create("image.ppm").expect("Couldn't create file!");
    file.write_all(image_data.as_bytes()).expect("Couldn't write out file");

    // logs
    eprintln!("Done in {}s", start_time.elapsed().as_secs_f64());
}
