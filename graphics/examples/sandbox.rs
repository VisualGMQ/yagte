use glium::glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    Api, ContextBuilder, GlRequest,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Learn OpenGL with Rust");

    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (4, 5)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");

    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Failed to make context current")
    };

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let mut renderer = graphics::renderer::Renderer::new().unwrap();
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
                WindowEvent::Resized(physical_size) => gl_context.resize(physical_size),
                _ => (),
            },
            Event::RedrawRequested(_) => {
                renderer.clear();
                gl_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
