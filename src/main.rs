pub mod vec3;
pub mod color;

use crate::color::{Color, write_color};
use std::io::{self, Write};

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;

fn main() {
    let mut output = io::stdout();
    write!(output, "P3\n").unwrap();
    write!(output, "{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").unwrap();
    for j in 0..IMAGE_HEIGHT {
        let percentage = j as f32 / IMAGE_HEIGHT as f32;
        eprintln!("Progress {:.2}", percentage);
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_WIDTH - 1) as f64;
            let b = 0.0;
            write_color(&mut output, &Color { x: r, y: g, z: b }).unwrap();
        }
    }
    eprintln!("Done");
}
