use crate::geom2d::*;
use crate::utilitiy::approx_equal;
use math::matrix::*;

pub fn is_circles_intersect(c1: &Circle, c2: &Circle) -> bool {
    let len_sqrd = (c2.center - c1.center).length_sqrd();
    len_sqrd >= (c1.radius - c2.radius) * (c1.radius - c2.radius)
        && len_sqrd <= (c1.radius + c2.radius) * (c1.radius + c2.radius)
}

/// calculate intersect point of two circle. these circle must intersect
pub fn circles_intersect(c1: &Circle, c2: &Circle) -> (Vec2, Option<Vec2>) {
    let dir = c2.center - c1.center;

    let len_sqrd = dir.length_sqrd();
    if approx_equal(
        len_sqrd,
        (c1.radius + c2.radius) * (c1.radius + c2.radius),
        6,
    ) {
        return (dir.normalize() * c1.radius + c1.center, None);
    }

    if approx_equal(
        len_sqrd,
        (c1.radius - c2.radius) * (c1.radius - c2.radius),
        6,
    ) {
        if c1.radius < c2.radius {
            return (-dir.normalize() * c1.radius + c1.center, None);
        } else {
            return (dir.normalize() * c1.radius + c1.center, None);
        }
    }

    let cosin = (len_sqrd + c1.radius * c1.radius - c2.radius * c2.radius)
        / (2.0 * c1.radius * len_sqrd.sqrt());
    let sin = (1.0 - cosin * cosin).sqrt();
    let norm = if approx_equal(dir.x(), 0.0, 6) {
        Vec2::from_xy(1.0, 0.0)
    } else {
        Vec2::from_xy(-dir.y() / dir.x(), 1.0).normalize()
    } * sin
        * c1.radius;

    let d = dir.normalize() * cosin * c1.radius;
    (c1.center + d + norm, Some(c1.center + d - norm))
}

pub fn is_line_intersect(l1: &Line, l2: &Line) -> bool {
    let end1 = l1.dir * l1.len + l1.start;
    let end2 = l2.dir * l2.len + l2.start;

    (l2.start - l1.start).cross(&l1.dir) * (end2 - l1.start).cross(&l1.dir) <= 0.0
        && (l1.start - l2.start).cross(&l2.dir) * (end1 - l2.start).cross(&l2.dir) <= 0.0
}
