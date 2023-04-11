use gl;
use log;
use math;

#[derive(Debug, PartialEq)]
pub enum GLError {
    NoError = 0,
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    InvalidFramebufferOperation,
    OutOfMemory,
    StackUnderflow,
    OverFlow,
    Unknown,
}

fn gl_get_top_error() -> GLError {
    unsafe {
        let err = gl::GetError();
        if err >= GLError::Unknown as u32 {
            GLError::Unknown
        } else {
            std::mem::transmute(err as u8)
        }
    }
}

fn gl_clear_error() {
    const MAX_LOOP_COUNT: u32 = 1000;
    let mut i = 0;
    let mut err = gl_get_top_error();
    while i < MAX_LOOP_COUNT && err != GLError::NoError {
        err = gl_get_top_error();
        i += 1
    }
}

fn gl_get_error() -> GLError {
    gl_clear_error();
    gl_get_top_error()
}

macro_rules! gl_call {
    ($expr: expr) => {
        unsafe {
            gl_clear_error();
            let result = $expr;
            let error = gl_get_error();
            if error == GLError::NoError {
                Ok(result)
            } else {
                log::error!(target: "OpenGL", "Occured Error: {:?}", error);
                Err(error)
            }
        }
    };
}

pub struct Renderer {
    color: math::cg::Color,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            color: math::cg::Color::white(),
        }
    }

    pub fn set_clear_color(&mut self, color: math::cg::Color) {
        self.color = color;
        gl_call!(gl::ClearColor(
            self.color.r() as f32,
            self.color.g() as f32,
            self.color.b() as f32,
            self.color.a() as f32,
        ))
        .unwrap();
    }

    pub fn clear(&mut self) {
        gl_call!(gl::Clear(
            gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT
        ))
        .unwrap();
    }
}
