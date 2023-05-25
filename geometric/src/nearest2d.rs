use crate::{geom2d::*, geom_common::*};
use math::matrix::*;

pub fn pt2rect(pt: &Vec2, rect: &AABB) -> Vec2 {
    if crate::contain2d::is_rect_contain_pt(pt, rect) {
        return *pt;
    }

    Vec2::from_xy(
        pt.x()
            .clamp(rect.min().x(), rect.min().x() + rect.size().x()),
        pt.y()
            .clamp(rect.min().y(), rect.min().y() + rect.size().y()),
    )
}

pub fn pt2triangle(_pt: &Vec2, _triangle: &Triangle2D) -> Vec2 {
    todo!();
}

// TODO: implement nearest pt to conic curve
