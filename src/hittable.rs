use glam::DVec3;
use crate::Color;

pub struct Ray {
    origin: DVec3,
    dir : DVec3
}

impl Ray {
    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.dir
    }
    pub fn new(origin: DVec3, dir: DVec3) -> Self {
        Self { origin, dir }
    }
}

pub fn ray_color(r: &Ray, world: &Box<hittable_list>) -> Color {

    match world.hit(r,0., f64::INFINITY) {
        None => {}
        Some(res) => { return 0.5 * (res.normal + Color::new(1., 1., 1.)) }
    }


    let unit_direction = r.dir.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    1.0 - a * Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
}

pub struct hit_record {
    pos: DVec3,
    normal: DVec3,
    t: f64,
    front_face: bool
}

impl hit_record {
    fn new(pos: DVec3, normal: DVec3, t: f64, front_face: bool) -> Self { Self{pos, normal, t, front_face} }

    fn get_face_normal(r: &Ray, outward_normal: DVec3) -> (bool, DVec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        let front_face = r.dir.dot(outward_normal) < 0.;

        (front_face, if front_face { outward_normal } else { -outward_normal })
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<hit_record>;
}

pub struct Sphere {
    center: DVec3,
    radius: f64
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64) -> Self {
        Self { center, radius: radius.max(0.) }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<hit_record> {
        let oc = self.center - r.origin;
        let a = r.dir.length_squared();
        let h= r.dir.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h*h - a*c;

        if (discriminant < 0.0) {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // nearest root that lies in the acceptable range w.r.t to `t`
        let root = (h - sqrtd) / a;
        if (root <= ray_tmin || root >= ray_tmax) {
            let secondRoot = (h + sqrtd) / a;
            if (secondRoot <= ray_tmin || secondRoot >= ray_tmax) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let (front_face, normal) = hit_record::get_face_normal(r, outward_normal);


        Some(hit_record::new(p, normal, t, front_face))
    }
}

pub struct hittable_list {
    objects: Vec<Box<dyn Hittable>>
}

impl hittable_list {
    pub fn new() -> Self { Self{ objects: vec![] } }
    pub fn clear(&mut self) { self.objects.clear(); }
    pub fn add(&mut self, object: Box<dyn Hittable>) { self.objects.push(object); }
}

impl Hittable for hittable_list {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<hit_record> {
        let mut hit: Option<hit_record>= None;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            match (object.hit(r, ray_tmin, closest_so_far.clone())) {
                None => {}
                Some(res) => {
                    closest_so_far = res.t;
                    hit = Some(res);
                }
            }
        }

        hit
    }
}