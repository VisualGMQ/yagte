use geometric::geom3d::Frustum;
use graphics::camera;

use window::{
    app::AppBuilder,
    winit::{
        event::{Event, WindowEvent},
        event_loop::ControlFlow,
    },
};

const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 720;

fn main() {
    env_logger::init();

    let app = AppBuilder::new(
        "VisualDebugger",
        math::matrix::Vec2::from_xy(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
    );

    let camera = camera::Camera::from_persp(
        Frustum::new(
            0.1,
            100.0,
            30f32.to_radians(),
            WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
        ),
        math::matrix::Vec3::zeros(),
    );

    let mut renderer =
        graphics::renderer::Renderer::new(WINDOW_WIDTH, WINDOW_HEIGHT, camera).unwrap();
    renderer.set_clear_color(math::cg::Color::from_rgb(0.1, 0.1, 0.1));

    window::app::run(app, move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    renderer.cleanup();
                }
                WindowEvent::Resized(physical_size) => {
                    renderer
                        .resize(
                            physical_size.width.try_into().unwrap(),
                            physical_size.height.try_into().unwrap(),
                        )
                        .unwrap();
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                renderer.clear();
            }
            _ => (),
        }
    });
}
