use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub};

use crate::matrix::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color(Vec4);

impl Color {
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self(Vec4::from_xyzw(r, g, b, 1.0))
    }

    pub fn from_rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self(Vec4::from_xyzw(r, g, b, a))
    }

    pub fn white() -> Self {
        Self::from_rgb(1.0, 1.0, 1.0)
    }

    pub fn black() -> Self {
        Self::from_rgb(0.0, 0.0, 0.0)
    }

    pub fn r(&self) -> f64 {
        (*self)[0]
    }

    pub fn g(&self) -> f64 {
        (*self)[1]
    }

    pub fn b(&self) -> f64 {
        (*self)[2]
    }

    pub fn a(&self) -> f64 {
        (*self)[3]
    }

    pub fn rgb(&self) -> Color {
        Color::from_rgb(self.r(), self.g(), self.b())
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color(self.0.add(rhs.0))
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color(self.0.sub(rhs.0))
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Color::black();
        for i in 0..4 {
            result[i] = self[i] * rhs[i];
        }
        result
    }
}

impl Div for Color {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut result = Color::black();
        for i in 0..4 {
            result[i] = self[i] / rhs[i];
        }
        result
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let mut result = Color::black();
        for i in 0..4 {
            result[i] = self[i] / rhs;
        }
        result
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut result = Color::black();
        for i in 0..4 {
            result[i] = self[i] * rhs;
        }
        result
    }
}

impl Deref for Color {
    type Target = Vec4;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<Vec4> for Color {
    fn into(self) -> Vec4 {
        self.0
    }
}

pub trait Transformation {
    fn get_mat(&self) -> Mat44;
}

pub struct Scale {
    x: f64,
    y: f64,
    z: f64,
}

impl Scale {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn chain(&self, s: &Self) -> Self {
        Self {
            x: self.x * s.x,
            y: self.y * s.y,
            z: self.z * s.z,
        }
    }
}

impl Transformation for Scale {
    #[rustfmt::skip]
    fn get_mat(&self) -> Mat44 {
        Mat44::from_row(&[
            self.x,    0.0,    0.0, 0.0,
               0.0, self.y,    0.0, 0.0,
               0.0,    0.0, self.z, 0.0,
               0.0,    0.0,    0.0, 1.0,
        ])
    }
}

pub struct Translation {
    x: f64,
    y: f64,
    z: f64,
}

impl Translation {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn chain(&self, s: &Self) -> Self {
        Self {
            x: self.x + s.x,
            y: self.y + s.y,
            z: self.z + s.z,
        }
    }
}

impl Transformation for Translation {
    #[rustfmt::skip]
    fn get_mat(&self) -> Mat44 {
        Mat44::from_row(&[
            1.0, 0.0, 0.0, self.x,
            0.0, 1.0, 0.0, self.y,
            0.0, 0.0, 1.0, self.z,
            0.0, 0.0, 0.0, 1.0,
        ])
    }
}

pub struct EularRotationXYZ {
    x: f64,
    y: f64,
    z: f64,
}

impl EularRotationXYZ {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn chain(&self, r: EularRotationXYZ) -> Self {
        Self {
            x: self.x + r.x,
            y: self.y + r.y,
            z: self.z + r.z,
        }
    }
}

#[rustfmt::skip]
fn create_z_rotation(radians: f64) -> Mat44 {
    let s = radians.sin();
    let c = radians.cos();
    Mat44::from_row(&[
          c,  -s, 0.0, 0.0,
          s,   c, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ])
}

#[rustfmt::skip]
fn create_x_rotation(radians: f64) -> Mat44 {
    let s = radians.sin();
    let c = radians.cos();
    Mat44::from_row(&[
        1.0, 0.0, 0.0, 0.0,
        0.0,   c,  -s, 0.0,
        0.0,   s,   c, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ])
}

#[rustfmt::skip]
fn create_y_rotation(radians: f64) -> Mat44 {
    let s = radians.sin();
    let c = radians.cos();
    Mat44::from_row(&[
          c, 0.0,  -s, 0.0,
        0.0, 1.0, 0.0, 0.0,
          s, 0.0,   c, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ])
}

impl Transformation for EularRotationXYZ {
    #[rustfmt::skip]
    fn get_mat(&self) -> Mat44 {
        create_z_rotation(self.z) * create_y_rotation(self.y) * create_x_rotation(self.x)
    }
}

// TODO: implement Rodriguez's formula
// TODO: implement Perspective & Orthograph Projection
// TODO: implement Mirror transform
// TODO: implement Quaternion
// TODO: implement Schmit Orthograph
