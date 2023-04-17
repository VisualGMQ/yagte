use geometric::geom3d;
use math::{cg::*, matrix::*};

pub enum ProjectGeomentry {
    Frustum(geom3d::Frustum),
    Cube(geom3d::Cube),
}

enum RotateType {
    LookAt(Vec3),
    EularRot(Vec3),
}

pub struct Camera {
    proj_obj: ProjectGeomentry,
    project: Mat44,
    view: Mat44,
    position: Vec3,

    rotate: RotateType,
}

impl Camera {
    pub fn from_ortho(cube: geom3d::Cube, position: Vec3) -> Self {
        let min = cube.center - cube.half_len;
        let max = cube.center + cube.half_len;
        Self {
            proj_obj: ProjectGeomentry::Cube(cube),
            project: create_ortho_project(min.x(), max.x(), min.y(), max.y(), max.z(), max.z()),
            view: Mat44::identity(),
            position,
            rotate: RotateType::EularRot(Vec3::zeros()),
        }
    }

    pub fn from_persp(frustum: geom3d::Frustum, position: Vec3) -> Self {
        Self {
            proj_obj: ProjectGeomentry::Frustum(frustum),
            project: create_persp_project(
                frustum.near,
                frustum.far,
                frustum.half_fovy,
                frustum.aspect,
            ),
            view: Mat44::identity(),
            position,
            rotate: RotateType::EularRot(Vec3::zeros()),
        }
    }

    pub fn get_project_object(&self) -> &ProjectGeomentry {
        &self.proj_obj
    }

    pub fn move_to(&mut self, position: Vec3) {
        self.position = position;
        self.recalc_view();
    }

    pub fn lookat(&mut self, target: Vec3) {
        self.rotate = RotateType::LookAt(target);
        self.recalc_view();
    }

    pub fn set_rotation(&mut self, rotation: Vec3) {
        self.rotate = RotateType::EularRot(rotation);
        self.recalc_view();
    }

    #[rustfmt::skip]
    fn recalc_view(&mut self) {
        let rotate = match self.rotate {
            RotateType::LookAt(target) => {
                let up = Vec3::y_axis();
                let back = (self.position - target).normalize();
                let right = up.cross(&back);
                let up = back.cross(&right);
                Mat44::from_row(&[
                    right.x(), right.y(), right.z(), 0.0,
                       up.x(),    up.y(),    up.z(), 0.0,
                     back.x(),  back.y(),  back.z(), 0.0,
                          0.0,       0.0,       0.0, 1.0,
                ])
            },
            RotateType::EularRot(rotation) => math::cg::EularRotationXYZ::new(-rotation.x(), -rotation.y(), -rotation.z()).get_mat(),
        };
        let translation = Translation::new(-self.position.x(), -self.position.y(), -self.position.z()).get_mat();
        self.view = rotate * translation;
    }

    pub fn get_project(&self) -> &Mat44 {
        &self.project
    }

    pub fn get_view(&self) -> &Mat44 {
        &self.view
    }
}
