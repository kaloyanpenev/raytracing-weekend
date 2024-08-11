use glam::{DVec3};
use crate::Color;

pub struct Ray {
    orig : DVec3,
    dir : DVec3
}

impl Ray {
    pub fn at(&self, t: f64) -> DVec3 {
        self.orig + t * self.dir
    }
    pub fn new(orig: DVec3, dir: DVec3) -> Self {
        Self { orig, dir }
    }
}

pub fn ray_color(r: &Ray) -> Color{
    let unit_direction = r.dir.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    1.0 - a * Color::new(1.0, 1.0, 1.0) + a* Color::new(0.5, 0.7, 1.0)
}
