use math::matrix::*;
use std::{
    fs,
    io::{self, Write},
};

pub struct PPM {
    data: Vec<Vec3>,
    w: usize,
    h: usize,
}

impl PPM {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            data: vec![Vec3::zeros(); w * h],
            w,
            h,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Vec3) {
        self.data[x + y * self.w] = pixel;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Vec3 {
        self.data[x + y * self.w]
    }

    pub fn width(&self) -> usize {
        self.w
    }

    pub fn height(&self) -> usize {
        self.h
    }

    pub fn write_to_file(&self, filename: &str) -> Result<(), io::Error> {
        let mut file = fs::File::create(filename)?;
        writeln!(&mut file, "P3")?;
        writeln!(&mut file, "{} {}", self.w, self.h)?;
        writeln!(&mut file, "255")?;
        for y in 0..self.h {
            for x in 0..self.w {
                let color = self.get_pixel(x, y) * 255.0;
                write!(
                    &mut file,
                    "{} {} {}  ",
                    color.x().clamp(0.0, 255.0) as u8,
                    color.y().clamp(0.0, 255.0) as u8,
                    color.z().clamp(0.0, 255.0) as u8
                )?;
            }
            writeln!(&mut file, "")?;
        }
        Ok(())
    }
}
