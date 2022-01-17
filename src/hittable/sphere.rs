use super::{HitRecord, Hittable};
use crate::{material::Material, ray::Ray, vec3::Point3};

pub struct Sphere<'a> {
    centre: Point3,
    radius: f64,
    material: &'a dyn Material,
}

impl Sphere<'_> {
    pub fn new(centre: Point3, radius: f64, material: &dyn Material) -> Sphere {
        Sphere {
            centre,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere<'_> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.centre;
        let a = ray.direction.dot(ray.direction);
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        let sqrt_disc = discriminant.sqrt();

        let root = (-half_b - discriminant.sqrt()) / a;
        if sqrt_disc.is_nan() || (root < t_min || t_max < root) {
            return None;
        }

        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - self.centre) / self.radius;

        Some(HitRecord::new(
            point,
            t,
            ray,
            outward_normal,
            &self.material,
        ))
    }
}
