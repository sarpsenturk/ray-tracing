pub mod vec3;
pub mod color;
pub mod ray;
pub mod object;
pub mod camera;
pub mod material;

use crate::vec3::{Vec3};
use crate::object::{Sphere, Scene};
use crate::camera::Camera;
use std::io::{self};
use crate::color::Color;
use crate::material::{LambertianMaterial, MetallicMaterial};

// Width of the final image output
const IMAGE_WIDTH: i32 = 960;
// Height of the final image output
const IMAGE_HEIGHT: i32 = 540;

fn main() -> io::Result<()> {

    // Scene/world
    let scene : Scene = vec![
        Box::new(Sphere::new(
            Vec3{x: 0.0, y: -100.5, z: -1.0},
            100.0,
            Box::new(LambertianMaterial::new(Color{x: 0.8, y: 0.8, z: 0.0}))
        )),
        Box::new(Sphere::new(
            Vec3{x: 0.0, y: 0.0, z: -1.2},
            0.5,
            Box::new(LambertianMaterial::new(Color{x: 0.1, y: 0.2, z: 0.5}))
        )),
        Box::new(Sphere::new(
            Vec3{x: -1.0, y: 0.0, z: -1.0},
            0.5,
            Box::new(MetallicMaterial::new(Color{x: 0.8, y: 0.8, z: 0.8}, 0.3))
        )),
        Box::new(Sphere::new(
            Vec3{x: 1.0, y: 0.0, z: -1.0},
            0.5,
            Box::new(MetallicMaterial::new(Color{x: 0.8, y: 0.6, z: 0.2}, 1.0))
        )),
    ];

    // Render
    let camera = Camera::new(IMAGE_WIDTH, IMAGE_HEIGHT, 10, 50);
    let mut output = io::stdout();
    camera.render(&mut output, &scene)
}
