use crate::object::{Hittable, Scene};
use crate::vec3::{cross, Vec3, random_in_unit_disk};
use crate::color::{write_color, Color};
use crate::ray::Ray;
use std::io::{self, BufWriter, Write};
use std::num::NonZeroUsize;
use std::time::Instant;
use rand::{self};

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    image_width: i32,
    image_height: i32,
    samples: i32,
    pixel_samples_scale: f64,
    max_depth: i32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        image_width: i32, image_height: i32,
        vfov: f64,
        samples: i32, max_depth: i32,
        position: Vec3, direction: Vec3, up: Vec3,
        defocus_angle: f64,
        focus_distance: f64) -> Camera {
        let center = position;
        let aspect_ratio = image_width as f64 / image_height as f64;
        let pixel_samples_scale = 1.0 / samples as f64;

        // Determine viewport dimensions.
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height * aspect_ratio;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (position - direction).normalized();
        let u = cross(&up, &w);
        let v = cross(&w, &u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = u * viewport_width;
        let viewport_v = v * -viewport_height;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center -
                (w * focus_distance) -
                viewport_u / 2.0 -
                viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_distance * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera{
            image_width,
            image_height,
            samples,
            pixel_samples_scale,
            max_depth,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render<T: Write>(&self, output: &mut T, scene: &Scene) -> io::Result<()> {
        // Allocate framebuffer
        let mut framebuffer = vec![u32::default(); (self.image_width * self.image_height) as usize];

        // Determine number of hardware threads
        let thread_count = std::thread::available_parallelism().map(NonZeroUsize::get).unwrap_or(1);
        eprintln!("Using {thread_count} threads...");

        let rows_per_thread = (self.image_height as usize + thread_count - 1) / thread_count;
        let start = Instant::now();
        std::thread::scope(|s| {
           for (i, chunk) in framebuffer
               .chunks_mut(rows_per_thread * self.image_width as usize)
               .enumerate() {
               let camera = self.clone();
               let start = i * rows_per_thread;
               let end = start + chunk.len() / self.image_width as usize;
               s.spawn(move || render(camera, start, end, scene, chunk));
           }
        });
        let time = start.elapsed();
        eprint!("\nDone. Finished in {} seconds\n", time.as_secs());

        let mut writer = BufWriter::new(output);
        write!(writer, "P3\n")?;
        write!(writer, "{} {}\n255\n", self.image_width, self.image_height)?;
        framebuffer
            .iter()
            .map(Color::from)
            .try_for_each(|color| write_color(&mut writer, &color))
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.

        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc +
            (self.pixel_delta_u * (i as f64 + offset.x)) +
            (self.pixel_delta_v * (j as f64 + offset.y));

        let origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() };
        let direction = pixel_sample - origin;
        Ray::new(origin, direction)
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3{
            x: rand::random::<f64>() - 0.5,
            y: rand::random::<f64>()  - 0.5,
            z: 0.0,
        }
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        // Returns a random point in the camera defocus disk.
        let p = random_in_unit_disk();
        self.center + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }
}

fn render(camera: Camera, start: usize, end: usize, scene: &Scene, framebuffer: &mut [u32]) {
    let width = camera.image_width as usize;
    for j in start..end {
        for i in 0..width {
            let mut color = Color::from(0.0);
            for _ in 0..camera.samples {
                let ray = camera.get_ray(i as i32, j as i32);
                color += ray_color(&ray, camera.max_depth, scene)
            }
            let idx = (j - start) * camera.image_width as usize + i; // Index relative to this chunk
            framebuffer[idx] = u32::from(color * camera.pixel_samples_scale);
        }
    }
}

fn ray_color(ray: &Ray, depth: i32, scene: &Scene) -> Color {
    if depth <= 0 { return Color::from(0.0); };

    if let Some(hit) = scene.hit(ray, 0.001, f64::INFINITY) {
        return if let Some(surface) = hit.surface() {
            *surface.color() * ray_color(surface.scattered(), depth - 1, scene)
        } else {
            Color::from(0.0)
        }
    }
    let unit = ray.direction().normalized();
    let a = 0.5 * (unit.y + 1.0);
    Color::from(1.0) * (1.0 - a) + Color{x: 0.5, y: 0.7, z: 1.0} * a
}
