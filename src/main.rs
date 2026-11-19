pub mod vec3;
pub mod color;
pub mod ray;
pub mod object;
pub mod camera;

use crate::vec3::{Vec3};
use crate::object::{Sphere, Scene};
use crate::camera::Camera;
use std::io::{self};

// Width of the final image output
const IMAGE_WIDTH: i32 = 960;
// Height of the final image output
const IMAGE_HEIGHT: i32 = 540;

fn main() -> io::Result<()> {
    // Scene/world
    let scene : Scene = vec![
        Box::new(Sphere::new(Vec3{x: 0.0, y: 0.0, z: -1.0}, 0.5)),
        Box::new(Sphere::new(Vec3{x: 0.0, y: -100.5, z: -1.0}, 100.0))
    ];

    // Render
    let camera = Camera::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut output = io::stdout();
    camera.render(&mut output, &scene)
}
