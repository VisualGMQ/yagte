use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub};

use crate::matrix::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color(Vec4);

impl Color {
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self(Vec4::from_xyzw(r, g, b, 1.0))
    }

    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self(Vec4::from_xyzw(r, g, b, a))
    }

    pub fn white() -> Self {
        Self::from_rgb(1.0, 1.0, 1.0)
    }

    pub fn black() -> Self {
        Self::from_rgb(0.0, 0.0, 0.0)
    }

    pub fn r(&self) -> f32 {
        (*self)[0]
    }

    pub fn g(&self) -> f32 {
        (*self)[1]
    }

    pub fn b(&self) -> f32 {
        (*self)[2]
    }

    pub fn a(&self) -> f32 {
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

impl Div<f32> for Color {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let mut result = Color::black();
        for i in 0..4 {
            result[i] = self[i] / rhs;
        }
        result
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
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

impl From<Color> for Vec4 {
    fn from(val: Color) -> Self {
        val.0
    }
}

pub trait Transformation {
    fn get_mat(&self) -> Mat44;
}

pub struct Scale {
    x: f32,
    y: f32,
    z: f32,
}

impl Scale {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
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
    x: f32,
    y: f32,
    z: f32,
}

impl Translation {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
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
    x: f32,
    y: f32,
    z: f32,
}

impl EularRotationXYZ {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
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
fn create_z_rotation(radians: f32) -> Mat44 {
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
fn create_x_rotation(radians: f32) -> Mat44 {
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
fn create_y_rotation(radians: f32) -> Mat44 {
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

#[rustfmt::skip]
pub fn create_persp_project(near: f32, far: f32, half_fovy: f32, aspect: f32) -> Mat44 {
    let inv_half_w = 1.0 / (half_fovy.tan() * near);
    let inv_half_h = aspect * inv_half_w;

    Mat44::from_row(&[
        near * inv_half_w,               0.0,                         0.0,                             0.0,
                      0.0, near * inv_half_h,                         0.0,                             0.0,
                      0.0,               0.0, (far + near) / (near - far), 2.0 * far * near / (near - far),
                      0.0,               0.0,                        -1.0,                             0.0,
    ])
}

#[rustfmt::skip]
pub fn create_ortho_project(left: f32, right: f32, bottom: f32, top: f32, far: f32, near: f32) -> Mat44 {
    let inv_rl = 1.0 / (right - left);
    let inv_tb = 1.0 / (top - bottom);
    let inv_nf = 1.0 / (near - far);

    Mat44::from_row(&[
        2.0 * inv_rl,          0.0,          0.0, - (left + right) * inv_rl,
                 0.0, 2.0 * inv_tb,          0.0, - (top + bottom) * inv_tb,
                 0.0,          0.0, 2.0 * inv_nf,   - (near + far) * inv_nf,
                 0.0,          0.0,          0.0,                       1.0,
    ])
}

// TODO: implement Rodriguez's formula
// TODO: implement Mirror transform
// TODO: implement Quaternion
// TODO: implement Schmit Orthograph
