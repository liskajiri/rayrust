use crate::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl Camera {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = Camera::ASPECT_RATIO * Camera::VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    pub fn new() -> Self {
        let origin = Point3::ZERO;
        let horizontal = Vec3::x(Camera::VIEWPORT_WIDTH);
        let vertical = Vec3::y(Camera::VIEWPORT_HEIGHT);
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vec3::z(Camera::FOCAL_LENGTH),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            orig: self.origin,
            dir: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}
