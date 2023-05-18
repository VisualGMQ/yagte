use crate::intersect_common::*;
use crate::utilitiy::approx_equal;
use crate::{contain2d, distance2d, geom2d::*};
use math::matrix::*;
use math::precision::real;

pub fn is_circles_intersect(c1: &Circle, c2: &Circle) -> bool {
    is_circular_intersect(c1, c2)
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

pub fn is_line_intersect(l1: &Line2D, l2: &Line2D) -> bool {
    let end1 = l1.dir * l1.len + l1.start;
    let end2 = l2.dir * l2.len + l2.start;

    (l2.start - l1.start).cross(&l1.dir) * (end2 - l1.start).cross(&l1.dir) <= 0.0
        && (l1.start - l2.start).cross(&l2.dir) * (end1 - l2.start).cross(&l2.dir) <= 0.0
}

pub fn is_line_circle_intersect(l: &Line2D, c: &Circle) -> bool {
    distance2d::pt2line_sqrd(&c.center, l) < c.radius * c.radius
}

pub fn is_seg_circle_intersect(s: &Segment2D, c: &Circle) -> bool {
    distance2d::pt2seg_sqrd(&c.center, &s) < c.radius * c.radius
}

pub fn is_rect_intersect(r1: &AABB, r2: &AABB) -> bool {
    (r2.center.x() - r1.center.x()).abs() < r1.half_size.x() + r2.half_size.x()
        && (r2.center.y() - r1.center.y()).abs() < r1.half_size.y() + r2.half_size.y()
}

pub fn is_ray_aabb_intersect(r: &Ray2D, aabb: &AABB) -> bool {
    if contain2d::is_rect_contain_pt(&r.start, &aabb) {
        return true;
    }

    let points = [
        aabb.min(),
        aabb.min() + Vec2::x_axis() * aabb.half_size.x(),
        aabb.max(),
        aabb.max() - Vec2::x_axis() * aabb.half_size.x(),
    ];
    for i in 0..4 {
        let p1 = points[i];
        let p2 = points[(i + 1) % 4];

        if is_ray_seg_intersect(&r, &Segment2D::new(p1, p2)) {
            return true;
        }
    }

    return false;
}

pub fn line_intersect_param(l1: &Line2D, l2: &Line2D) -> Option<real> {
    if l1.is_parallel(&l2) {
        return None;
    }

    let d = l2.start - l1.start;
    Some(d.cross(&l2.dir) / l1.dir.cross(&l2.dir))
}

pub fn line_intersect(l1: &Line2D, l2: &Line2D) -> Option<Vec2> {
    let param = line_intersect_param(&l1, &l2);
    if let Some(p) = param {
        Some(l1.start + l1.dir * p)
    } else {
        None
    }
}

pub fn line_seg_intersect_param(s: &Segment2D, l: &Line2D) -> Option<real> {
    let t = line_intersect_param(&Line2D::new(s.start, s.dir), l);
    match t {
        Some(t) => {
            if t >= 0.0 && t <= s.len {
                Some(t)
            } else {
                None
            }
        }
        None => None,
    }
}

pub fn line_seg_intersect(s: &Segment2D, l: &Line2D) -> Option<Vec2> {
    match line_seg_intersect_param(s, l) {
        Some(t) => Some(s.start + s.dir * t),
        None => None,
    }
}

pub fn line_ray_intersect_param(r: &Ray2D, l: &Line2D) -> Option<real> {
    let t = line_intersect_param(&Line2D::new(r.start, r.dir), l);
    match t {
        Some(t) => {
            if t >= 0.0 {
                Some(t)
            } else {
                None
            }
        }
        None => None,
    }
}

pub fn line_ray_intersect(r: &Ray2D, l: &Line2D) -> Option<Vec2> {
    match line_ray_intersect_param(r, l) {
        Some(t) => Some(r.start + r.dir * t),
        None => None,
    }
}

pub fn line_circle_intersect_param(l: &Line2D, c: &Circle) -> Option<(real, Option<real>)> {
    let m = l.start - c.center;
    let b = m.dot(&l.dir);
    let c = m.length_sqrd() - c.radius * c.radius;

    let delta = b * b - c;
    if approx_equal(delta, 0.0, 6) {
        Some((-b, None))
    } else if delta < 0.0 {
        None
    } else {
        Some((-b - delta.sqrt(), Some(-b + delta.sqrt())))
    }

}

pub fn line_circle_intersect(l: &Line2D, c: &Circle) -> Option<(Vec2, Option<Vec2>)> {
    if let Some((a, b)) = line_circle_intersect_param(&l, &c) {
        if let Some(b) = b {
            Some((l.start + l.dir * a, Some(l.start + l.dir * b)))
        } else {
            Some((l.start + l.dir * a, None))
        }
    } else {
        None
    }
}

pub fn ray_seg_intersect(seg: &Segment2D, r: &Ray2D) -> Option<Vec2> {
    let param = line_intersect_param(&Line2D::new(seg.start, seg.dir), &Line2D::new(r.start, r.dir));
    match param {
        Some(p) => {
            if p >= 0.0 && p <= seg.len {
                let pt = seg.start + seg.dir * p;
                if (pt - r.start).dot(&r.dir) < 0.0 {
                    None
                } else {
                    Some(pt)
                }
            } else {
                None
            }
        }
        None => None,
    }
}

pub fn is_ray_seg_intersect(r: &Ray2D, seg: &Segment2D) -> bool {
    if ray_seg_intersect(&seg, &r).is_none() {
        return false;
    } else {
        return true;
    }
}

pub fn seg_intersect(s1: &Segment2D, s2: &Segment2D) -> Option<Vec2> {
    if let Some(p) = line_intersect(&Line2D::new(s1.start, s1.dir), &Line2D::new(s2.start, s2.dir)) {
        let proj = (p - s2.start).dot(&s2.dir);
        if proj >= 0.0 && proj <= s2.len {
            Some(p)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn ray_circle_intersect_param(r: &Ray2D, c: &Circle) -> Option<(real, Option<real>)> {
    let result = line_circle_intersect_param(&Line2D::new(r.start, r.dir), &c);
    match result {
        Some((a, b)) => {
            if a < 0.0 {
                if let Some(p) = b {
                    if p >= 0.0 {
                        return Some((p, None));
                    } else {
                        return None;
                    }
                }
                None
            } else {
                Some((a, b))
            }
        }
        None => None,
    }
}

pub fn ray_circle_intersect(r: &Ray2D, c: &Circle) -> Option<(Vec2, Option<Vec2>)> {
    let result = ray_circle_intersect_param(&r, &c);
    match result {
        Some((a, b)) => match b {
            Some(p) => Some((r.start + r.dir * a, Some(r.start + r.dir * p))),
            None => Some((r.start + r.dir * a, None)),
        },
        None => None,
    }
}

pub fn is_ray_circle_intersect(r: &Ray2D, c: &Circle) -> bool {
    if ray_circle_intersect_param(&r, &c).is_none() {
        return false;
    } else {
        return true;
    }
}

pub fn seg_circle_intersect_param(s: &Segment2D, c: &Circle) -> Option<(real, Option<real>)> {
    let result = line_circle_intersect_param(&Line2D::new(s.start, s.dir), &c);
    match result {
        Some((a, b)) => {
            if a >= 0.0 && a <= s.len {
                if let Some(p) = b {
                    if p >= 0.0 && p < s.len {
                        return Some((a, Some(p)));
                    }
                }
                Some((a, None))
            } else {
                if let Some(p) = b {
                    if p >= 0.0 && p < s.len {
                        return Some((p, None));
                    }
                }
                None
            }
        }
        None => None,
    }
}

pub fn segs_circle_intersect(s: &Segment2D, c: &Circle) -> Option<(Vec2, Option<Vec2>)> {
    let result = seg_circle_intersect_param(&s, &c);
    match result {
        Some((a, b)) => match b {
            Some(p) => Some((s.start + s.dir * a, Some(s.start + s.dir * p))),
            None => Some((s.start + s.dir * a, None)),
        },
        None => None,
    }
}

pub fn rays_intersect(r1: &Ray2D, r2: &Ray2D) -> Option<Vec2> {
    if let Some(p) =
        line_intersect_param(&Line2D::new(r1.start, r1.dir), &Line2D::new(r2.start, r2.dir))
    {
        if p < 0.0 {
            None
        } else {
            let pt = r1.start + r1.dir * p;
            if (pt - r2.start).dot(&r2.dir) >= 0.0 {
                Some(pt)
            } else {
                None
            }
        }
    } else {
        None
    }
}
