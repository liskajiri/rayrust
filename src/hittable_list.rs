use crate::hittable::{HitRecord, Hittable};
use crate::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub const EMPTY: HittableList = HittableList { objects: vec![] };

    pub fn new(object: Box<dyn Hittable>) -> Self {
        HittableList {
            objects: vec![object],
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = t_max;

        for object in self.objects.as_slice() {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                hit_anything = Some(rec);
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
}
