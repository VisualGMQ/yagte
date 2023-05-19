use math::precision::Real;

use crate::geom3d::*;
use crate::geom_common::Sphere;
use crate::utilitiy::approx_equal;

pub fn planes_intersect(p1: &Plane, p2: &Plane) -> Line {
    let dir = p1.normal.cross(&p2.normal);
    let s1 = p1.normal.dot(&p1.pt);
    let s2 = p1.normal.dot(&p1.pt);
    let dot = p1.normal.dot(&p2.normal);
    let base = dot * dot - 1.0;
    let a = (s2 * dot - s1) / base;
    let b = (s1 * dot - s2) / base;

    Line::new(p1.normal * a + p2.normal * b, dir)
}

pub fn line_sphere_intersect_param(r: &Line3D, c: &Sphere) -> Option<(Real, Option<Real>)> {
    let d = r.start - c.center;
    let a = 1.0;
    let b = 2.0 * r.dir.dot(&d);
    let c = d.length_sqrd() - c.radius * c.radius;

    let delta = b * b - 4.0 * a * c;
    if delta < 0.0 {
        None
    } else if approx_equal(delta, 0.0, 0.00001) {
        Some((- b / (2.0 * a), None))
    } else {
        let delta_sqrt = delta.sqrt();
        Some(((-delta_sqrt - b) / (2.0 * a), Some((delta_sqrt - b) / (2.0 * a))))
    }
}

pub fn ray_sphere_intersect_param(r: &Ray3D, c: &Sphere) -> Option<(Real, Option<Real>)> {
    let result = line_sphere_intersect_param(&Line3D::new(r.start, r.dir), &c);
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
