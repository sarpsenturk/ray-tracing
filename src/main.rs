pub mod vec3;
pub mod color;
pub mod ray;
pub mod object;
pub mod camera;
pub mod material;

use crate::vec3::{Vec3};
use crate::object::{Sphere, Scene};
use crate::camera::Camera;
use crate::color::Color;
use crate::material::{Material, DielectricMaterial, LambertianMaterial, MetallicMaterial};
use std::io::{self};

// Width of the final image output
const IMAGE_WIDTH: i32 = 1200;
// Height of the final image output
const IMAGE_HEIGHT: i32 = 675;

fn main() -> io::Result<()> {

    // Scene/world
    let mut scene = Scene::new();
    // Ground
    scene.push(Box::new(Sphere::new(
        Vec3{x: 0.0, y: -1000.0, z: 0.0},
        1000.0,
        Box::new(LambertianMaterial::new(
            Color{x: 0.5, y: 0.5, z: 0.5}
        ))
    )));

    // Add random spheres to scene
    for a in -11..11 {
        for b in -11..11 {
            let random_material: f64 = rand::random();
            let center = Vec3{
                x: a as f64 + 0.9 * rand::random::<f64>(),
                y: 0.2,
                z: b as f64 + 0.9 * rand::random::<f64>(),
            };

            if (center - Vec3{x: 4.0, y: 0.2, z: 0.0}).magnitude() > 0.9 {
                let material: Box<dyn Material> = match random_material {
                    ..0.8 => {
                        let albedo = rand::random::<Color>() * rand::random::<Color>();
                        Box::new(LambertianMaterial::new(albedo))
                    },
                    ..0.95 => {
                        let albedo: Color = rand::random();
                        let fuzz: f64 = rand::random_range(0.5..1.0);
                        Box::new(MetallicMaterial::new(albedo, fuzz))
                    }
                    _ => {
                        Box::new(DielectricMaterial::new(1.5))
                    }
                };
                scene.push(Box::new(Sphere::new(
                    center,
                    0.2,
                    material
                )));
            }
        }
    }

    // Places spheres
    scene.push(Box::new(Sphere::new(
        Vec3{x: 0.0, y: 1.0, z: 0.0},
        1.0,
        Box::new(DielectricMaterial::new(1.5))
    )));
    scene.push(Box::new(Sphere::new(
        Vec3{x: -4.0, y: 1.0, z: 0.0},
        1.0,
        Box::new(LambertianMaterial::new(
            Color{x: 0.4, y: 0.2, z: 0.1}
        ))
    )));
    scene.push(Box::new(Sphere::new(
        Vec3{x: 4.0, y: 1.0, z: 0.0},
        1.0,
        Box::new(MetallicMaterial::new(
            Color{x: 0.7, y: 0.6, z: 0.5},
            0.0
        ))
    )));

    // Render
    let fov = 20.0;
    let samples = 20;
    let max_depth = 25;
    let camera_position = Vec3{x: 13.0, y: 2.0, z: 3.0};
    let camera_direction = Vec3{x: 0.0, y: 0.0, z: 0.0};
    let camera_up = Vec3{x: 0.0, y: 1.0, z: 0.0};
    let defocus_angle = 0.6;
    let defocus_distance = 10.0;
    let camera = Camera::new(
        IMAGE_WIDTH, IMAGE_HEIGHT,
        fov,
        samples, max_depth,
        camera_position, camera_direction, camera_up,
        defocus_angle, defocus_distance,
    );
    let mut output = io::stdout();
    camera.render(&mut output, &scene)
}
