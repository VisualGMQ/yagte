use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub};

use crate::{matrix::*, precision::Real};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color(Vec4);

impl Color {
    pub fn from_rgb(r: Real, g: Real, b: Real) -> Self {
        Self(Vec4::from_xyzw(r, g, b, 1.0))
    }

    pub fn from_rgba(r: Real, g: Real, b: Real, a: Real) -> Self {
        Self(Vec4::from_xyzw(r, g, b, a))
    }

    pub fn white() -> Self {
        Self::from_rgb(1.0, 1.0, 1.0)
    }

    pub fn black() -> Self {
        Self::from_rgb(0.0, 0.0, 0.0)
    }

    pub fn r(&self) -> Real {
        (*self)[0]
    }

    pub fn g(&self) -> Real {
        (*self)[1]
    }

    pub fn b(&self) -> Real {
        (*self)[2]
    }

    pub fn a(&self) -> Real {
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

impl Div<Real> for Color {
    type Output = Self;

    fn div(self, rhs: Real) -> Self::Output {
        let mut result = Color::black();
        for i in 0..4 {
            result[i] = self[i] / rhs;
        }
        result
    }
}

impl Mul<Real> for Color {
    type Output = Self;

    fn mul(self, rhs: Real) -> Self::Output {
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

pub trait Transformation3D {
    fn get_mat(&self) -> Mat44;
}

pub trait Transformation2D {
    fn get_mat(&self) -> Mat22;
}

pub struct Scale {
    x: Real,
    y: Real,
    z: Real,
}

impl Scale {
    pub fn new(x: Real, y: Real, z: Real) -> Self {
        Self { x, y, z }
    }
    pub fn x(&self) -> Real {
        self.x
    }

    pub fn y(&self) -> Real {
        self.y
    }

    pub fn z(&self) -> Real {
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

impl Transformation3D for Scale {
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
    x: Real,
    y: Real,
    z: Real,
}

impl Translation {
    pub fn new(x: Real, y: Real, z: Real) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> Real {
        self.x
    }

    pub fn y(&self) -> Real {
        self.y
    }

    pub fn z(&self) -> Real {
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

impl Transformation3D for Translation {
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

pub struct EularRotationXY {
    rotation: Real,
}

impl EularRotationXY {
    pub fn new(rotation: Real) -> Self {
        Self { rotation }
    }
}

impl Transformation2D for EularRotationXY {
    #[rustfmt::skip]
    fn get_mat(&self) -> Mat22 {
        let c = self.rotation.cos();
        let s = self.rotation.sin();
        Mat22::from_row(&[
            c, -s,
            s, c,
        ])
    }
}

pub struct EularRotationXYZ {
    x: Real,
    y: Real,
    z: Real,
}

impl EularRotationXYZ {
    pub fn new(x: Real, y: Real, z: Real) -> Self {
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
fn create_z_rotation(radians: Real) -> Mat44 {
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
fn create_x_rotation(radians: Real) -> Mat44 {
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
fn create_y_rotation(radians: Real) -> Mat44 {
    let s = radians.sin();
    let c = radians.cos();
    Mat44::from_row(&[
          c, 0.0,   s, 0.0,
        0.0, 1.0, 0.0, 0.0,
         -s, 0.0,   c, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ])
}

impl Transformation3D for EularRotationXYZ {
    #[rustfmt::skip]
    fn get_mat(&self) -> Mat44 {
        create_z_rotation(self.z) * create_y_rotation(self.y) * create_x_rotation(self.x)
    }
}

#[rustfmt::skip]
pub fn create_persp_project(near: Real, far: Real, half_fovy: Real, aspect: Real) -> Mat44 {
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
pub fn create_ortho_project(left: Real, right: Real, bottom: Real, top: Real, far: Real, near: Real) -> Mat44 {
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

pub struct Berycentric {
    alpha: Real,
    beta: Real,
    gamma: Real,
}

impl Berycentric {
    pub fn new(triangle: &[Vec2; 3], pt: Vec2) -> Self {
        let area = (triangle[1] - triangle[0])
            .cross(&(triangle[2] - triangle[1]))
            .abs();
        let area1 = (triangle[1] - pt).cross(&(triangle[2] - pt)).abs();
        let area2 = (triangle[0] - pt).cross(&(triangle[2] - pt)).abs();
        let area3 = (triangle[0] - pt).cross(&(triangle[1] - pt)).abs();

        Self {
            alpha: area1 / area,
            beta: area2 / area,
            gamma: area3 / area,
        }
    }

    pub fn alpha(&self) -> Real {
        self.alpha
    }

    pub fn beta(&self) -> Real {
        self.beta
    }

    pub fn gamma(&self) -> Real {
        self.gamma
    }
}

// TODO: implement Rodriguez's formula
// TODO: implement Mirror transform
// TODO: implement Quaternion
// TODO: implement Schmit Orthograph
