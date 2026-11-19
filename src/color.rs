use crate::vec3::Vec3;

use std::io::{self, Write};

pub type Color = Vec3;

pub fn write_color<T: Write>(output: &mut T, color: &Color) -> io::Result<()> {
    let r = (255.999 * color.x.clamp(0.0, 1.0)) as i32;
    let g = (255.999 * color.y.clamp(0.0, 1.0)) as i32;
    let b = (255.999 * color.z.clamp(0.0, 1.0)) as i32;
    write!(output, "{r} {g} {b}\n")
}