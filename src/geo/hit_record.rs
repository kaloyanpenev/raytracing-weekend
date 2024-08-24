use glam::DVec3;
use crate::geo::ray::Ray;

pub struct HitRecord {
    pub pos: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn new(pos: DVec3, normal: DVec3, t: f64, front_face: bool) -> Self { Self{pos, normal, t, front_face} }
    pub fn get_face_normal(r: &Ray, outward_normal: DVec3) -> (bool, DVec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        let front_face = r.dir.dot(outward_normal) < 0.;

        (front_face, if front_face { outward_normal } else { -outward_normal })
    }
}
