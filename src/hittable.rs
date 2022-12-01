use crate::{dot, Point3, Ray, Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.direction(), *outward_normal) < 0.0;

        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -*outward_normal;
        }
    }

    pub const EMPTY: HitRecord = HitRecord {
        p: Vec3::ZERO,
        normal: Vec3::ZERO,
        t: 0.0,
        front_face: false,
    };

    pub fn new(p: Vec3, normal: Vec3, t: f64, front_face: bool) -> Self {
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, _r: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
        None
    }
}
