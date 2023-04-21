use crate::{gl_call, glhelper::GLResult};
pub struct PhysicalDevice;

impl PhysicalDevice {
    pub fn new() -> Self {
        Self {}
    }
}

impl PhysicalDevice {
    pub fn get_vendor(&self) -> GLResult<String> {
        let vendor = gl_call!(gl::GetString(gl::VENDOR))?;
        Ok(String::from(
            std::str::from_utf8(unsafe {
                std::ffi::CStr::from_ptr(vendor as *const i8).to_bytes()
            })
            .unwrap(),
        ))
    }
}
