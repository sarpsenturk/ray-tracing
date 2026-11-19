use crate::object::{Hittable, Scene};
use crate::vec3::Vec3;
use crate::color::{write_color, Color};
use crate::ray::Ray;
use std::io::{self, Write};

pub struct Camera {
    image_width: i32,
    image_height: i32,
    aspect_ratio: f64,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(image_width: i32, image_height: i32) -> Camera {
        let center = Vec3::from(0.0);
        let aspect_ratio = image_width as f64 / image_height as f64;

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3{x: viewport_width, y: 0.0, z: 0.0};
        let viewport_v = Vec3{x: 0.0, y: -viewport_height, z: 0.0};

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center -
            Vec3{x: 0.0, y: 0.0, z: focal_length} -
            viewport_u / 2.0 -
            viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Camera{
            image_width,
            image_height,
            aspect_ratio,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render<T: Write>(&self, output: &mut T, scene: &Scene) -> io::Result<()> {
        write!(output, "P3\n")?;
        write!(output, "{} {}\n255\n", self.image_width, self.image_height)?;
        for j in 0..self.image_height {
            eprintln!("Progress {:.2}", j as f32 / self.image_height as f32);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
                let ray_direction = pixel_center - self.center;
                let color = ray_color(&Ray::new(self.center, ray_direction), &scene);
                write_color(output, &color)?
            }
        }
        Ok(())
    }
}

fn ray_color(ray: &Ray, scene: &Scene) -> Color {
    if let Some(hit) = scene.hit(ray, 0.0, f64::INFINITY) {
        return *hit.normal() + Color::from(1.0) * 0.5;
    }
    let unit = ray.direction().normalized();
    let a = 0.5 * (unit.y + 1.0);
    Color::from(1.0) * (1.0 - a) + Color{x: 0.5, y: 0.7, z: 1.0} * a
}
