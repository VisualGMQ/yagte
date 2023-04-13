use std::ptr::null;

use crate::{
    gl_call,
    glhelper::{GLErrorType, GLResult},
    shader,
};

#[derive(Clone, Copy, Debug)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

pub struct ShaderUnit {
    stype: ShaderType,
    id: u32,
}

fn shadertype2glenum(t: ShaderType) -> u32 {
    match t {
        ShaderType::Vertex => gl::VERTEX_SHADER,
        ShaderType::Fragment => gl::FRAGMENT_SHADER,
    }
}

impl ShaderUnit {
    pub fn new(t: ShaderType, source: &str) -> GLResult<Self> {
        let id = gl_call!(gl::CreateShader(shadertype2glenum(t)))?;
        if id == 0 {
            return Err(GLErrorType::CreateShaderFailed);
        }
        gl_call!(gl::ShaderSource(
            id,
            1,
            &(source.as_bytes().as_ptr().cast()),
            &(source.len().try_into().unwrap())
        ))?;
        gl_call!(gl::CompileShader(id))?;

        let mut err = 0;
        gl_call!(gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut err as *mut _))?;
        if err == 0 {
            let mut buf = [0i8; 1024];
            gl_call!(gl::GetShaderInfoLog(id, 1024, 0 as _, &mut buf as *mut _))?;
            log::error!(
                "{}",
                String::from_utf8(buf.iter().map(|&c| c as u8).collect()).unwrap()
            );
            return Err(GLErrorType::ShaderCompileFailed);
        }

        Ok(Self { stype: t, id })
    }
}

impl Drop for ShaderUnit {
    fn drop(&mut self) {
        gl_call!(gl::DeleteShader(self.id)).unwrap();
    }
}

pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn new(vertex_shader: &ShaderUnit, frag_shader: &ShaderUnit) -> GLResult<Self> {
        let id = gl_call!(gl::CreateProgram())?;
        if id == 0 {
            return Err(GLErrorType::CreateShaderProgramFailed);
        }
        gl_call!(gl::AttachShader(id, vertex_shader.id))?;
        gl_call!(gl::AttachShader(id, frag_shader.id))?;
        gl_call!(gl::LinkProgram(id))?;
        let mut err = 0;
        gl_call!(gl::GetProgramiv(id, gl::LINK_STATUS, &mut err as _))?;
        if err == 0 {
            return Err(GLErrorType::ShaderLinkFailed);
        }
        gl_call!(gl::DetachShader(id, vertex_shader.id))?;
        gl_call!(gl::DetachShader(id, frag_shader.id))?;

        Ok(Self { id })
    }

    pub fn bind(&self) -> GLResult<()> {
        gl_call!(gl::BindProgramPipeline(self.id))?;
        Ok(())
    }

    pub fn unbind(&self) -> GLResult<()> {
        gl_call!(gl::BindProgramPipeline(0))?;
        Ok(())
    }

    pub fn set_mat4(&self, name: &str, m: &math::matrix::Mat44) -> GLResult<()> {
        let loc = gl_call!(gl::GetUniformLocation(self.id, name.as_ptr() as *const i8))?;
        if loc < 0 {
            log::warn!("non-exists uniform variable: {}", name);
        } else {
            gl_call!(gl::ProgramUniformMatrix4fv(
                self.id,
                loc,
                1,
                gl::FALSE.into(),
                m.as_ptr()
            ))?;
        }
        Ok(())
    }

    pub fn set_mat3(&self, name: &str, m: &math::matrix::Mat44) -> GLResult<()> {
        let loc = gl_call!(gl::GetUniformLocation(self.id, name.as_ptr() as *const i8))?;
        if loc < 0 {
            log::warn!("non-exists uniform variable: {}", name);
        } else {
            gl_call!(gl::ProgramUniformMatrix3fv(
                self.id,
                loc,
                1,
                gl::FALSE.into(),
                m.as_ptr()
            ))?;
        }
        Ok(())
    }

    pub fn set_vec2(&self, name: &str, v: &math::matrix::Vec2) -> GLResult<()> {
        let loc = gl_call!(gl::GetUniformLocation(self.id, name.as_ptr() as *const i8))?;
        if loc < 0 {
            log::warn!("non-exists uniform variable: {}", name);
        } else {
            gl_call!(gl::ProgramUniform2fv(self.id, loc, 1, v.as_ptr()))?;
        }
        Ok(())
    }

    pub fn set_vec3(&self, name: &str, v: &math::matrix::Vec3) -> GLResult<()> {
        let loc = gl_call!(gl::GetUniformLocation(self.id, name.as_ptr() as *const i8))?;
        if loc < 0 {
            log::warn!("non-exists uniform variable: {}", name);
        } else {
            gl_call!(gl::ProgramUniform3fv(self.id, loc, 1, v.as_ptr()))?;
        }
        Ok(())
    }

    pub fn set_vec4(&self, name: &str, v: &math::matrix::Vec4) -> GLResult<()> {
        let loc = gl_call!(gl::GetUniformLocation(self.id, name.as_ptr() as *const i8))?;
        if loc < 0 {
            log::warn!("non-exists uniform variable: {}", name);
        } else {
            gl_call!(gl::ProgramUniform4fv(self.id, loc, 1, v.as_ptr()))?;
        }
        Ok(())
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        gl_call!(gl::DeleteProgram(self.id)).unwrap();
    }
}
