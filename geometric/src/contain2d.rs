use crate::geom2d::*;
use math::matrix::*;

pub fn is_circle_contain_pt(circle: &Circle, pt: &Vec2) -> bool {
    (*pt - circle.center).length_sqrd() <= circle.radius * circle.radius
}

pub fn is_rect_contain_pt(pt: &Vec2, rect: &AABB) -> bool {
    pt.x() >= rect.min().x()
        && pt.x() <= rect.min().x() + rect.size().x()
        && pt.y() >= rect.min().y()
        && pt.y() <= rect.min().y() + rect.size().y()
}

pub fn is_polygon_contain_pt(_pt: &Vec2, _polygon: &[Vec2]) -> bool {
    todo!();
}

pub fn is_obb_contain_pt(pt: &Vec2, obb: &OBB) -> bool {
    let dir = *pt - obb.center;
    let x_magnitude = dir.dot(&obb.x_axis()).abs();
    let y_magnitude = dir.dot(&obb.y_axis()).abs();

    x_magnitude < obb.half_size.x() && y_magnitude < obb.half_size.y()
}
