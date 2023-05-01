use std::ffi::{CStr, CString};

use ::math::matrix::Vec2;
use geometric::{
    geom2d::{self, *},
    intersect2d::*,
};
use playgrounds::draw_utility::*;
use raylib::prelude::*;

enum Primitive {
    Line(Line),
    Seg(Segment),
    Ray(geom2d::Ray),
    Circle(Circle),
    AABB(AABB),
}

fn intersect(p1: &Primitive, p2: &Primitive) -> Option<(Vec2, Option<Vec2>)> {
    match p1 {
        Primitive::Line(l) => match p2 {
            Primitive::Line(o) => {
                if let Some(p) = line_intersect(&l, &o) {
                    Some((p, None))
                } else {
                    None
                }
            }
            Primitive::Seg(o) => {
                if let Some(p) = line_seg_intersect(&o, &l) {
                    Some((p, None))
                } else {
                    None
                }
            }
            Primitive::Ray(o) => {
                if let Some(p) = line_ray_intersect(&o, &l) {
                    Some((p, None))
                } else {
                    None
                }
            }
            Primitive::Circle(o) => line_circle_intersect(&l, &o),
            Primitive::AABB(o) => todo!(),
        },
        Primitive::Seg(s) => match p2 {
            Primitive::Line(_) => intersect(p2, p1),
            Primitive::Seg(o) => {
                if let Some(p) = seg_intersect(&s, &o) {
                    Some((p, None))
                } else {
                    None
                }
            }
            Primitive::Ray(r) => {
                if let Some(p) = ray_seg_intersect(&s, &r) {
                    Some((p, None))
                } else {
                    None
                }
            }
            Primitive::Circle(c) => segs_circle_intersect(&s, &c),
            Primitive::AABB(_) => todo!(),
        },
        Primitive::Ray(r) => match p2 {
            Primitive::Ray(o) => {
                if let Some(p) = rays_intersect(r, o) {
                    Some((p, None))
                } else {
                    None
                }
            }
            Primitive::Circle(c) => ray_circle_intersect(r, c),
            Primitive::AABB(_) => todo!(),
            _ => intersect(p2, p1),
        },
        Primitive::Circle(c) => match p2 {
            Primitive::Circle(o) => {
                if is_circles_intersect(c, o) {
                    Some(circles_intersect(c, o))
                } else {
                    None
                }
            }
            Primitive::AABB(_) => todo!(),
            _ => intersect(p2, p1),
        },
        Primitive::AABB(_) => todo!(),
    }
}

fn draw_primitive(d: &mut RaylibDrawHandle, p: &Primitive) {
    match p {
        Primitive::Line(l) => draw_line(d, l, Color::GREEN),
        Primitive::Seg(s) => draw_seg(d, s, Color::BROWN),
        Primitive::Ray(r) => draw_ray(d, r, Color::PINK),
        Primitive::Circle(c) => draw_circle(d, c, Color::BLUE),
        Primitive::AABB(r) => draw_rect(d, r, Color::GOLD),
    }
}

const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 720;

const POINT_RADIUS: f32 = 5.0;

fn draw_primitives_and_point(d: &mut RaylibDrawHandle, primitives: &Vec<Primitive>) {
    for primitive in primitives {
        draw_primitive(d, primitive);
    }

    for i in 0..primitives.len() {
        for j in (i + 1)..primitives.len() {
            let result = intersect(&primitives[i], &primitives[j]);
            match result {
                Some((a, b)) => {
                    d.draw_circle(a.x() as i32, a.y() as i32, POINT_RADIUS, Color::RED);
                    if let Some(b) = b {
                        d.draw_circle(b.x() as i32, b.y() as i32, POINT_RADIUS, Color::RED);
                    }
                }
                None => {}
            }
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("playground 2d")
        .msaa_4x()
        .build();

    let primitives: Vec<Primitive> = vec![
        Primitive::Circle(Circle::new(Vec2::from_xy(400.0, 400.0), 50.0)),
        Primitive::Circle(Circle::new(Vec2::from_xy(300.0, 350.0), 70.0)),
        Primitive::Circle(Circle::new(Vec2::from_xy(200.0, 300.0), 60.0)),
        Primitive::Line(Line::new(
            Vec2::from_xy(425.0, 425.0),
            Vec2::from_xy(1.0, 0.5),
        )),
        Primitive::Ray(geom2d::Ray::new(
            Vec2::from_xy(350.0, 100.0),
            Vec2::from_xy(-1.0, 1.0),
        )),
        Primitive::Ray(geom2d::Ray::new(
            Vec2::from_xy(200.0, 100.0),
            Vec2::from_xy(1.0, 1.0),
        )),
        Primitive::Ray(geom2d::Ray::new(
            Vec2::from_xy(600.0, 100.0),
            Vec2::from_xy(1.0, 0.0),
        )),
        Primitive::Seg(geom2d::Segment::new(
            Vec2::from_xy(450.0, 300.0),
            Vec2::from_xy(350.0, 600.0),
        )),
        Primitive::Seg(geom2d::Segment::new(
            Vec2::from_xy(330.0, 400.0),
            Vec2::from_xy(350.0, 610.0),
        )),
    ];

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::DARKGRAY);

        draw_primitives_and_point(&mut d, &primitives);
    }
}
