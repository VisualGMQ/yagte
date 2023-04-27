use ::math::matrix::Vec2;
use geometric::{
    geom2d::*,
    intersect2d::{circles_intersect, is_circles_intersect},
    nearest2d,
};
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

        let c1 = Circle::new(Vec2::from_xy(400.0, 400.0), 50.0);
        let c2 = Circle::new(
            Vec2::from_xy(d.get_mouse_x() as f32, d.get_mouse_y() as f32),
            70.0,
        );

        draw_circle(&mut d, &c1, Color::GREEN);
        draw_circle(&mut d, &c2, Color::GREEN);

        if is_circles_intersect(&c1, &c2) {
            let (pt1, pt2) = circles_intersect(&c1, &c2);
            draw_circle(
                &mut d,
                &Circle {
                    center: pt1,
                    radius: 3.0,
                },
                Color::BLUE,
            );

            if let Some(pt) = pt2 {
                draw_circle(
                    &mut d,
                    &Circle {
                        center: pt,
                        radius: 3.0,
                    },
                    Color::BLUE,
                );
            }
        }
    }
}
