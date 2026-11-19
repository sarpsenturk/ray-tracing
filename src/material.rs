use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::{random_unit_vector, reflect, Vec3, dot, refract};

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
    fn scatter(&self, ray_in: &Ray, point: &Vec3, normal: &Vec3, front_face: bool) -> Option<SurfaceOutput>;
}

pub struct LambertianMaterial {
    color: Color,
}

impl LambertianMaterial {
    pub fn new(color: Color) -> Self { Self{color} }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _ray_in: &Ray, point: &Vec3, normal: &Vec3, _front_face: bool) -> Option<SurfaceOutput> {
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
    fuzz: f64,
}

impl MetallicMaterial {
    pub fn new(color: Color, fuzz: f64) -> Self { Self{color, fuzz} }
}

impl Material for MetallicMaterial {
    fn scatter(&self, ray_in: &Ray, point: &Vec3, normal: &Vec3, _front_face: bool) -> Option<SurfaceOutput> {
        let reflected =
            reflect(ray_in.direction(), normal).normalized() +
                (random_unit_vector() * self.fuzz);
        if dot(&reflected, normal) > 0.0 {
            Some(SurfaceOutput::new(
                self.color,
                Ray::new(*point, reflected),
            ))
        } else {
            None
        }
    }
}

pub struct DielectricMaterial {
    refraction_index: f64,
}

impl DielectricMaterial {
    pub fn new(refraction_index: f64) -> Self { Self{refraction_index} }
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray_in: &Ray, point: &Vec3, normal: &Vec3, front_face: bool) -> Option<SurfaceOutput> {
        let ri = if front_face { 1.0 / self.refraction_index } else { self.refraction_index };
        let direction = ray_in.direction().normalized();
        let refracted = refract(&direction, normal, ri);
        Some(SurfaceOutput::new(
            Color::from(1.0),
            Ray::new(*point, refracted),
        ))
    }
}
