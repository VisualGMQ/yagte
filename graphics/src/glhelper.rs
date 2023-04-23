#[derive(Debug, PartialEq)]
pub enum GLErrorType {
    NoError = 0,

    // gl inner error
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    InvalidFramebufferOperation,
    OutOfMemory,
    StackUnderflow,
    OverFlow,

    // custom define error
    ShaderCompileFailed,
    ShaderLinkFailed,
    CreateShaderProgramFailed,
    CreateShaderFailed,
    CreateVertexAttributeFailed,
    CreateBufferFailed,
    InvalidCStr,

    Unknown,
}

pub fn gl_get_top_error() -> GLErrorType {
    unsafe {
        let err = gl::GetError();
        if err >= GLErrorType::Unknown as u32 {
            GLErrorType::Unknown
        } else {
            std::mem::transmute(err as u8)
        }
    }
}

pub fn gl_clear_error() {
    const MAX_LOOP_COUNT: u32 = 1000;
    let mut i = 0;
    let mut err = gl_get_top_error();
    while i < MAX_LOOP_COUNT && err != GLErrorType::NoError {
        err = gl_get_top_error();
        i += 1
    }
}

pub fn gl_get_error() -> GLErrorType {
    let err = gl_get_top_error();
    gl_clear_error();
    err
}

pub type GLResult<T> = Result<T, GLErrorType>;

#[macro_export]
macro_rules! gl_call {
    ($expr: expr) => {
        unsafe {
            $crate::glhelper::gl_clear_error();
            let result = $expr;
            let error = $crate::glhelper::gl_get_error();
            if error == $crate::glhelper::GLErrorType::NoError {
                Ok(result)
            } else {
                log::error!(target: "OpenGL", "Occured Error: {:?}", error);
                Err(error)
            }
        }
    };
}
