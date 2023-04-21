use crate::buffer::*;
use crate::camera;
use crate::gl_call;
use crate::glhelper::GLResult;
use crate::shader::Shader;
use crate::shader::ShaderModule;
use crate::shader::ShaderType;
use crate::vertex_attr::*;
use gl;
use math;

static VERTEX_SHADER_CODE: &str = r#"#version 330 core

layout(location = 0) in vec3 aPosition;

uniform mat4 model;
uniform mat4 view;
uniform mat4 project;

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
    clear_color: math::cg::Color,
    vao: VertexAttribute,
    vbo: Buffer,
    ebo: Buffer,
    shader: Shader,
    camera: camera::Camera,
}

pub enum RenderType {
    Solid,
    Framework,
}

impl Renderer {
    pub fn new(w: i32, h: i32, camera: camera::Camera) -> GLResult<Self> {
        let mut bunch = AttrBunch::default();
        bunch.add(Attribute {
            attrib_type: AttribType::Float,
            count: 3,
        });

        let shader = Shader::new(
            &ShaderModule::new(ShaderType::Vertex, VERTEX_SHADER_CODE)?,
            &ShaderModule::new(ShaderType::Fragment, FRAG_SHADER_CODE)?,
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
            clear_color: math::cg::Color::white(),
            vbo,
            ebo,
            vao,
            shader,
            camera,
        })
    }

    pub fn set_render_type(render_type: RenderType) -> GLResult<()> {
        match render_type {
            RenderType::Solid => gl_call!(gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL))?,
            RenderType::Framework => gl_call!(gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE))?,
        }
        Ok(())
    }

    pub fn resize(&self, w: i32, h: i32) -> GLResult<()> {
        gl_call!(gl::Viewport(0, 0, w, h))?;
        Ok(())
    }

    pub fn set_clear_color(&mut self, color: math::cg::Color) {
        self.clear_color = color;
        gl_call!(gl::ClearColor(
            self.clear_color.r(),
            self.clear_color.g(),
            self.clear_color.b(),
            self.clear_color.a(),
        ))
        .unwrap();
    }

    pub fn clear(&mut self) {
        gl_call!(gl::Clear(
            gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT
        ))
        .unwrap();
    }

    pub fn draw_lines(
        &mut self,
        vertices: &[math::matrix::Vec3],
        model: &math::matrix::Mat44,
        color: &math::matrix::Vec4,
    ) -> GLResult<()> {
        self.vao.bind()?;

        self.vbo.bind()?;
        let (_, datas, _) = unsafe { vertices.align_to::<u8>() };
        self.vbo.buffer_data(datas)?;

        self.shader.use_shader()?;
        self.shader.set_mat4("project", self.camera.get_project())?;
        self.shader.set_mat4("view", self.camera.get_view())?;
        self.shader.set_mat4("model", model)?;
        self.shader.set_vec4("color", color)?;

        gl_call!(gl::DrawArrays(
            gl::LINES,
            0,
            vertices.len().try_into().unwrap()
        ))?;

        Ok(())
    }

    pub fn draw_linestrip(
        &mut self,
        vertices: &[math::matrix::Vec3],
        model: &math::matrix::Mat44,
        color: &math::matrix::Vec4,
    ) -> GLResult<()> {
        self.vao.bind()?;

        self.vbo.bind()?;
        let (_, datas, _) = unsafe { vertices.align_to::<u8>() };
        self.vbo.buffer_data(datas)?;

        self.shader.use_shader()?;
        self.shader.set_mat4("project", self.camera.get_project())?;
        self.shader.set_mat4("view", self.camera.get_view())?;
        self.shader.set_mat4("model", model)?;
        self.shader.set_vec4("color", color)?;

        gl_call!(gl::DrawArrays(
            gl::LINE_STRIP,
            0,
            vertices.len().try_into().unwrap()
        ))?;

        Ok(())
    }

    pub fn draw_lineloop(
        &mut self,
        vertices: &[math::matrix::Vec3],
        model: &math::matrix::Mat44,
        color: &math::matrix::Vec4,
    ) -> GLResult<()> {
        self.vao.bind()?;

        self.vbo.bind()?;
        let (_, datas, _) = unsafe { vertices.align_to::<u8>() };
        self.vbo.buffer_data(datas)?;

        self.shader.use_shader()?;
        self.shader.set_mat4("project", self.camera.get_project())?;
        self.shader.set_mat4("view", self.camera.get_view())?;
        self.shader.set_mat4("model", model)?;
        self.shader.set_vec4("color", color)?;

        gl_call!(gl::DrawArrays(
            gl::LINE_LOOP,
            0,
            vertices.len().try_into().unwrap()
        ))?;

        Ok(())
    }

    pub fn draw_arrays(
        &mut self,
        vertices: &[Vertex],
        model: &math::matrix::Mat44,
        color: &math::matrix::Vec4,
    ) -> GLResult<()> {
        self.vao.bind()?;

        self.vbo.bind()?;
        let (_, datas, _) = unsafe { vertices.align_to::<u8>() };
        self.vbo.buffer_data(datas)?;

        self.shader.use_shader()?;
        self.shader.set_mat4("project", self.camera.get_project())?;
        self.shader.set_mat4("view", self.camera.get_view())?;
        self.shader.set_mat4("model", model)?;
        self.shader.set_vec4("color", color)?;

        gl_call!(gl::DrawArrays(
            gl::TRIANGLES,
            0,
            vertices.len().try_into().unwrap()
        ))?;

        Ok(())
    }

    pub fn draw_elements(
        &mut self,
        vertices: &[Vertex],
        indices: &[u32],
        model: &math::matrix::Mat44,
        color: &math::matrix::Vec4,
    ) -> GLResult<()> {
        self.vao.bind()?;

        self.vbo.bind()?;
        self.ebo.bind()?;
        let (_, datas, _) = unsafe { vertices.align_to::<u8>() };
        self.vbo.buffer_data(datas)?;
        let (_, datas, _) = unsafe { indices.align_to::<u8>() };
        self.ebo.buffer_data(datas)?;

        self.shader.use_shader()?;
        self.shader.set_mat4("project", self.camera.get_project())?;
        self.shader.set_mat4("view", self.camera.get_view())?;
        self.shader.set_mat4("model", model)?;
        self.shader.set_vec4("color", color)?;

        gl_call!(gl::DrawElements(
            gl::TRIANGLES,
            vertices.len().try_into().unwrap(),
            gl::UNSIGNED_INT,
            std::ptr::null()
        ))?;

        Ok(())
    }

    pub fn cleanup(&mut self) {
        self.ebo.cleanup();
        self.vao.cleanup();
        self.vbo.cleanup();
        self.shader.cleanup();
    }
}
