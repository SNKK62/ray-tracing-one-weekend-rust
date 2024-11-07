use crate::utils::clamp;
use rand::Rng;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Color {
    pub fn r(&self) -> u8 {
        (255.999 * self.x()) as u8
    }

    pub fn g(&self) -> u8 {
        (255.999 * self.y()) as u8
    }

    pub fn b(&self) -> u8 {
        (255.999 * self.z()) as u8
    }

    pub fn write(&self, samples_per_pixel: i64) {
        let scale = 1.0 / samples_per_pixel as f64;
        // Divide the color by the number of samples and gamma-correct for gamma=2.0
        let r = (self.x() * scale).sqrt();
        let g = (self.y() * scale).sqrt();
        let b = (self.z() * scale).sqrt();

        let min = 0.0;
        let max = 0.999;

        println!(
            "{} {} {}",
            (255.999 * clamp(r, min, max)).round() as u8,
            (255.999 * clamp(g, min, max)).round() as u8,
            (255.999 * clamp(b, min, max)).round() as u8
        );
    }
}

impl Vec3 {
    pub fn zero() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { e: [x, y, z] }
    }

    pub fn rand() -> Self {
        Self::new(
            rand::thread_rng().gen_range(0.0..1.0),
            rand::thread_rng().gen_range(0.0..1.0),
            rand::thread_rng().gen_range(0.0..1.0),
        )
    }

    pub fn rand_range(min: f64, max: f64) -> Self {
        Self::new(
            rand::thread_rng().gen_range(min..max),
            rand::thread_rng().gen_range(min..max),
            rand::thread_rng().gen_range(min..max),
        )
    }

    pub fn rand_unit_sphere() -> Self {
        loop {
            let sphere = Self::rand_range(-1.0, 1.0);
            if sphere.len() >= 1.0 {
                continue;
            };
            return sphere;
        }
    }

    pub fn rand_unit_vector() -> Self {
        let a = rand::thread_rng().gen_range(0.0..2.0 * std::f64::consts::PI);
        let z = rand::thread_rng().gen_range(-1.0..1.0);
        let r = ((1.0 - z * z) as f64).sqrt();
        Self::new(r * a.cos(), r * a.sin(), z)
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn power(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn len(&self) -> f64 {
        self.power().sqrt()
    }

    pub fn unit(&self) -> Self {
        *self / self.len()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(&self, other: &Self) -> Vec3 {
        Self {
            e: [
                self.e[1] * other.e[2] - self.e[2] * other.e[1],
                self.e[2] * other.e[0] - self.e[0] * other.e[2],
                self.e[0] * other.e[1] - self.e[1] * other.e[0],
            ],
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        if i >= self.e.len() {
            panic!("Index out of bounds");
        }
        &self.e[i]
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            e: [rhs.e[0] * self, rhs.e[1] * self, rhs.e[2] * self],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Self::Output {
        Self::Output {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs],
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}
