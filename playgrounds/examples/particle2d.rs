use ::math::matrix::*;
use physics::{particle::Particle, world::World, *};
use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 720;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("particle2d")
        .build();

    rl.set_target_fps(60);

    let mut world = World::new();
    world.create_particle(1, Vec3::from_xyz(400.0, 400.0, 0.0), 1.0);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::DARKGRAY);

        world.update(
            0.05,
            |id, p: &mut Particle, duration, d: &mut RaylibDrawHandle| {
                const FORCE: f32 = 10.0;
                if id == 1 {
                    if d.is_key_down(KeyboardKey::KEY_D) {
                        p.add_force(Vec3::from_xyz(FORCE, 0.0, 0.0));
                    }
                    if d.is_key_down(KeyboardKey::KEY_A) {
                        p.add_force(Vec3::from_xyz(-FORCE, 0.0, 0.0));
                    }
                    if d.is_key_down(KeyboardKey::KEY_W) {
                        p.add_force(Vec3::from_xyz(0.0, -FORCE, 0.0));
                    }
                    if d.is_key_down(KeyboardKey::KEY_S) {
                        p.add_force(Vec3::from_xyz(0.0, FORCE, 1.0));
                    }

                    d.draw_circle(p.pos.x() as i32, p.pos.y() as i32, 3.0, Color::GREEN);
                }
            },
            &mut d,
        );
    }
}
