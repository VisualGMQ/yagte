use crate::utilitiy::approx_equal;
use math::{coord::Cartesian3D, matrix::*};
use std::ops::{Index, IndexMut};

pub struct Plane {
    pub normal: Vec3,
    pub pt: Vec3,
}

impl Plane {
    pub fn new(normal: Vec3, pt: Vec3) -> Self {
        Self { normal, pt }
    }

    pub fn is_parallel(&self, plane: &Plane) -> bool {
        approx_equal(self.normal.cross(&plane.normal).length_sqrd(), 0.0, 6)
    }
}

pub struct Line {
    pub start: Vec3,
    pub dir: Vec3, // normalized
}

impl Line {
    pub fn new(start: Vec3, dir: Vec3) -> Self {
        Self { start, dir }
    }
}

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
        Self { center, half_len }
    }

    pub fn from_min_max(min: Vec3, max: Vec3) -> Self {
        let center = (max + min) * 0.5;
        let half_len = (max - min) * 0.5;
        Self { center, half_len }
    }
}

#[derive(Clone, Debug)]
pub struct Polygon {
    pub points: Vec<Vec3>,
}

#[derive(Clone, Debug, Copy)]
pub struct Ellipse {
    pub x_axis: Vec3, // must be normalized
    pub normal: Vec3,
    pub a: f32,
    pub b: f32,
    pub position: Vec3,
}

impl Ellipse {
    pub fn get_coord(&self) -> Cartesian3D {
        let x = self.x_axis;
        let y = self.normal;
        let z = x.cross(&y);

        Cartesian3D::new(x, y, z, self.position)
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Parabola {
    pub x_axis: Vec3, // must be normalized
    pub normal: Vec3,
    pub p: f32,
    pub position: Vec3,
}

impl Parabola {
    pub fn get_coord(&self) -> Cartesian3D {
        let x = self.x_axis;
        let y = self.normal;
        let z = x.cross(&y);

        Cartesian3D::new(x, y, z, self.position)
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Hyperbola {
    pub x_axis: Vec3, // must be normalized
    pub normal: Vec3,
    pub a: f32,
    pub b: f32,
    pub position: Vec3,
}

impl Hyperbola {
    pub fn get_coord(&self) -> Cartesian3D {
        let x = self.x_axis;
        let y = self.normal;
        let z = x.cross(&y);

        Cartesian3D::new(x, y, z, self.position)
    }
}

pub enum Conic {
    Ellipse(Ellipse),
    Hyperbola(Hyperbola),
    Parabola(Parabola),
}

pub struct ConicArc {
    pub conic: Conic,
    pub range: (f32, f32), // range in theta(radians)
}

pub fn conic_to_polar(conic: &Conic) -> ConicArcInPolar {
    let (p, e, position, x_axis, normal) = match conic {
        Conic::Parabola(p) => (p.p, 1.0, p.position, p.x_axis, p.normal),
        Conic::Ellipse(e) => {
            let c = (e.a * e.a - e.b * e.b).sqrt();
            let alig = e.a * e.a / c;
            (alig - c, c / e.a, e.position, e.x_axis, e.normal)
        }
        Conic::Hyperbola(h) => {
            let c = (h.a * h.a + h.b * h.b).sqrt();
            let alig = h.a * h.a / c;
            (c - alig, c / h.a, h.position, h.x_axis, h.normal)
        }
    };
    ConicArcInPolar::new(p, e, position, x_axis, normal)
}

pub struct ConicArcInPolar {
    pub p: f32,
    pub e: f32,
    pub origin: Vec3,
    pub axis: Vec3,
    pub normal: Vec3,
}

impl ConicArcInPolar {
    pub fn new(p: f32, e: f32, origin: Vec3, axis: Vec3, normal: Vec3) -> Self {
        Self {
            p,
            e,
            origin,
            axis,
            normal,
        }
    }

    pub fn is_ellipse(&self) -> bool {
        self.e < 1.0
    }

    pub fn is_parabola(&self) -> bool {
        self.e == 1.0
    }

    pub fn is_hyperbola(&self) -> bool {
        self.e > 1.0
    }

    pub fn calc_l(&self, theta: f32) -> f32 {
        self.e * self.p / (1.0 - self.e * theta.cos())
    }

    pub fn get_focal_len(&self) -> f32 {
        if self.is_parabola() {
            -self.p / 2.0
        } else {
            self.e * self.e * self.p / (1.0 - self.e * self.e)
        }
    }

    pub fn get_major_axis_len(&self) -> f32 {
        self.get_focal_len() / self.e
    }

    pub fn get_minor_axis_len(&self) -> f32 {
        let a = self.get_major_axis_len();
        let c = self.get_focal_len();
        (a * a - c * c).sqrt()
    }

    pub fn contain(&self, pt: &Vec3) -> bool {
        let a = self.get_major_axis_len();
        let b = self.get_minor_axis_len();
        let c = (a * a - b * b).sqrt();

        if self.is_ellipse() {
            (pt.x() - c) * (pt.x() - c) / a * a + pt.y() * pt.y() / b * b <= 1.0
        } else if self.is_hyperbola() {
            (pt.x() - c) * (pt.x() - c) / a * a + pt.y() * pt.y() / b * b >= 1.0
        } else {
            pt.y() * pt.y() - 2.0 * self.p * (pt.x() - self.p / 2.0) <= 0.0
        }
    }
}

pub struct Cylinder {
    pub bottom: Vec3,
    pub dir: Vec3, // normalized
    pub height: f32,
    pub radius: f32,
}

pub struct Cone {
    pub bottom: Vec3,
    pub bottom_radius: f32,
    pub dir: Vec3, // normalized
    pub height: f32,
}

pub struct TruncatedCone {
    pub bottom: Vec3,
    pub bottom_radius: f32,
    pub top_radius: f32,
    pub dir: Vec3, // normalized
    pub height: f32,
}

pub enum CylinderLike {
    Cylinder(Cylinder),
    Cone(Cone),
    TruncatedCone(TruncatedCone),
}

pub struct CircleArc {
    pub radius: f32,
    pub center: Vec3,
    pub norm: Vec3,
    pub x_axis: Vec3,
    pub range: (f32, f32),
}
