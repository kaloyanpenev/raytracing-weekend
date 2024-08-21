use glam::{DVec3};
use crate::Color;

pub struct Ray {
    origin: DVec3,
    dir : DVec3
}

impl Ray {
    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.dir
    }
    pub fn new(orig: DVec3, dir: DVec3) -> Self {
        Self { origin: orig, dir }
    }
}

pub fn ray_color(r: &Ray) -> Color {
    if (hit_sphere(&DVec3::new(0., 0., -1.), 0.5, r)) {
        return Color::new(1., 0., 0.);
    }

    let unit_direction = r.dir.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    1.0 - a * Color::new(1.0, 1.0, 1.0) + a* Color::new(0.5, 0.7, 1.0)
}

pub fn hit_sphere(center: &DVec3, radius: f64, r: &Ray) -> bool {
    let oc = *center - r.origin;
    let a = r.dir.dot(r.dir);
    let b = -2.0 * r.dir.dot(oc);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant >= 0.0
}
