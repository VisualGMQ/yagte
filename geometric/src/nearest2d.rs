use crate::geom2d::*;
use math::matrix::*;

pub fn pt2line_param(pt: &Vec2, line: &Line) -> f64 {
    (*pt - *line.start()).dot(line.dir()) / line.dir().length_sqrd()
}

pub fn pt2line(pt: &Vec2, line: &Line) -> Vec2 {
    let t = pt2line_param(pt, line);
    *line.start() + *line.dir() * t
}

pub fn pt2ray_param(pt: &Vec2, line: &Line) -> f64 {
    let t = pt2line_param(pt, line);
    t.max(0.0)
}

pub fn pt2ray(pt: &Vec2, line: &Line) -> Vec2 {
    let t = pt2ray_param(pt, line);
    *line.start() + *line.dir() * t
}

pub fn pt2segment_param(pt: &Vec2, line: &Line) -> f64 {
    let t = pt2line_param(pt, line);
    t.clamp(0.0, 1.0)
}

pub fn pt2segment(pt: &Vec2, line: &Line) -> Vec2 {
    let t = pt2segment_param(pt, line);
    *line.start() + *line.dir() * t
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

pub fn pt2triangle(pt: &Vec2, triangle: &Triangle) -> Vec2 {
    todo!();
}

// TODO: implement nearest pt to conic curve
