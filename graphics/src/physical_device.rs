use std::str::Utf8Error;

use crate::{gl_call, glhelper::{GLResult, GLErrorType}};
pub struct PhysicalDevice;

impl PhysicalDevice {
    pub fn new() -> Self {
        Self {}
    }
}

impl PhysicalDevice {
    pub fn get_vendor(&self) -> GLResult<String> {
        let vendor = gl_call!(gl::GetString(gl::VENDOR))?;
        string_from_c(vendor).map_err(|_|GLErrorType::InvalidCStr)
    }

    pub fn get_renderer(&self) -> GLResult<String> {
        let renderer = gl_call!(gl::GetString(gl::RENDERER))?;
        string_from_c(renderer).map_err(|_|GLErrorType::InvalidCStr)
    }

    pub fn get_version(&self) -> GLResult<String> {
        let version = gl_call!(gl::GetString(gl::VERSION))?;
        string_from_c(version).map_err(|_|GLErrorType::InvalidCStr)
    }
 
    pub fn get_shading_language_version(&self) -> GLResult<String> {
        let version = gl_call!(gl::GetString(gl::SHADING_LANGUAGE_VERSION))?;
        string_from_c(version).map_err(|_|GLErrorType::InvalidCStr)
    }

    pub fn enumerate_extensions(&self) -> GLResult<Option<Vec<String>>> {
        let mut count: i32 = 0;
        gl_call!(gl::GetIntegerv(gl::NUM_EXTENSIONS, &mut count as *mut i32))?;
        if count <= 0 {
            Ok(None)
        } else {
            let mut extensions: Vec<String> = Vec::new();
            for i in 0..count {
                let i = i as u32;
                let ext = gl_call!(gl::GetStringi(gl::EXTENSIONS, i))?;
                extensions.push(string_from_c(ext).map_err(|_|GLErrorType::InvalidCStr)?)
            }
            Ok(Some(extensions))
        }
    }
}

fn string_from_c(s: *const u8) -> Result<String, Utf8Error> {
    Ok(String::from(
        std::str::from_utf8(unsafe {
            std::ffi::CStr::from_ptr(s as *const i8).to_bytes()
        })?
    ))
}