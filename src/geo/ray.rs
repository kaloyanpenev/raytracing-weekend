use glam::DVec3;

pub struct Ray {
    pub origin: DVec3,
    pub dir : DVec3
}

impl Ray {
    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.dir
    }
    pub fn new(origin: DVec3, dir: DVec3) -> Self {
        Self { origin, dir }
    }
}
