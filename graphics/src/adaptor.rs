use std::{str::Utf8Error, ptr::null};

use crate::{gl_call, glhelper::{GLResult, GLErrorType}, command::{Command, self}, error, pipeline::*};
pub struct Adaptor;

impl Adaptor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Adaptor {
    pub fn get_vendor(&self) -> GLResult<String> {
        let vendor = gl_call!(gl::GetString(gl::VENDOR))?;
        string_from_c(vendor).map_err(|_|GLErrorType::InvalidCStr)
    }

    pub fn get_renderer(&self) -> GLResult<String> {
        let renderer = gl_call!(gl::GetString(gl::RENDERER))?;
        string_from_c(renderer).map_err(|_|GLErrorType::InvalidCStr)
    }

    pub fn get_version(&self) -> GLResult<String> {
        let version = gl_call!(gl::GetString(gl::VERSION))?;
        string_from_c(version).map_err(|_|GLErrorType::InvalidCStr)
    }
 
    pub fn get_shading_language_version(&self) -> GLResult<String> {
        let version = gl_call!(gl::GetString(gl::SHADING_LANGUAGE_VERSION))?;
        string_from_c(version).map_err(|_|GLErrorType::InvalidCStr)
    }

    pub fn enumerate_extensions(&self) -> GLResult<Option<Vec<String>>> {
        let mut count: i32 = 0;
        gl_call!(gl::GetIntegerv(gl::NUM_EXTENSIONS, &mut count as *mut i32))?;
        if count <= 0 {
            Ok(None)
        } else {
            let mut extensions: Vec<String> = Vec::new();
            for i in 0..count {
                let i = i as u32;
                let ext = gl_call!(gl::GetStringi(gl::EXTENSIONS, i))?;
                extensions.push(string_from_c(ext).map_err(|_|GLErrorType::InvalidCStr)?)
            }
            Ok(Some(extensions))
        }
    }

    pub fn submit(&self, cmd: &Command) -> Result<(), error::Error> {
        if let Some(buffer) = cmd.vertex_buffer {
            buffer.bind().unwrap();
        }
        if let Some(buffer) = cmd.index_buffer {
            buffer.bind().unwrap();
        }

        if let Some(Pipeline::Graphics(pipeline)) = cmd.pipeline {
            self.bind_graphics_pipeline(pipeline)?;

            if let Some(info) = &cmd.draw_info {
                match info.dtype {
                    command::DrawType::Arrays => {
                        gl_call!(gl::DrawArrays(topology2glenum(pipeline.input_asm.topology), info.offset, info.count)).map_err(|_|error::Error::CommandExecuteFailed)?;
                    },
                    command::DrawType::Elements => {
                        gl_call!(gl::DrawElements(topology2glenum(pipeline.input_asm.topology), info.count, gl::UNSIGNED_INT, null())).map_err(|_|error::Error::CommandExecuteFailed)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn bind_graphics_pipeline(&self, pipeline: &GraphicsPipeline) -> Result<(), error::Error> {
        self.bind_gl_graphics_pipeline(pipeline)
            .map_err(|_| error::Error::BindPipelineFailed)
    }

    fn bind_gl_graphics_pipeline(&self, pipeline: &GraphicsPipeline) -> GLResult<()> {
        gl_call!(gl::Viewport(
            pipeline.viewport.x,
            pipeline.viewport.y,
            pipeline.viewport.w,
            pipeline.viewport.h
        ))?;

        if pipeline.multisample.enable {
            gl_call!(gl::Enable(gl::MULTISAMPLE))?;
        } else {
            gl_call!(gl::Disable(gl::MULTISAMPLE))?;
        }

        let raster = &pipeline.raster;
        let face_cull = cullface2glenum(raster.face_cull);
        match face_cull {
            Some(cull) => {
                gl_call!(gl::Enable(gl::CULL_FACE))?;
                gl_call!(gl::CullFace(cull))?;
            }
            None => {
                gl_call!(gl::Disable(gl::CULL_FACE))?;
            }
        }

        gl_call!(gl::FrontFace(frontface2glenum(raster.front_face)))?;
        gl_call!(gl::PolygonMode(
            gl::FRONT_AND_BACK,
            polygonmod2glenum(raster.polygon_mode)
        ))?;
        Ok(())
    }

}

fn string_from_c(s: *const u8) -> Result<String, Utf8Error> {
    Ok(String::from(
        std::str::from_utf8(unsafe {
            std::ffi::CStr::from_ptr(s as *const i8).to_bytes()
        })?
    ))
}
