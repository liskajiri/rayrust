use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

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

    // Arithmetic functions

    fn add(&mut self, other: Vec3) -> &mut Vec3 {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self
    }

    // fn sub(&self, other: Vec3) -> Vec3 {
    //     Vec3 {
    //         x: self.x - other.x,
    //         y: self.y - other.y,
    //         z: self.z - other.z,
    //     }
    // }

    fn neg(&self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    fn mult(&mut self, t: f64) -> &mut Vec3 {
        self.x *= t;
        self.y *= t;
        self.z *= t;
        self
    }

    fn div(&mut self, t: f64) -> &mut Vec3 {
        self.mult(1.0 / t)
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
    }

    // vector operations

    fn dot(a: Vec3, b: Vec3) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    pub(crate) fn unit_vector(&mut self) -> &mut Vec3 {
        self.div(self.length())
    }
}

// Utility functions
impl Display for Vec3 {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec: {} {} {}", self.x, self.y, self.z)
    }
}

impl Add<Vec3> for Vec3 {
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

impl Sub<Vec3> for Vec3 {
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

impl Mul<Vec3> for Vec3 {
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

impl Div<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

pub fn write_color(pixel_color: Color) {
    let const_260 = 255.999;
    println!(
        "{} {} {}",
        ((const_260 * pixel_color.x) as i32),
        ((const_260 * pixel_color.y) as i32),
        ((const_260 * pixel_color.z) as i32)
    )
}
