use crate::geom2d::*;
use math::matrix::*;

pub fn is_circle_contain_pt(circle: &Circle, pt: &Vec2) -> bool {
    (*pt - circle.center).length_sqrd() <= circle.radius * circle.radius
}
