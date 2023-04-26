use crate::matrix::*;

#[derive(Copy, Clone)]
pub struct Cartesian3D {
    x: Vec3,
    y: Vec3,
    z: Vec3,
    position: Vec3,

    mat: Mat44,
    inv_mat: Mat44,
}

impl Default for Cartesian3D {
    fn default() -> Self {
        Self {
            x: Vec3::x_axis(),
            y: Vec3::y_axis(),
            z: Vec3::z_axis(),
            position: Vec3::zeros(),
            mat: Mat44::identity(),
            inv_mat: Mat44::identity(),
        }
    }
}

impl Cartesian3D {
    #[rustfmt::skip]
    pub fn new(x: Vec3, y: Vec3, z: Vec3, position: Vec3) -> Self {
        let a = Vec3::from_xyz(x.x(), y.x(), z.x());
        let b = Vec3::from_xyz(x.y(), y.y(), z.y());
        let c = Vec3::from_xyz(x.z(), y.z(), z.z());

        Self {
            x, y, z,
            position,
            mat: Mat44::from_coordination(x, y, z, position),
            inv_mat: Mat44::from_row(&[
            a.x(), a.y(), a.z(), -a.dot(&position),
            b.x(), b.y(), b.z(), -b.dot(&position),
            c.x(), c.y(), c.z(), -c.dot(&position),
              0.0,   0.0,   0.0,               1.0,
        ])
        }
    }

    pub fn x_axis(&self) -> Vec3 {
        self.x
    }

    pub fn y_axis(&self) -> Vec3 {
        self.y
    }

    pub fn z_axis(&self) -> Vec3 {
        self.z
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn get_mat(&self) -> &Mat44 {
        &self.mat
    }

    // a quick way to get inverse transform mat(not calculate inverse matrix)
    pub fn get_inv_mat(&self) -> &Mat44 {
        &self.inv_mat
    }

    pub fn transform(&self, v: Vec3) -> Vec3 {
        (self.mat * Vec4::from(v)).into()
    }

    pub fn transform_inv(&self, v: Vec3) -> Vec3 {
        (self.inv_mat * Vec4::from(v)).into()
    }
}

#[derive(Clone, Copy)]
pub struct Cartesian2D {
    x: Vec2,
    y: Vec2,
    position: Vec2,
}

impl Cartesian2D {
    pub fn x_axis(&self) -> Vec2 {
        self.x
    }

    pub fn y_axis(&self) -> Vec2 {
        self.y
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }
}

impl Default for Cartesian2D {
    fn default() -> Self {
        Self {
            x: Vec2::x_axis(),
            y: Vec2::y_axis(),
            position: Vec2::zeros(),
        }
    }
}

pub struct Polar {
    pub axis: Vec3,
    pub theta: f64,
}

pub struct Cylinder {
    pub polar: Polar,
    pub z: Vec3,
}
