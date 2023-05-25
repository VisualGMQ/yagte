pub use crate::geom_common::{Circle, Line2D, Linear2D, Ray2D, Segment2D, Triangle2D};
use math::cg::EularRotationXY;
use math::precision::Real;
use math::{cg::Transform2D, matrix::*};

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub center: Vec2,
    pub half_size: Vec2,
}

impl AABB {
    pub fn from_min_size(min: Vec2, size: Vec2) -> Self {
        Self {
            center: min + size / 2.0,
            half_size: size / 2.0,
        }
    }

    pub fn from_min_max(min: Vec2, max: Vec2) -> Self {
        Self {
            center: (max + min) / 2.0,
            half_size: (max - min) / 2.0,
        }
    }

    pub fn from_center(center: Vec2, half_size: Vec2) -> Self {
        Self { center, half_size }
    }

    pub fn min(&self) -> Vec2 {
        self.center - self.half_size
    }

    pub fn max(&self) -> Vec2 {
        self.center + self.half_size
    }

    pub fn size(&self) -> Vec2 {
        self.half_size * 2.0
    }
}

pub struct OBB {
    pub center: Vec2,
    pub half_size: Vec2,
    rotation: Real,
    x_axis: Vec2,
    y_axis: Vec2,
}

impl OBB {
    pub fn new(center: Vec2, half_size: Vec2) -> Self {
        Self {
            center,
            half_size,
            rotation: 0.0,
            x_axis: Vec2::x_axis(),
            y_axis: Vec2::y_axis(),
        }
    }

    pub fn x_axis(&self) -> Vec2 {
        self.x_axis
    }

    pub fn y_axis(&self) -> Vec2 {
        self.y_axis
    }

    pub fn rotation(&self) -> Real {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: Real) {
        self.rotation = rotation;
        let rotation = EularRotationXY::new(rotation);
        self.x_axis = rotation.get_mat() * Vec2::x_axis();
        self.y_axis = rotation.get_mat() * Vec2::y_axis();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ellipse {
    pub a: Real,
    pub b: Real,
    pub position: Vec2,
}

#[derive(Clone, Copy, Debug)]
pub struct Parabola {
    pub p: Real,
    pub position: Vec2,
}

#[derive(Clone, Copy, Debug)]
pub struct Hyperbola {
    pub a: Real,
    pub b: Real,
    pub position: Vec2,
}

#[derive(Clone, Copy, Debug)]
pub struct CircleArc {
    pub radius: Real,
    pub center: Vec3,
    pub norm: Vec3,
    pub x_axis: Vec3,
    pub range: (Real, Real),
}

// TODO: implement B-Splin, Bezier Curve
