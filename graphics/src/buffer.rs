use gl;

use crate::gl_call;
use crate::glhelper::{GLResult, GLErrorType};

#[derive(Debug, Clone, Copy)]
pub enum BufferType {
    ArrayBuffer,
    ElementBuffer,
}

#[derive(Debug)]
pub struct Buffer {
    btype: BufferType,
    id: u32,
}

impl Buffer {
    pub fn new(btype: BufferType) -> GLResult<Buffer> {
        let mut id: u32 = 0;
        gl_call!(gl::GenBuffers(1, &mut id as *mut u32))?;

        if id == 0 {
            return Err(GLErrorType::CreateBufferFailed);
        }

        Ok(Self { btype, id })
    }

    pub fn bind(&self) -> GLResult<()> {
        gl_call!(gl::BindBuffer(buffertype2u32(self.btype), self.id))?;
        Ok(())
    }

    pub fn unbind(&self) -> GLResult<()> {
        gl_call!(gl::BindBuffer(buffertype2u32(self.btype), 0))?;
        Ok(())
    }

    pub fn buffer_data(&self, data: &[u8]) -> GLResult<()> {
        self.bind()?;
        gl_call!(gl::BufferData(
            buffertype2u32(self.btype),
            std::mem::size_of_val(data) as isize,
            data.as_ptr().cast(),
            gl::STATIC_DRAW
        ))?;
        Ok(())
    }

    pub fn get_type(&self) -> BufferType {
        self.btype
    }
}

fn buffertype2u32(t: BufferType) -> u32 {
    match t {
        BufferType::ArrayBuffer => gl::ARRAY_BUFFER,
        BufferType::ElementBuffer => gl::ELEMENT_ARRAY_BUFFER,
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        gl_call!(gl::DeleteBuffers(1, &self.id as *const u32)).unwrap();
    }
}
