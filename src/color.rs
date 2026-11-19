use crate::vec3::Vec3;

use std::io::{self, Write};

pub type Color = Vec3;

pub fn write_color<T: Write>(output: &mut T, color: &Color) -> io::Result<()> {
    let r = (255.0 * linear_to_gamma(color.x).clamp(0.0, 1.0)) as i32;
    let g = (255.0 * linear_to_gamma(color.y).clamp(0.0, 1.0)) as i32;
    let b = (255.0 * linear_to_gamma(color.z).clamp(0.0, 1.0)) as i32;
    write!(output, "{r} {g} {b}\n")
}

fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0.0 { linear.sqrt() } else { 0.0 }
}
