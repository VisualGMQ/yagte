use glium::glutin::{
    dpi::Size,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    Api, ContextBuilder, GlRequest,
};
use graphics::renderer::Vertex;

const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 720;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Learn OpenGL with Rust")
        .with_inner_size(Size::Physical(glium::glutin::dpi::PhysicalSize {
            width: WINDOW_WIDTH.try_into().unwrap(),
            height: WINDOW_HEIGHT.try_into().unwrap(),
        }));

    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (4, 4)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");

    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Failed to make context current")
    };

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let mut renderer = graphics::renderer::Renderer::new(WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
    renderer.set_clear_color(math::cg::Color::from_rgb(0.1, 0.1, 0.1));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    renderer.cleanup();
                }
                WindowEvent::Resized(physical_size) => {
                    gl_context.resize(physical_size);
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
                let vertices = [
                    Vertex::new(math::matrix::Vec3::from_xyz(-0.5, -0.5, 0.0)),
                    Vertex::new(math::matrix::Vec3::from_xyz(0.5, -0.5, 0.0)),
                    Vertex::new(math::matrix::Vec3::from_xyz(0.0, 0.5, 0.0)),
                ];
                renderer.draw_arrays(&vertices).unwrap();
                gl_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
