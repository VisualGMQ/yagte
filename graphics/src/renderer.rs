use crate::buffer::*;
use crate::gl_call;
use crate::glhelper::GLResult;
use crate::vertex_attr::*;
use gl;
use math;

pub struct Renderer {
    color: math::cg::Color,
    vao: VertexAttribute,
    vbo: Buffer,
    ebo: Buffer,
}

impl Renderer {
    pub fn new() -> GLResult<Self> {
        let mut bunch = AttrBunch::new();
        bunch.add(Attribute {
            attrib_type: AttribType::Float,
            count: 2,
        });

        Ok(Self {
            color: math::cg::Color::white(),
            vao: VertexAttribute::new(&bunch)?,
            vbo: Buffer::new(BufferType::ArrayBuffer)?,
            ebo: Buffer::new(BufferType::ElementBuffer)?,
        })
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

    pub fn cleanup(&mut self) {
        drop(&mut self.ebo);
        drop(&mut self.vbo);
        drop(&mut self.vao);
    }
}
