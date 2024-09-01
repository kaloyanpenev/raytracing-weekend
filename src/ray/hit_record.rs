use std::rc::Rc;
use glam::DVec3;
use crate::materials::Material;

pub struct HitRecord {
    pub pos: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>
}

impl HitRecord {
    pub fn new(pos: DVec3, normal: DVec3, t: f64, front_face: bool, material: Rc<dyn Material>) -> Self {
        Self{pos, normal, t, front_face, material }
    }

    pub fn get_face_normal(dir: &DVec3, outward_normal: DVec3) -> (bool, DVec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        let front_face = dir.dot(outward_normal) < 0.0;
        (front_face, if front_face { outward_normal } else { -outward_normal })
    }

}
