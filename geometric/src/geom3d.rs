use std::ops::{Index, IndexMut};

use math::matrix::*;

pub struct Triangle {
    pts: [Vec3; 3],
}

impl Triangle {
    pub fn new(pts: [Vec3; 3]) -> Self {
        Self { pts }
    }

    pub fn is_clockwise(&self) -> bool {
        let v1 = self.pts[1] - self.pts[0];
        let v2 = self.pts[2] - self.pts[0];
        v1.cross(&v2).dot(&Vec3::ones()) < 0.0
    }
}

impl Index<usize> for Triangle {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pts[index]
    }
}

impl IndexMut<usize> for Triangle {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pts[index]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Frustum {
    pub near: f32,
    pub far: f32,
    pub half_fovy: f32,
    pub aspect: f32,
}

impl Frustum {
    pub fn new(near: f32, far: f32, half_fovy: f32, aspect: f32) -> Self {
        Self {
            near,
            far,
            half_fovy,
            aspect,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Cube {
    pub center: Vec3,
    pub half_len: Vec3,
}

impl Cube {
    pub fn from_center(center: Vec3, half_len: Vec3) -> Self {
        let min = center - half_len;
        let max = center + half_len;

        Self { center, half_len }
    }
}
