use geometric::geom3d;
use math::{cg::*, matrix::*};

pub enum ProjectGeomentry {
    Frustum(geom3d::Frustum),
    Cube(geom3d::Cube),
}

pub struct Camera {
    proj_obj: ProjectGeomentry,
    project: Mat44,
    view: Mat44,
    position: Vec3,
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
            view: Camera::calc_view(&position),
            position,
        }
    }

    pub fn move_to(&mut self, position: Vec3) {
        self.position = position;
        self.recalc_view();
    }

    fn recalc_view(&mut self) {
        // TODO add rotation
        self.view = Camera::calc_view(&self.position);
    }

    fn calc_view(position: &Vec3) -> Mat44 {
        Translation::new(-position.x(), -position.y(), -position.z()).get_mat()
    }

    pub fn get_project(&self) -> &Mat44 {
        &self.project
    }

    pub fn get_view(&self) -> &Mat44 {
        &self.view
    }
}
