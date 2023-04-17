use crate::buffer::*;
use crate::gl_call;
use crate::glhelper::GLResult;
use crate::shader::Shader;
use crate::shader::ShaderType;
use crate::shader::ShaderUnit;
use crate::vertex_attr::*;
use gl;
use math;

/*
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
*/

static VERTEX_SHADER_CODE: &str = r#"#version 330 core

layout(location = 0) in vec3 aPosition;

void main() {
    gl_Position = vec4(aPosition, 1.0);
}
"#;

static FRAG_SHADER_CODE: &str = r#"#version 330 core

out vec4 FragColor;

void main() {
    FragColor = vec4(0.0, 1.0, 0.0, 1.0);
}
"#;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vertex {
    position: math::matrix::Vec3,
}

impl Vertex {
    pub fn new(position: math::matrix::Vec3) -> Self {
        Self { position }
    }
}

pub struct Renderer {
    color: math::cg::Color,
    vao: VertexAttribute,
    vbo: Buffer,
    ebo: Buffer,
    shader: Shader,
}

impl Renderer {
    pub fn new(w: i32, h: i32) -> GLResult<Self> {
        let mut bunch = AttrBunch::new();
        bunch.add(Attribute {
            attrib_type: AttribType::Float,
            count: 3,
        });

        let shader = Shader::new(
            &ShaderUnit::new(ShaderType::Vertex, VERTEX_SHADER_CODE)?,
            &ShaderUnit::new(ShaderType::Fragment, FRAG_SHADER_CODE)?,
        )?;

        gl_call!(gl::Viewport(0, 0, w, h))?;
        gl_call!(gl::Enable(gl::DEPTH_TEST))?;
        gl_call!(gl::Enable(gl::STENCIL_TEST))?;

        gl_call!(gl::FrontFace(gl::CCW))?;
        gl_call!(gl::CullFace(gl::BACK))?;

        let vbo = Buffer::new(BufferType::ArrayBuffer)?;
        vbo.bind()?;
        let ebo = Buffer::new(BufferType::ElementBuffer)?;
        ebo.bind()?;
        let vao = VertexAttribute::new(&bunch)?;

        Ok(Self {
            color: math::cg::Color::white(),
            vbo,
            ebo,
            vao,
            shader,
        })
    }

    pub fn resize(&self, w: i32, h: i32) -> GLResult<()> {
        gl_call!(gl::Viewport(0, 0, w, h))?;
        Ok(())
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

    pub fn draw_arrays(&mut self, vertices: &[Vertex]) -> GLResult<()> {
        self.vao.bind()?;

        self.vbo.bind()?;
        let (_, datas, _) = unsafe { vertices.align_to::<u8>() };
        self.vbo.buffer_data(datas)?;

        self.shader.use_shader()?;

        gl_call!(gl::DrawArrays(
            gl::TRIANGLES,
            0,
            vertices.len().try_into().unwrap()
        ))?;

        Ok(())
    }

    pub fn draw_elements(&mut self, vertices: &[Vertex], indices: &[u32]) -> GLResult<()> {
        self.vao.bind()?;

        self.vbo.bind()?;
        self.ebo.bind()?;
        let (_, datas, _) = unsafe { vertices.align_to::<u8>() };
        self.vbo.buffer_data(datas)?;
        let (_, datas, _) = unsafe { indices.align_to::<u8>() };
        self.ebo.buffer_data(datas)?;

        self.shader.use_shader()?;

        gl_call!(gl::DrawElements(
            gl::TRIANGLES,
            vertices.len().try_into().unwrap(),
            gl::UNSIGNED_INT,
            std::ptr::null()
        ))?;

        Ok(())
    }

    pub fn cleanup(&mut self) {
        drop(&mut self.ebo);
        drop(&mut self.vbo);
        drop(&mut self.vao);
        drop(&mut self.shader);
    }
}
