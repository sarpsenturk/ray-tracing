use crate::vec3::Vec3;

use std::io::{self, Write};

pub type Color = Vec3;

pub fn write_color<T: Write>(output: &mut T, color: &Color) -> io::Result<()> {
    let r = (255.999 * color.x) as i32;
    let g = (255.999 * color.y) as i32;
    let b = (255.999 * color.z) as i32;
    writeln!(output, "{r} {g} {b}\n")
}