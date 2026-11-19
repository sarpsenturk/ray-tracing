pub mod vec3;
pub mod color;
pub mod ray;

use crate::vec3::{dot, Vec3};
use crate::color::{Color, write_color};
use crate::ray::{Ray};
use std::io::{self, Write};

// Width of the final image output
const IMAGE_WIDTH: i32 = 960;
// Height of the final image output
const IMAGE_HEIGHT: i32 = 540;

// Ratio of IMAGE_WIDTH / IMAGE_HEIGHT
const ASPECT_RATIO: f64 = IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;

// Viewport height
const VIEWPORT_HEIGHT: f64 = 2.0;
// Viewport width, defined by the aspect ratio
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
const FOCAL_LENGTH: f64 = 1.0;

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = center - ray.origin();
    let a = dot(ray.direction(), ray.direction());
    let b = -2.0 * dot(ray.direction(), &oc);
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 { -1.0 } else { (-b - discriminant.sqrt()) / (2.0 * a) }
}

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(&Vec3{x: 0.0, y: 0.0, z: -1.0}, 0.5, ray);
    if  t > 0.0 {
        let normal = (ray.at(t) - Vec3{x: 0.0, y: 0.0, z: -1.0}).normalized();
        return Color{x: normal.x + 1.0, y: normal.y + 1.0, z: normal.z} * 0.5;
    }
    let unit = ray.direction().normalized();
    let a = 0.5 * (unit.y + 1.0);
    Color::from(1.0) * (1.0 - a) + Color{x: 0.5, y: 0.7, z: 1.0} * a
}

fn main() {
    // Camera position
    let camera_position = Vec3{..Default::default()};

    // Viewports uv vectors
    let viewport_u = Vec3{x: VIEWPORT_WIDTH, y: 0.0, z: 0.0};
    let viewport_v = Vec3{x: 0.0, y: -VIEWPORT_HEIGHT, z: 0.0};

    // Calculate horizontal and vertical delta vector from pixel to pixel
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left =
        camera_position -
            Vec3{x: 0.0, y: 0.0, z: FOCAL_LENGTH} -
            viewport_u / 2.0 -
            viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + Vec3::from(0.5)  * (pixel_delta_u + pixel_delta_v);

    // Render
    let mut output = io::stdout();
    write!(output, "P3\n").unwrap();
    write!(output, "{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").unwrap();
    for j in 0..IMAGE_HEIGHT {
        let percentage = j as f32 / IMAGE_HEIGHT as f32;
        eprintln!("Progress {:.2}", percentage);
        for i in 0..IMAGE_WIDTH {
            let pixel_center = pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_position;
            let color = ray_color(&Ray::new(camera_position, ray_direction));
            write_color(&mut output, &color).unwrap()
        }
    }
    eprintln!("Done");
}
