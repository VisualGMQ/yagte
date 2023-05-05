use geometric::geom3d;
use math::matrix::*;
use serde::{Serialize, Deserialize};

pub enum Type {
    Cylinder = 1,
    Cone = 2,
    TruncatedCone = 3,
    CircleArc = 4,
    Point = 5,
    Line = 6,
    Ellipse = 7,
    Hyperbola = 8,
    Parabola = 9,
    Polygon = 10,
    Arch = 11,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Cylinder {
    top: [f32; 3],
    bottom: [f32; 3],
    radius: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Cone {
    radius: f32,
    top: [f32; 3],
    bottom: [f32; 3],
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct TruncatedCone {
    top_radius: f32,
    bottom_radius: f32,
    top: [f32; 3],
    bottom: [f32; 3],
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Segment {
    begin: [f32; 3],
    end: [f32; 3],
}

impl Into<geom3d::Cylinder> for Cylinder {
    fn into(self) -> geom3d::Cylinder {
        let dir = Vec3::new(self.top) - Vec3::new(self.bottom);
        let height = dir.length();
        let dir = dir.normalize();
        geom3d::Cylinder{
            bottom: Vec3::new(self.bottom),
            dir,
            height,
            radius: self.radius,
        }
    }
}

impl Into<geom3d::Cone> for Cone {
    fn into(self) -> geom3d::Cone {
        let dir = Vec3::new(self.top) - Vec3::new(self.bottom);
        let height = dir.length();
        let dir = dir.normalize();
        geom3d::Cone {
            bottom: Vec3::new(self.bottom),
            dir,
            height,
            bottom_radius: self.radius,
        }
    }
}

impl Into<geom3d::TruncatedCone> for TruncatedCone {
    fn into(self) -> geom3d::TruncatedCone {
        let dir = Vec3::new(self.top) - Vec3::new(self.bottom);
        let height = dir.length();
        let dir = dir.normalize();
        geom3d::TruncatedCone {
            bottom: Vec3::new(self.bottom),
            dir,
            height,
            bottom_radius: self.bottom_radius,
            top_radius: self.top_radius,
        }
    }
}

impl Into<geom3d::Segment3D> for Segment {
    fn into(self) -> geom3d::Segment3D {
        let begin = Vec3::new(self.begin);
        let end = Vec3::new(self.end);
        geom3d::Segment3D::new(begin, end)
    }
}

pub enum NetGeometry {
    Seg(Segment),
    Cone(Cone),
    Cylinder(Cylinder),
    TruncatedCone(TruncatedCone),
}
