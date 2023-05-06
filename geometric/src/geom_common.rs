use math::matrix::*;
use std::ops::{Deref, Index, IndexMut};

#[derive(Clone, Copy, Debug)]
pub struct Linear<const DIM: usize> {
    pub start: Vector<f32, DIM>,
    pub dir: Vector<f32, DIM>,
    pub len: f32,
}

pub type Linear2D = Linear<2>;
pub type Linear3D = Linear<3>;

impl Linear2D {
    pub fn normal(&self) -> Vec2 {
        if self.dir.y() == 0.0 {
            Vec2::from_xy(0.0, 1.0)
        } else {
            Vec2::from_xy(self.dir.y(), -self.dir.x()).normalize()
        }
    }

    pub fn is_parallel_approx(&self, l: &Linear2D, decimal_place: u8) -> bool {
        crate::utilitiy::approx_equal(self.dir.cross(&l.dir), 0.0, decimal_place)
    }

    pub fn is_parallel(&self, l: &Linear2D) -> bool {
        self.dir.cross(&l.dir) == 0.0
    }
}

impl Linear3D {
    pub fn is_parallel_approx(&self, l: &Linear3D, decimal_place: u8) -> bool {
        crate::utilitiy::approx_equal(self.dir.cross(&l.dir).length_sqrd(), 0.0, decimal_place)
    }

    pub fn is_parallel(&self, l: &Linear3D) -> bool {
        self.dir.cross(&l.dir).length_sqrd() == 0.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Line<const DIM: usize>(Linear<DIM>);

impl<const DIM: usize> Deref for Line<DIM> {
    type Target = Linear<DIM>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const DIM: usize> Line<DIM> {
    pub fn new(start: Vector<f32, DIM>, dir: Vector<f32, DIM>) -> Self {
        Self(Linear::<DIM> {
            start,
            dir: dir.normalize(),
            len: 1.0,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Segment<const DIM: usize>(Linear<DIM>);

impl<const DIM: usize> Deref for Segment<DIM> {
    type Target = Linear<DIM>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const DIM: usize> Segment<DIM> {
    pub fn new(start: Vector<f32, DIM>, end: Vector<f32, DIM>) -> Self {
        let dir = end - start;
        Self(Linear::<DIM> {
            start,
            dir: dir.normalize(),
            len: dir.length(),
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ray<const DIM: usize>(Linear<DIM>);

impl<const DIM: usize> Deref for Ray<DIM> {
    type Target = Linear<DIM>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const DIM: usize> Ray<DIM> {
    pub fn new(start: Vector<f32, DIM>, dir: Vector<f32, DIM>) -> Self {
        Self(Linear::<DIM> {
            start,
            dir: dir.normalize(),
            len: 1.0,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle<const DIM: usize> {
    pub pts: [Vector<f32, DIM>; 3],
}

impl<const DIM: usize> Triangle<DIM> {
    pub fn new(pts: [Vector<f32, DIM>; 3]) -> Self {
        Self { pts }
    }
}

impl Triangle<2> {
    pub fn is_clockwise(&self) -> bool {
        let v1 = self.pts[1] - self.pts[0];
        let v2 = self.pts[2] - self.pts[0];
        v1.cross(&v2) < 0.0
    }
}

impl<const DIM: usize> Index<usize> for Triangle<DIM> {
    type Output = Vector<f32, DIM>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pts[index]
    }
}

impl<const DIM: usize> IndexMut<usize> for Triangle<DIM> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pts[index]
    }
}

pub type Line2D = Line<2>;
pub type Line3D = Line<3>;
pub type Segment2D = Segment<2>;
pub type Segment3D = Segment<3>;
pub type Ray2D = Ray<2>;
pub type Ray3D = Ray<3>;
pub type Triangle2D = Triangle<2>;
pub type Triangle3D = Triangle<3>;
