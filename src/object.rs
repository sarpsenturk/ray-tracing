use crate::material::{Material, SurfaceOutput};
use crate::vec3::{dot, Vec3};
use crate::ray::Ray;

pub struct Hit {
    point: Vec3,
    normal: Vec3,
    surface: Option<SurfaceOutput>,
    time: f64,
}

impl Hit {
    pub fn new(point: Vec3, normal: Vec3, surface: Option<SurfaceOutput>, time: f64) -> Hit {
        Hit{point, normal, surface, time}
    }

    pub fn point(&self) -> &Vec3 { &self.point }
    pub fn normal(&self) -> &Vec3 { &self.normal }
    pub fn time(&self) -> f64 { self.time }
    pub fn surface(&self) -> &Option<SurfaceOutput> { &self.surface }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<Hit>;
}

pub type Scene = Vec<Box<dyn Hittable>>;

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<Hit> {
        let mut closest = ray_tmax;
        let mut closest_hit: Option<Hit> = None;
        for hittable in self {
            if let Some(hit) = hittable.hit(ray, ray_tmin, closest) {
                closest = hit.time();
                closest_hit = Some(hit);
            }
        }
        closest_hit
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Self {
        Self{center, radius, material}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<Hit> {
        let oc = self.center - *ray.origin();
        let a = ray.direction().sqr_magnitude();
        let h = dot(ray.direction(), &oc);
        let c = oc.sqr_magnitude() - self.radius*self.radius;

        let discriminant = h*h - a*c;
        if discriminant < 0.0 { return None; }

        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || root >= ray_tmax {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || root >= ray_tmax {
                return None;
            }
        }
        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;
        let front_face = dot(ray.direction(), &normal) < 0.0;
        let normal = if front_face { normal } else { -normal };
        let surface = self.material.scatter(ray, &point, &normal);
        Some(Hit::new(point, normal, surface, root))
    }
}
