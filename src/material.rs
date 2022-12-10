use crate::hittable::HitRecord;
use crate::{dot, Color, Ray, Vec3};

pub trait Material {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Color)> {
        None
    }
}

pub struct EmptyMaterial {}

impl Material for EmptyMaterial {}

pub struct Lambertian {
    // albedo = measure of diffuse reflection
    pub(crate) albedo: Color,
}

impl Lambertian {
    pub fn new(color: &Color) -> Lambertian {
        Lambertian { albedo: *color }
    }
}

impl Material for Lambertian {
    // impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray {
            orig: rec.p,
            dir: scatter_direction,
        };
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

pub struct Metal {
    pub(crate) albedo: Color,
    pub fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Self {
        // each ray going to a metal is randomized to simulate fuzzing
        let fuzziness = if fuzziness < 1.0 { fuzziness } else { 1.0 };
        Metal { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(&Vec3::unit_vector(ray_in.direction()), &rec.normal);
        let scattered = Ray {
            orig: rec.p,
            dir: reflected + self.fuzziness * Vec3::random_in_unit_sphere(),
        };
        let attenuation = self.albedo;
        if dot(scattered.direction(), rec.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
