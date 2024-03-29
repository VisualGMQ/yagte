use ::math::{matrix::Vec2, precision::Real};
use geometric::{geom2d::*, nearest2d, nearest_common};
use playgrounds::draw_utility::*;
use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 720;

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
        let seg = Segment2D::new(Vec2::from_xy(100.0, 100.0), Vec2::from_xy(200.0, 400.0));
        let line = Line2D::new(
            Vec2::from_xy(100.0, 300.0),
            Vec2::from_xy(400.0, 200.0).normalize(),
        );
        let ray = geometric::geom2d::Ray2D::new(
            Vec2::from_xy(500.0, 300.0),
            Vec2::from_xy(700.0, 300.0).normalize(),
        );
        let rect = AABB::from_min_size(Vec2::from_xy(600.0, 200.0), Vec2::from_xy(100.0, 50.0));

        // draw mouse point
        let mouse_pt: ::math::matrix::Matrix<Real, 1, 2> =
            Vec2::from_xy(d.get_mouse_x() as Real, d.get_mouse_y() as Real);
        draw_circle(
            &mut d,
            &Circle {
                center: mouse_pt,
                radius: 3.0,
            },
            Color::RED,
        );

        draw_seg(&mut d, &seg, Color::GREEN);
        draw_line(&mut d, &line, Color::GREEN);
        draw_ray(&mut d, &ray, Color::GREEN);
        draw_rect(&mut d, &rect, Color::GREEN);

        // nearest results
        let results = [
            nearest_common::pt2segment(&mouse_pt, &seg),
            nearest_common::pt2line(&mouse_pt, &line.start, &line.dir),
            nearest_common::pt2ray(&mouse_pt, &ray),
            nearest2d::pt2rect(&mouse_pt, &rect),
        ];

        // draw results
        for result in results {
            draw_circle(
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
