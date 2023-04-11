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
