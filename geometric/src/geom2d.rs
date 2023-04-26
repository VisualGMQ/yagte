use math::matrix::*;
use std::ops::{Deref, Index, IndexMut};

#[derive(Clone, Copy, Debug)]
pub struct Linear {
    pub start: Vec2,
    pub dir: Vec2,
    pub len: f32,
}

impl Linear {
    pub fn normal(&self) -> Vec2 {
        if self.dir.y() == 0.0 {
            Vec2::from_xy(0.0, 1.0)
        } else {
            Vec2::from_xy(self.dir.y(), -self.dir.x()).normalize()
        }
    }

    pub fn is_parallel_approx(&self, l: &Linear, decimal_place: u8) -> bool {
        crate::utilitiy::approx_equal(self.dir.cross(&l.dir), 0.0, decimal_place)
    }

    pub fn is_parallel(&self, l: &Linear) -> bool {
        self.dir.cross(&l.dir) == 0.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Line(Linear);

impl Deref for Line {
    type Target = Linear;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Line {
    pub fn new(start: Vec2, dir: Vec2) -> Self {
        Self(Linear {
            start,
            dir,
            len: 1.0,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Segment(Linear);

impl Deref for Segment {
    type Target = Linear;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Segment {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        let dir = end - start;
        Self(Linear {
            start,
            dir: dir.normalize(),
            len: dir.length(),
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ray(Linear);

impl Deref for Ray {
    type Target = Linear;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Ray {
    pub fn new(start: Vec2, dir: Vec2) -> Self {
        Self(Linear {
            start,
            dir,
            len: 1.0,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub pts: [Vec2; 3],
}

impl Triangle {
    pub fn new(pts: [Vec2; 3]) -> Self {
        Self { pts }
    }

    pub fn is_clockwise(&self) -> bool {
        let v1 = self.pts[1] - self.pts[0];
        let v2 = self.pts[2] - self.pts[0];
        v1.cross(&v2) < 0.0
    }
}

impl Index<usize> for Triangle {
    type Output = Vec2;

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
pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
}

impl Circle {
    pub fn new(center: Vec2, radius: f32) -> Circle {
        Self {
            center, radius
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub min: Vec2,
    pub size: Vec2,
}

impl Rect {
    pub fn from_min_size(min: Vec2, size: Vec2) -> Self {
        Self { min, size }
    }

    pub fn from_min_max(min: Vec2, max: Vec2) -> Self {
        Self {
            min,
            size: max - min,
        }
    }

    pub fn from_center(center: Vec2, half_size: Vec2) -> Self {
        Self {
            min: center - half_size,
            size: half_size * 2.0,
        }
    }

    pub fn min(&self) -> &Vec2 {
        &self.min
    }

    pub fn size(&self) -> &Vec2 {
        &self.size
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ellipse {
    pub a: f32,
    pub b: f32,
    pub position: Vec2,
}

#[derive(Clone, Copy, Debug)]
pub struct Parabola {
    pub p: f32,
    pub position: Vec2,
}

#[derive(Clone, Copy, Debug)]
pub struct Hyperbola {
    pub a: f32,
    pub b: f32,
    pub position: Vec2,
}

#[derive(Clone, Copy, Debug)]
pub struct CircleArc {
    pub radius: f32,
    pub center: Vec3,
    pub norm: Vec3,
    pub x_axis: Vec3,
    pub range: (f32, f32),
}

// TODO: implement B-Splin, Bezier Curve
