use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::utilities::{random_double, random_double_from_range};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3::ZERO, |a, b| Vec3 {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        })
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const ONE: Vec3 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub fn x(x: f64) -> Vec3 {
        Vec3 { x, y: 0.0, z: 0.0 }
    }
    pub fn y(y: f64) -> Vec3 {
        Vec3 { x: 0.0, y, z: 0.0 }
    }
    pub fn z(z: f64) -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
    }

    // vector operations

    fn _dot(a: Vec3, b: Vec3) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    pub(crate) fn unit_vector(self) -> Vec3 {
        let len = self.length();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn random() -> Vec3 {
        Vec3 {
            x: random_double(),
            y: random_double(),
            z: random_double(),
        }
    }

    // True Lambertian diffuse
    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_from_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: random_double_from_range(min, max),
            y: random_double_from_range(min, max),
            z: random_double_from_range(min, max),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_from_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    // Diffuse method - Uniform scatter direction with no dependence on the normal
    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if dot(in_unit_sphere, *normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn near_zero(self) -> bool {
        // return true if vector is close to zero
        let threshold = 1e-8;
        self.x.abs() < threshold && self.y.abs() < threshold && self.z.abs() < threshold
    }

    // Reflections for smooth metals
    pub fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
        *v - 2.0 * dot(*v, *normal) * *normal
    }

    pub fn refract(uv: &Vec3, n: &Vec3, refr_indexes_proportion: f64) -> Vec3 {
        let cos_theta = 1.0_f64.min(dot(-*uv, *n));
        let r_out_perp = refr_indexes_proportion * (*uv + cos_theta * *n);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * *n;

        r_out_perp + r_out_parallel
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3 {
                x: random_double_from_range(-1.0, 1.0),
                y: random_double_from_range(-1.0, 1.0),
                z: 0.0,
            };
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
}

// Utility functions
impl Display for Vec3 {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec: {} {} {}", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}
