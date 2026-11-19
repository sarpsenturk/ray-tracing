use crate::object::{Hittable, Scene};
use crate::vec3::{cross, Vec3, random_in_unit_disk};
use crate::color::{write_color, Color};
use crate::ray::Ray;
use std::io::{self, BufWriter, Write};
use std::num::NonZeroUsize;
use std::time::Instant;
use std::sync::atomic::{AtomicUsize, Ordering};
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

const TILE_SIZE: usize = 32;

#[derive(Debug, Copy, Clone)]
struct Tile {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

fn make_tiles(image_width: usize, image_height: usize, tile_size: usize) -> Vec<Tile> {
    let mut tiles = Vec::new();
    for y in (0..image_height).step_by(tile_size) {
        for x in (0..image_width).step_by(tile_size) {
            tiles.push(Tile {
                x,
                y,
                // Edge tiles may be smaller than tile_size
                width: tile_size.min(image_width - x),
                height: tile_size.min(image_height - y),
            });
        }
    }
    tiles
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
        let width = self.image_width as usize;
        let height = self.image_height as usize;
        let mut framebuffer = vec![u32::default(); width * height];

        let tiles = make_tiles(width, height, TILE_SIZE);
        let thread_count = std::thread::available_parallelism()
            .map(NonZeroUsize::get)
            .unwrap_or(1)
            .min(tiles.len());
        eprintln!("Using {thread_count} threads, {} tiles of {TILE_SIZE}x{TILE_SIZE}...", tiles.len());

        let start = Instant::now();
        let next_tile = AtomicUsize::new(0);

        // Each worker returns the tiles it rendered, along with their pixels.
        let results: Vec<Vec<(Tile, Vec<u32>)>> = std::thread::scope(|s| {
            let tiles = &tiles;
            let next_tile = &next_tile;
            let mut handles = Vec::with_capacity(thread_count);

            for _ in 0..thread_count {
                let camera = *self; // Camera is Copy
                handles.push(s.spawn(move || {
                    let mut done = Vec::new();
                    loop {
                        let idx = next_tile.fetch_add(1, Ordering::Relaxed);
                        let Some(&tile) = tiles.get(idx) else { break };
                        done.push((tile, render_tile(&camera, &tile, scene)));
                    }
                    done
                }));
            }

            handles.into_iter().map(|h| h.join().unwrap()).collect()
        });

        // Copy each tile into its place in the framebuffer.
        for (tile, pixels) in results.into_iter().flatten() {
            for row in 0..tile.height {
                let dst = (tile.y + row) * width + tile.x;
                let src = row * tile.width;
                framebuffer[dst..dst + tile.width]
                    .copy_from_slice(&pixels[src..src + tile.width]);
            }
        }

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

fn render_tile(camera: &Camera, tile: &Tile, scene: &Scene) -> Vec<u32> {
    let mut pixels = Vec::with_capacity(tile.width * tile.height);
    for j in tile.y..tile.y + tile.height {
        for i in tile.x..tile.x + tile.width {
            let mut color = Color::from(0.0);
            for _ in 0..camera.samples {
                let ray = camera.get_ray(i as i32, j as i32);
                color += ray_color(&ray, camera.max_depth, scene);
            }
            pixels.push(u32::from(color * camera.pixel_samples_scale));
        }
    }
    pixels
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
