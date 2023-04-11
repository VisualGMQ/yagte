use crate::geom2d::*;
use crate::nearest2d::*;
use math::matrix::*;

pub fn pt2pt_sqrd(pt1: &Vec2, pt2: &Vec2) -> f64 {
    (*pt2 - *pt1).length_sqrd()
}

pub fn pt2line_sqrd(pt: &Vec2, line: &Line) -> f64 {
    let p = pt2line(pt, line);
    pt2pt_sqrd(&p, pt)
}

pub fn pt2seg_sqrd(pt: &Vec2, seg: &Line) -> f64 {
    let p = pt2segment(pt, seg);
    pt2pt_sqrd(&p, pt)
}

pub fn pt2ray_sqrd(pt: &Vec2, line: &Line) -> f64 {
    let p = pt2ray(pt, line);
    pt2pt_sqrd(&p, pt)
}

pub fn pt2polyline_sqrd(pt: &Vec2, polyline: &[Vec2]) -> Option<f64> {
    if polyline.is_empty() {
        return None;
    }

    if polyline.len() == 1 {
        return Some(pt2pt_sqrd(pt, polyline.first().unwrap()));
    }

    let mut min_dist: Option<f64> = None;

    let iter = polyline.windows(2);
    for pts in iter {
        match min_dist {
            None => {
                let line = Line::new(pts[0], pts[1] - pts[0]);
                min_dist = Some((*pt - pt2segment(pt, &line)).length_sqrd());
            }
            Some(dist_sqrt) => {
                let dx1 = pts[0].x() - pt.x();
                let dy1 = pts[0].y() - pt.y();
                let dx2 = pts[1].x() - pt.x();
                let dy2 = pts[1].y() - pt.y();
                if dx1 * dx1 <= dist_sqrt && dx2 * dx2 <= dist_sqrt
                    || dy1 * dy1 <= dist_sqrt && dy2 * dy2 <= dist_sqrt
                {
                    let line = Line::new(pts[0], pts[1] - pts[0]);
                    min_dist = Some((*pt - pt2segment(pt, &line)).length_sqrd());
                }
            }
        }
    }

    min_dist
}

pub fn pt2polygon_sqrd(pt: &Vec2, polygon: &[Vec2]) -> f64 {
    todo!();
}

pub fn pt2triangle_sqrd(pt: &Vec2, triangle: Triangle) -> f64 {
    todo!();
}

pub fn line2line_sqrd(l1: &Line, l2: &Line) -> Option<f64> {
    if l1.dir().cross(l2.dir()) != 0.0 {
        return None;
    }

    Some((*l1.start() - *l2.start()).dot(l1.normal()).abs())
}

pub fn ray2line_sqrd(l: &Line, ray: &Line) -> Option<f64> {
    match line2line_sqrd(l, ray) {
        Some(dist) => Some(dist),
        None => {
            let normal = if (*ray.start() - *l.start()).dot(l.normal()) >= 0.0 {
                *l.normal()
            } else {
                -*l.normal()
            };
            if ray.dir().dot(&normal) >= 0.0 {
                Some(pt2line_sqrd(ray.start(), l))
            } else {
                None
            }
        }
    }
}

pub fn line2seg_sqrd(l: &Line, seg: &Line) -> Option<f64> {
    todo!();
}

pub fn seg2seg_sqrd(s1: &Line, s2: &Line) -> Option<f64> {
    todo!();
}

pub fn ray2ray_sqrd(r1: &Line, r2: &Line) -> Option<f64> {
    todo!();
}

pub fn ray2seg_sqrd(ray: &Line, seg: &Line) -> Option<f64> {
    todo!();
}
