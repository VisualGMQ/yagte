use glium::glutin::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
    Api, ContextBuilder, ContextWrapper, GlRequest, PossiblyCurrent,
};

pub struct AppBuilder {
    gl_context: ContextWrapper<PossiblyCurrent, Window>,
    event_loop: EventLoop<()>,
}

impl AppBuilder {
    pub fn new(title: &str, size: math::matrix::Vec2) -> Self {
        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(PhysicalSize::new(size.x() as u32, size.y() as u32));

        let gl_context = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (4, 4)))
            .build_windowed(window_builder, &event_loop)
            .expect("Cannot create windowed context");

        let gl_context = unsafe {
            gl_context
                .make_current()
                .expect("Failed to make context current")
        };

        gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

        Self {
            gl_context,
            event_loop,
        }
    }
}

pub fn run<F>(app: AppBuilder, mut handler: F)
where
    F: 'static + FnMut(&Event<'_, ()>, &EventLoopWindowTarget<()>, &mut ControlFlow),
{
    app.event_loop.run(move |events, event_loop, control_flow| {
        if let Event::WindowEvent {
            event: WindowEvent::Resized(physical_size),
            ..
        } = &events
        {
            app.gl_context.resize(*physical_size);
        }

        handler(&events, event_loop, control_flow);

        if let Event::RedrawRequested(_) = &events {
            app.gl_context.swap_buffers().unwrap();
        }
    });
}
