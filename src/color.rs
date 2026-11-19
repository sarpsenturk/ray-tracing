use crate::vec3::Vec3;

use std::io::{self, Write};

pub type Color = Vec3;

pub fn write_color<T: Write>(output: &mut T, color: &Color) -> io::Result<()> {
    let r = (255.0 * linear_to_gamma(color.x).clamp(0.0, 1.0)) as i32;
    let g = (255.0 * linear_to_gamma(color.y).clamp(0.0, 1.0)) as i32;
    let b = (255.0 * linear_to_gamma(color.z).clamp(0.0, 1.0)) as i32;
    write!(output, "{r} {g} {b}\n")
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        let r = (255.0 * color.x.clamp(0.0, 1.0)) as u32;
        let g = (255.0 * color.y.clamp(0.0, 1.0)) as u32;
        let b = (255.0 * color.z.clamp(0.0, 1.0)) as u32;
        r | g << 8 | b << 16
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        let x = (value & 0xff) as f64 / 255.0;
        let y = ((value >> 8) & 0xff) as f64 / 255.0;
        let z = ((value >> 16) & 0xff) as f64 / 255.0;
        Self{x, y, z}
    }
}

impl From<&u32> for Color {
    fn from(value: &u32) -> Self {
        let x = (value & 0xff) as f64 / 255.0;
        let y = ((value >> 8) & 0xff) as f64 / 255.0;
        let z = ((value >> 16) & 0xff) as f64 / 255.0;
        Self{x, y, z}
    }
}

fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0.0 { linear.sqrt() } else { 0.0 }
}
