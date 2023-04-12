use crate::gl_call;
use crate::glhelper::GLResult;
use gl;

#[derive(Debug, Clone, Copy)]
pub enum AttribType {
    Float,
    Double,
    U32,
    U8,
    I32,
    I8,
}

fn get_attribtype_size(t: AttribType) -> usize {
    match t {
        AttribType::Float => 4,
        AttribType::Double => 8,
        AttribType::U32 | AttribType::I32 => 4,
        AttribType::U8 | AttribType::I8 => 1,
    }
}

fn get_attribtype_gl_type(t: AttribType) -> u32 {
    match t {
        AttribType::Float => gl::FLOAT,
        AttribType::Double => gl::DOUBLE,
        AttribType::U32 => gl::UNSIGNED_INT,
        AttribType::I32 => gl::INT,
        AttribType::U8 => gl::UNSIGNED_BYTE,
        AttribType::I8 => gl::BYTE,
    }
}

pub struct Attribute {
    pub(crate) attrib_type: AttribType,
    pub(crate) count: u32,
}

pub struct AttrBunch {
    attrs: Vec<Attribute>,
    stride: u32,
}

impl AttrBunch {
    pub fn new() -> Self {
        Self {
            attrs: Vec::new(),
            stride: 0,
        }
    }

    pub fn add(&mut self, attr: Attribute) {
        self.stride += u32::try_from(get_attribtype_size(attr.attrib_type)).unwrap();
        self.attrs.push(attr);
    }

    pub fn stride(&self) -> u32 {
        self.stride
    }

    pub fn iter(&self) -> AttrBunchIterator {
        AttrBunchIterator {
            bunch: &self,
            index: 0,
        }
    }
}

pub struct AttrBunchIterator<'a> {
    bunch: &'a AttrBunch,
    index: usize,
}

impl<'a> Iterator for AttrBunchIterator<'a> {
    type Item = &'a Attribute;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.bunch.attrs.len() {
            let result = Some(&self.bunch.attrs[self.index]);
            self.index += 1;
            return result;
        }
        None
    }
}

pub struct VertexAttribute {
    id: u32,
}

impl VertexAttribute {
    pub fn new( attributes: &AttrBunch) -> GLResult<Self> {
        let mut id: u32 = 0;

        gl_call!(gl::GenVertexArrays(1, &mut id as *mut u32))?;
        gl_call!(gl::BindVertexArray(id))?;

        let mut i = 0;
        for attribute in attributes.iter() {
            gl_call!(gl::VertexAttribPointer(
                i,
                get_attribtype_size(attribute.attrib_type)
                    .try_into()
                    .unwrap(),
                get_attribtype_gl_type(attribute.attrib_type),
                0,
                attributes.stride().try_into().unwrap(),
                0 as *const _
            ))?;
            gl_call!(gl::EnableVertexAttribArray(i))?;
            i += 1;
        }


        Ok(Self { id })
    }

    pub fn bind(&self) -> GLResult<()> {
        gl_call!(gl::BindVertexArray(self.id))?;
        Ok(())
    }

    pub fn unbind(&self) -> GLResult<()> {
        gl_call!(gl::BindVertexArray(0))?;
        Ok(())
    }
}

impl Drop for VertexAttribute {
    fn drop(&mut self) {
        gl_call!(gl::DeleteVertexArrays(1, &self.id as *const u32)).unwrap();
    }
}