use crate::{error, gl_call, glhelper::GLResult, pipeline::*};

pub enum CommandType {
    DrawArrays,
}

pub struct Command {}

impl Command {
    pub fn bind_pipeline(&self, pipeline: &Pipeline) -> Result<(), error::Error> {
        match pipeline {
            Pipeline::Graphics(pipeline) => self.apply_graphics_pipeline(pipeline),
        }
    }

    fn apply_graphics_pipeline(&self, pipeline: &GraphicsPipeline) -> Result<(), error::Error> {
        self.apply_gl_graphics_pipeline(pipeline)
            .map_err(|_| error::Error::BindPipelineFailed)
    }

    fn apply_gl_graphics_pipeline(&self, pipeline: &GraphicsPipeline) -> GLResult<()> {
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

fn cullface2glenum(face_cull: FaceCull) -> Option<u32> {
    match face_cull {
        FaceCull::None => None,
        FaceCull::Front => Some(gl::FRONT),
        FaceCull::Back => Some(gl::BACK),
    }
}

fn frontface2glenum(face: FrontFace) -> u32 {
    match face {
        FrontFace::CW => gl::CW,
        FrontFace::CCW => gl::CCW,
    }
}

fn polygonmod2glenum(mode: PolygonMode) -> u32 {
    match mode {
        PolygonMode::Point => gl::POINT,
        PolygonMode::Line => gl::LINE,
        PolygonMode::Fill => gl::FILL,
    }
}
