use crate::geom2d::*;
use math::matrix::*;

pub fn is_rect_contain_pt(pt: &Vec2, rect: &Rect) -> bool {
    pt.x() >= rect.min().x()
        && pt.x() <= rect.min().x() + rect.size().x()
        && pt.y() >= rect.min().y()
        && pt.y() <= rect.min().y() + rect.size().y()
}

pub fn is_polygon_contain_pt(_pt: &Vec2, _polygon: &[Vec2]) -> bool {
    todo!();
}
