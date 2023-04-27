use crate::geom2d::*;
use crate::nearest2d::*;
use crate::utilitiy::approx_equal;
use math::matrix::*;

pub fn pt2pt_sqrd(pt1: &Vec2, pt2: &Vec2) -> f32 {
    (*pt2 - *pt1).length_sqrd()
}

pub fn pt2line_sqrd(pt: &Vec2, line: &Line) -> f32 {
    let p = pt2line(pt, &line.start, &line.dir);
    pt2pt_sqrd(&p, pt)
}

pub fn pt2seg_sqrd(pt: &Vec2, seg: &Segment) -> f32 {
    let p = pt2segment(pt, seg);
    pt2pt_sqrd(&p, pt)
}

pub fn pt2ray_sqrd(pt: &Vec2, ray: &Ray) -> f32 {
    let p = pt2ray(pt, ray);
    pt2pt_sqrd(&p, pt)
}

pub fn pt2polyline_sqrd(pt: &Vec2, polyline: &[Vec2]) -> Option<f32> {
    if polyline.is_empty() {
        return None;
    }

    if polyline.len() == 1 {
        return Some(pt2pt_sqrd(pt, polyline.first().unwrap()));
    }

    let mut min_dist: Option<f32> = None;

    let iter = polyline.windows(2);
    for pts in iter {
        match min_dist {
            None => {
                let seg = Segment::new(pts[0], pts[1] - pts[0]);
                min_dist = Some((*pt - pt2segment(pt, &seg)).length_sqrd());
            }
            Some(dist_sqrt) => {
                let dx1 = pts[0].x() - pt.x();
                let dy1 = pts[0].y() - pt.y();
                let dx2 = pts[1].x() - pt.x();
                let dy2 = pts[1].y() - pt.y();
                if dx1 * dx1 <= dist_sqrt && dx2 * dx2 <= dist_sqrt
                    || dy1 * dy1 <= dist_sqrt && dy2 * dy2 <= dist_sqrt
                {
                    let seg = Segment::new(pts[0], pts[1] - pts[0]);
                    min_dist = Some((*pt - pt2segment(pt, &seg)).length_sqrd());
                }
            }
        }
    }

    min_dist
}

pub fn pt2polygon_sqrd(pt: &Vec2, polygon: &[Vec2]) -> Option<f32> {
    if polygon.len() <= 1 {
        None
    } else {
        Some(
            pt2polyline_sqrd(pt, polygon)
                .unwrap_or(0.0)
                .min(pt2seg_sqrd(
                    pt,
                    &Segment::new(polygon[0], *polygon.last().unwrap()),
                )),
        )
    }
}

pub fn pt2triangle_sqrd(pt: &Vec2, triangle: Triangle) -> Option<f32> {
    todo!()
}

pub fn line2line_sqrd(l1: &Line, l2: &Line) -> Option<f32> {
    if approx_equal(l1.dir.cross(&l2.dir), 0.0, 6) {
        return None;
    }

    Some((l1.start - l2.start).dot(&l1.normal()).abs())
}

pub fn ray2line_sqrd(l: &Line, ray: &Line) -> Option<f32> {
    match line2line_sqrd(l, ray) {
        Some(dist) => Some(dist),
        None => {
            let normal = if (ray.start - l.start).dot(&l.normal()) >= 0.0 {
                l.normal()
            } else {
                -l.normal()
            };
            if ray.dir.dot(&normal) >= 0.0 {
                Some(pt2line_sqrd(&ray.start, l))
            } else {
                None
            }
        }
    }
}

pub fn line2seg_sqrd(l: &Line, seg: &Line) -> Option<f32> {
    let end = seg.start + seg.dir * seg.len;
    if (seg.start - l.start).cross(&l.dir) * end.cross(&l.dir) < 0.0 {
        None
    } else {
        Some(pt2line_sqrd(&seg.start, &l).min(pt2line_sqrd(&end, &l)))
    }
}

pub fn seg2seg_sqrd(_s1: &Line, _s2: &Line) -> Option<f32> {
    todo!();
}

pub fn ray2ray_sqrd(_r1: &Line, _r2: &Line) -> Option<f32> {
    todo!();
}

pub fn ray2seg_sqrd(_ray: &Line, _seg: &Line) -> Option<f32> {
    todo!();
}
