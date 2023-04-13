use crate::buffer::*;
use crate::gl_call;
use crate::glhelper::GLResult;
use crate::shader::Shader;
use crate::shader::ShaderType;
use crate::shader::ShaderUnit;
use crate::vertex_attr::*;
use gl;
use math;

static VERTEX_SHADER_CODE: &str = r#"#version 330 core

layout(location = 0) in vec3 aPosition;

uniform mat4 project;
uniform mat4 model;
uniform mat4 view;

void main() {
    gl_Position = project * model * view * vec4(aPosition, 1.0);
}
"#;

static FRAG_SHADER_CODE: &str = r#"#version 330 core

out vec4 FragColor;

uniform vec4 color;

void main() {
    FragColor = color;
}
"#;

pub struct Renderer {
    color: math::cg::Color,
    vao: VertexAttribute,
    vbo: Buffer,
    ebo: Buffer,
    shader: Shader,
}

impl Renderer {
    pub fn new() -> GLResult<Self> {
        let mut bunch = AttrBunch::new();
        bunch.add(Attribute {
            attrib_type: AttribType::Float,
            count: 2,
        });

        let shader = Shader::new(
            &ShaderUnit::new(ShaderType::Vertex, VERTEX_SHADER_CODE)?,
            &ShaderUnit::new(ShaderType::Fragment, FRAG_SHADER_CODE)?,
        )?;

        Ok(Self {
            color: math::cg::Color::white(),
            vao: VertexAttribute::new(&bunch)?,
            vbo: Buffer::new(BufferType::ArrayBuffer)?,
            ebo: Buffer::new(BufferType::ElementBuffer)?,
            shader,
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
        drop(&mut self.shader);
    }
}
