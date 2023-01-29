use crate::hittable::HitRecord;
use crate::utilities::random_double;
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
    pub fn new(color: &Color) -> Self {
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

pub struct Dielectric {
    pub(crate) index_of_refraction: f64,
}

impl Dielectric {
    pub(crate) fn new(index_of_refraction: f64) -> Self {
        Dielectric {
            index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Schlick's approximation for reflectance
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::ONE;

        let refraction_ratio = if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let unit_direction = ray_in.direction().unit_vector();

        let cos_theta = 1.0_f64.min(dot(-unit_direction, rec.normal));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        // Snell's law if no solution exists
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double()
        {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        let scattered = Ray {
            orig: rec.p,
            dir: direction,
        };

        Some((scattered, attenuation))
    }
}
