use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::{random_unit_vector, reflect, Vec3};

pub struct SurfaceOutput {
    color: Color,
    scattered: Ray,
}

impl SurfaceOutput {
    pub fn new(color: Color, scattered: Ray) -> Self { SurfaceOutput{color, scattered} }
    pub fn color(&self) -> &Color { &self.color }
    pub fn scattered(&self) -> &Ray { &self.scattered }
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, point: &Vec3, normal: &Vec3) -> Option<SurfaceOutput>;
}

pub struct LambertianMaterial {
    color: Color,
}

impl LambertianMaterial {
    pub fn new(color: Color) -> Self { Self{color} }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _ray_in: &Ray, point: &Vec3, normal: &Vec3) -> Option<SurfaceOutput> {
        let direction = *normal + random_unit_vector();
        let scatter_direction = if direction.near_zero() { *normal } else { direction };
        Some(SurfaceOutput::new(
            self.color,
            Ray::new(*point, scatter_direction)
        ))
    }
}

pub struct MetallicMaterial {
    color: Color,
}

impl MetallicMaterial {
    pub fn new(color: Color) -> Self { Self{color} }
}

impl Material for MetallicMaterial {
    fn scatter(&self, ray_in: &Ray, point: &Vec3, normal: &Vec3) -> Option<SurfaceOutput> {
        let reflected = reflect(ray_in.direction(), normal);
        Some(SurfaceOutput::new(
            self.color,
            Ray::new(*point, reflected),
        ))
    }
}
