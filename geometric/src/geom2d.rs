use std::ops::{Index, IndexMut};

use math::matrix::*;

pub struct Line {
    start: Vec2,
    dir: Vec2,
    normal: Vec2,
}

impl Line {
    pub fn new(start: Vec2, dir: Vec2) -> Line {
        Line {
            start,
            dir,
            normal: if dir.y() == 0.0 {
                Vec2::from_xy(0.0, 1.0)
            } else {
                Vec2::from_xy(dir.y(), -dir.x()).normalize()
            },
        }
    }

    pub fn start(&self) -> &Vec2 {
        &self.start
    }

    pub fn dir(&self) -> &Vec2 {
        &self.dir
    }

    pub fn normal(&self) -> &Vec2 {
        &self.normal
    }

    pub fn is_parallel_approx(&self, l: &Line, decimal_place: u8) -> bool {
        crate::utilitiy::approx_equal(self.dir().cross(l.dir()), 0.0, decimal_place)
    }

    pub fn is_parallel(&self, l: &Line) -> bool {
        self.dir().cross(l.dir()) == 0.0
    }
}

pub enum LinearLine {
    Line(Line),
    Segment(Line),
    Ray(Line),
}

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

pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
}

impl Circle {
    pub fn new(center: Vec2, radius: f32) -> Circle {
        Circle { center, radius }
    }

    pub fn is_contain_pt(&self, pt: &Vec2) -> bool {
        (*pt - self.center).length_sqrd() <= self.radius * self.radius
    }
}

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

pub struct CircleArc {
    pub radius: f32,
    pub center: Vec3,
    pub norm: Vec3,
    pub x_axis: Vec3,
    pub range: (f32, f32),
}

// TODO: implement B-Splin, Bezier Curve
