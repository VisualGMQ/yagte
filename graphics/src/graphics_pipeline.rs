use crate::{
    error,
    shader::{self, ShaderType},
};

#[derive(Clone, Copy)]
pub enum Topology {
    Triangles,
    Lines,
    LineStrip,
    LineLoop,
}

#[derive(Clone, Copy)]
pub struct InputAssembly {
    pub topology: Topology,
    pub primitive_restart: bool,
}

#[derive(Clone, Copy)]
pub enum FaceCull {
    None,
    Front,
    Back,
}

#[derive(Clone, Copy)]
pub enum FrontFace {
    CW,
    CCW,
}

#[derive(Clone, Copy)]
pub enum PolygonMode {
    Point,
    Line,
    Fill,
}

#[derive(Clone, Copy)]
pub struct Rasterizer {
    pub face_cull: FaceCull,
    pub front_face: FrontFace,
    pub line_width: f32,
    pub polygon_mode: PolygonMode,
}

#[derive(Clone, Copy)]
pub struct Multisample {
    pub enable: bool,
}

#[derive(Clone, Copy)]
pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

pub struct GraphicsPipeline {
    pub(crate) input_asm: InputAssembly,
    pub(crate) shader: shader::Shader,
    pub(crate) viewport: Viewport,
    pub(crate) raster: Rasterizer,
    pub(crate) multisample: Multisample,
}

impl GraphicsPipeline {
    pub(crate) fn cleanup(&mut self) {
        self.shader.cleanup();
    }
}

pub struct PipelineBuilder<'a> {
    input_asm: Option<InputAssembly>,
    shader_modules: Option<&'a [shader::ShaderModule]>,
    viewport: Option<Viewport>,
    raster: Option<Rasterizer>,
    multisample: Option<Multisample>,
}

impl<'a> PipelineBuilder<'a> {
    pub fn new() -> Self {
        Self {
            input_asm: None,
            shader_modules: None,
            viewport: None,
            raster: None,
            multisample: None,
        }
    }

    pub fn with_input_assembly(&mut self, input: InputAssembly) -> &mut Self {
        self.input_asm = Some(input);
        self
    }

    pub fn with_shader(&mut self, shaders: &'a [shader::ShaderModule]) -> &mut Self {
        self.shader_modules = Some(shaders);
        self
    }

    pub fn with_viewport(&mut self, viewport: Viewport) -> &mut Self {
        self.viewport = Some(viewport);
        self
    }

    pub fn with_rasterizer(&mut self, rasterizer: Rasterizer) -> &mut Self {
        self.raster = Some(rasterizer);
        self
    }

    pub fn with_multisample(&mut self, multisample: Multisample) -> &mut Self {
        self.multisample = Some(multisample);
        self
    }

    pub fn build(&self) -> Result<GraphicsPipeline, error::Error> {
        let err = error::Error::ParamNotEnough;
        let shader_modules = self.shader_modules.ok_or(err)?;
        let shader = shader::Shader::new(
            shader_modules
                .iter()
                .filter(|s| s.stype == ShaderType::Vertex)
                .next()
                .ok_or(err)?,
            shader_modules
                .iter()
                .filter(|s| s.stype == ShaderType::Fragment)
                .next()
                .ok_or(err)?,
        )
        .map_err(|_| error::Error::ParamNotEnough)?;
        Ok(GraphicsPipeline {
            input_asm: self.input_asm.ok_or(err)?,
            shader,
            viewport: self.viewport.ok_or(err)?,
            raster: self.raster.ok_or(err)?,
            multisample: self.multisample.ok_or(err)?,
        })
    }
}
