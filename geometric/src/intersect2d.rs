use crate::geom2d::*;
use math::matrix::*;
use crate::utilitiy::approx_equal;

pub fn is_circles_intersect(c1: &Circle, c2: &Circle) -> bool {
    let len_sqrd = (c2.center - c1.center).length_sqrd();
    len_sqrd >= (c1.radius - c2.radius) * (c1.radius - c2.radius) &&
    len_sqrd <= (c1.radius + c2.radius) * (c1.radius + c2.radius)
}

/// calculate intersect point of two circle. these circle must intersect
pub fn circles_intersect(c1: &Circle, c2: &Circle) -> (Vec2, Option<Vec2>) {
    let dir = c2.center - c1.center;

    let len_sqrd = dir.length_sqrd();
    if approx_equal(len_sqrd, (c1.radius + c2.radius) * (c1.radius + c2.radius), 6) {
        return (dir.normalize() * c1.radius + c1.center, None);
    }

    if approx_equal(len_sqrd, (c1.radius - c2.radius) * (c1.radius - c2.radius), 6) {
        if c1.radius < c2.radius {
            return (-dir.normalize() * c1.radius + c1.center, None);
        } else {
            return (dir.normalize() * c1.radius + c1.center, None);
        }
    }

    let cosin = (dir.length_sqrd() + c1.radius * c1.radius - c2.radius * c2.radius) / (2.0 * c1.radius * dir.length());
    let sin = (1.0 - cosin * cosin).sqrt();
    let norm = if approx_equal(dir.x(), 0.0, 6) {
        Vec2::from_xy(1.0, 0.0)
    } else {
        Vec2::from_xy(-dir.y() / dir.x(), 1.0).normalize()
    } * sin * c1.radius;

    let d = dir.normalize() * cosin * c1.radius;
    (c1.center + d + norm, Some(c1.center + d - norm))
}