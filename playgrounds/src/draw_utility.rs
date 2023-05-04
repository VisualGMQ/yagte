use ::math::matrix::*;
use geometric::geom2d::{Circle, Line2D, Segment2D, Triangle2D, AABB};
use raylib::prelude::*;

pub fn draw_line(d: &mut RaylibDrawHandle, line: &Line2D, color: Color) {
    if line.dir.x() == 0.0 {
        d.draw_line(
            line.start.x() as i32,
            0,
            line.start.x() as i32,
            d.get_screen_height(),
            color,
        );
    } else {
        let t = -line.start.x() / line.dir.x();
        let pt1 = Vec2::from_xy(0.0, line.start.y() + t * line.dir.y());
        let t = (d.get_screen_width() as f32 - line.start.x()) / line.dir.x();
        let pt2 = Vec2::from_xy(
            d.get_screen_width() as f32,
            line.start.y() + t * line.dir.y(),
        );
        d.draw_line(
            pt1.x() as i32,
            pt1.y() as i32,
            pt2.x() as i32,
            pt2.y() as i32,
            color,
        );
    }
}

pub fn draw_ray(d: &mut RaylibDrawHandle, line: &geometric::geom2d::Ray2D, color: Color) {
    if line.dir.x() == 0.0 {
        d.draw_line(
            line.start.x() as i32,
            0,
            line.start.x() as i32,
            d.get_screen_height() as i32,
            color,
        );
    } else {
        let t = -line.start.x() / line.dir.x();
        if t > 0.0 {
            let end = Vec2::from_xy(0.0, line.start.y() + t * line.dir.y());
            d.draw_line(
                line.start.x() as i32,
                line.start.y() as i32,
                end.x() as i32,
                end.y() as i32,
                color,
            );
        } else {
            let t = (d.get_screen_width() as f32 - line.start.x()) / line.dir.x();
            let end = Vec2::from_xy(
                d.get_screen_width() as f32,
                line.start.y() + t * line.dir.y(),
            );
            d.draw_line(
                line.start.x() as i32,
                line.start.y() as i32,
                end.x() as i32,
                end.y() as i32,
                color,
            );
        }
    }
}

pub fn draw_seg(d: &mut RaylibDrawHandle, line: &Segment2D, color: Color) {
    let end = line.start + line.dir * line.len;
    d.draw_line(
        line.start.x() as i32,
        line.start.y() as i32,
        end.x() as i32,
        end.y() as i32,
        color,
    );
}

pub fn draw_circle(d: &mut RaylibDrawHandle, circle: &Circle, color: Color) {
    d.draw_circle(
        circle.center.x() as i32,
        circle.center.y() as i32,
        circle.radius,
        color,
    );
}

pub fn draw_triangle(d: &mut RaylibDrawHandle, tri: &Triangle2D, color: Color) {
    for i in 0..tri.pts.len() {
        let pt1 = &tri.pts[i];
        let pt2 = &tri.pts[(i + 1) % tri.pts.len()];
        d.draw_line(
            pt1.x() as i32,
            pt1.y() as i32,
            pt2.x() as i32,
            pt2.y() as i32,
            color,
        );
    }
}

pub fn draw_rect(d: &mut RaylibDrawHandle, rect: &AABB, color: Color) {
    let min = rect.min();
    let size = rect.size();
    d.draw_rectangle(
        min.x() as i32,
        min.y() as i32,
        size.x() as i32,
        size.y() as i32,
        color,
    );
}
