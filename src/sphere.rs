use crate::hittable::{HitRecord, Hittable};
use crate::{dot, Material, Point3, Ray};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Box<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let shifted_center = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = dot(shifted_center, ray.direction());
        let c = shifted_center.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let root = (-half_b - sqrt_d) / a;

        // First root is not in stated range
        if root < t_min || root > t_max {
            let root = (-half_b + sqrt_d) / a;
            // Second root is also not in stated range
            if root < t_min || root > t_max {
                return None;
            }
        }

        let mut rec = HitRecord::EMPTY;

        rec.t = root;
        rec.p = ray.at(rec.t);

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        rec.material = &*self.material;

        Some(rec)
    }
}
