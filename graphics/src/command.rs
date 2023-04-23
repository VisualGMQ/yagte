use crate::{pipeline::*, buffer::Buffer};

pub(crate) enum DrawType {
    Arrays,
    Elements,
}

pub(crate) struct DrawInfo {
    pub(crate) dtype: DrawType,
    pub(crate) offset: i32,
    pub(crate) count: i32,
}

pub struct Command<'a> {
    pub(crate) draw_info: Option<DrawInfo>,
    pub(crate) pipeline: Option<&'a Pipeline>,
    pub(crate) vertex_buffer: Option<&'a Buffer>,
    pub(crate) index_buffer: Option<&'a Buffer>,
}

impl<'a> Command<'a> {
    pub fn new() -> Self {
        Self { draw_info: None, pipeline: None, vertex_buffer: None, index_buffer: None }
    }

    pub fn bind_pipeline(&mut self, pipeline: &'a Pipeline) {
        self.pipeline = Some(pipeline);
    }
    
    pub fn draw_arrays(&mut self, first: i32, count: i32) {
        self.draw_info = Some(DrawInfo {
            offset: first,
            dtype: DrawType::Arrays,
            count,
        });
    }

    pub fn draw_elements(&mut self, count: i32, offset: i32) {
        self.draw_info = Some(DrawInfo {
            dtype: DrawType::Elements,
            offset,
            count,
        });
    }

    pub fn bind_vertex_buffer(&mut self, buffer: &'a Buffer) {
        self.vertex_buffer = Some(buffer);
    }

    pub fn bind_indices_buffer(&mut self, buffer: &'a Buffer) {
        self.index_buffer = Some(buffer);
    }

    pub fn reset(&mut self) {
        self.vertex_buffer = None;
        self.index_buffer = None;
        self.pipeline = None;
    }
}