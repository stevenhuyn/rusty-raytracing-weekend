use std::sync::Arc;

use super::{HitRecord, Hittable};
use crate::{material::Material, ray::Ray, vec3::Point3};

pub struct Sphere {
    centre: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(centre: Point3, radius: f64, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            centre,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.centre;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - self.centre) / self.radius;

        Some(HitRecord::new(
            point,
            t,
            ray,
            outward_normal,
            Arc::clone(&self.material),
        ))
    }
}
