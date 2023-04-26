use crate::geom2d::*;
use math::matrix::*;

pub fn pt2line_param(pt: &Vec2, start: &Vec2, dir: &Vec2) -> f32 {
    (*pt - *start).dot(&dir)
}

pub fn pt2line(pt: &Vec2, start: &Vec2, dir: &Vec2) -> Vec2 {
    let t = pt2line_param(pt, start, dir);
    *start + *dir * t
}

pub fn pt2ray_param(pt: &Vec2, ray: &Ray) -> f32 {
    let t = pt2line_param(pt, &ray.start, &ray.dir);
    t.max(0.0)
}

pub fn pt2ray(pt: &Vec2, ray: &Ray) -> Vec2 {
    let t = pt2ray_param(pt, ray);
    ray.start + ray.dir * t
}

pub fn pt2segment_param(pt: &Vec2, seg: &Segment) -> f32 {
    let t = pt2line_param(pt, &seg.start, &seg.dir);
    t.clamp(0.0, seg.len)
}

pub fn pt2segment(pt: &Vec2, seg: &Segment) -> Vec2 {
    let t = pt2segment_param(pt, seg);
    seg.start + seg.dir * t
}

pub fn pt2rect(pt: &Vec2, rect: &Rect) -> Vec2 {
    if crate::intersect2d::is_rect_contain_pt(pt, rect) {
        return *pt;
    }

    Vec2::from_xy(
        pt.x()
            .clamp(rect.min().x(), rect.min().x() + rect.size().x()),
        pt.y()
            .clamp(rect.min().y(), rect.min().y() + rect.size().y()),
    )
}

pub fn pt2triangle(_pt: &Vec2, _triangle: &Triangle) -> Vec2 {
    todo!();
}

// TODO: implement nearest pt to conic curve
