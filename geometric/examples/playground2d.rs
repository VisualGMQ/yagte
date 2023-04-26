use ::math::matrix::{Vec2, Vec3};
use geometric::{distance2d, geom2d::*, nearest2d};
use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 720;

mod draw {
    use ::math::matrix::*;
    use geometric::geom2d::{Circle, Line, Rect, Segment, Triangle};
    use raylib::prelude::*;

    use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

    pub fn draw_line(d: &mut RaylibDrawHandle, line: &Line, color: Color) {
        if line.dir.x() == 0.0 {
            d.draw_line(
                line.start.x() as i32,
                0,
                line.start.x() as i32,
                WINDOW_HEIGHT,
                color,
            );
        } else {
            let t = -line.start.x() / line.dir.x();
            let pt1 = Vec2::from_xy(0.0, line.start.y() + t * line.dir.y());
            let t = (WINDOW_WIDTH as f32 - line.start.x()) / line.dir.x();
            let pt2 = Vec2::from_xy(WINDOW_WIDTH as f32, line.start.y() + t * line.dir.y());
            d.draw_line(
                pt1.x() as i32,
                pt1.y() as i32,
                pt2.x() as i32,
                pt2.y() as i32,
                color,
            );
        }
    }

    pub fn draw_ray(d: &mut RaylibDrawHandle, line: &geometric::geom2d::Ray, color: Color) {
        if line.dir.x() == 0.0 {
            d.draw_line(
                line.start.x() as i32,
                0,
                line.start.x() as i32,
                WINDOW_HEIGHT,
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
                let t = (WINDOW_WIDTH as f32 - line.start.x()) / line.dir.x();
                let end = Vec2::from_xy(WINDOW_WIDTH as f32, line.start.y() + t * line.dir.y());
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

    pub fn draw_seg(d: &mut RaylibDrawHandle, line: &Segment, color: Color) {
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

    pub fn draw_triangle(d: &mut RaylibDrawHandle, tri: &Triangle, color: Color) {
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

    pub fn draw_rect(d: &mut RaylibDrawHandle, rect: &Rect, color: Color) {
        d.draw_rectangle(
            rect.min.x() as i32,
            rect.min.y() as i32,
            rect.size.x() as i32,
            rect.size.y() as i32,
            color,
        );
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("playground 2d")
        .msaa_4x()
        .build();

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::DARKGRAY);

        // some primitives
        let seg = Segment::new(Vec2::from_xy(100.0, 100.0), Vec2::from_xy(200.0, 400.0));
        let line = Line::new(
            Vec2::from_xy(100.0, 300.0),
            Vec2::from_xy(400.0, 200.0).normalize(),
        );
        let ray = geometric::geom2d::Ray::new(
            Vec2::from_xy(500.0, 300.0),
            Vec2::from_xy(700.0, 300.0).normalize(),
        );
        let rect = Rect::from_min_size(Vec2::from_xy(600.0, 200.0), Vec2::from_xy(100.0, 50.0));

        // draw mouse point
        let mouse_pt = Vec2::from_xy(d.get_mouse_x() as f32, d.get_mouse_y() as f32);
        draw::draw_circle(
            &mut d,
            &Circle {
                center: mouse_pt,
                radius: 3.0,
            },
            Color::RED,
        );

        draw::draw_seg(&mut d, &seg, Color::GREEN);
        draw::draw_line(&mut d, &line, Color::GREEN);
        draw::draw_ray(&mut d, &ray, Color::GREEN);
        draw::draw_rect(&mut d, &rect, Color::GREEN);

        // nearest results
        let results = [
            nearest2d::pt2segment(&mouse_pt, &seg),
            nearest2d::pt2line(&mouse_pt, &line.start, &line.dir),
            nearest2d::pt2ray(&mouse_pt, &ray),
            nearest2d::pt2rect(&mouse_pt, &rect),
        ];

        // draw results
        for result in results {
            draw::draw_circle(
                &mut d,
                &Circle {
                    center: result,
                    radius: 3.0,
                },
                Color::BLUE,
            );
        }
    }
}
