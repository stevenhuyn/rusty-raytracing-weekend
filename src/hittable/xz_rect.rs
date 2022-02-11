use std::sync::Arc;

use crate::{
    aabb::Aabb,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

use super::{HitRecord, Hittable};

/// Infinitely thin rectangle in the XY plane
pub struct XZRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Arc<dyn Material>,
}

impl Hittable for XZRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Get time of hit
        let t = (self.k - ray.origin.y) / ray.direction.y;

        if t < t_min || t >= t_max {
            return None;
        }

        // Get x, z coordinates of hit on XZ plane at z = k
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let point = ray.at(t);

        let outward_normal = Vec3::new(0.0, 1.0, 0.0);

        Some(HitRecord::new(
            point,
            t,
            ray,
            outward_normal,
            Arc::clone(&self.material),
            u,
            v,
        ))
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        XZRect {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}
