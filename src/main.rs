use std::fs::File;
use std::io::Write;
use std::time::{Instant};
use indicatif::{ProgressBar};
use glam::{Vec3};

fn main() {
    // image
    let image_width = 256;
    let image_height = 256;
    let mut image_data = String::new();

    // render
    image_data += &format!("P3\n{} {}\n255\n", image_width, image_height);

    let pb = ProgressBar::new(image_height);

    let started = Instant::now();
    for j in 0..image_height {
        pb.inc(1);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            image_data += &format!("{} {} {}\n", ir, ig, ib);
        }
    }
    let _a = Vec3::splat(0.0);

    let mut file = File::create("image.ppm").expect("Couldn't create file!");
    file.write_all(image_data.as_bytes()).expect("Couldn't write out file");

    // logs
    pb.finish();
    eprintln!("Done in {}s", started.elapsed().as_secs_f64());
}
