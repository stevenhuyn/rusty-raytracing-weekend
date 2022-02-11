use std::sync::Arc;

use crate::{
    aabb::Aabb,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

use super::{HitRecord, Hittable};

/// Infinitely thin rectangle in the XY plane
pub struct YZRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Arc<dyn Material>,
}

impl Hittable for YZRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Get time of hit
        let t = (self.k - ray.origin.x) / ray.direction.x;

        if t < t_min || t >= t_max {
            return None;
        }

        // Get y, z coordinates of hit on YZ plane at z = k
        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let point = ray.at(t);

        let outward_normal = Vec3::new(1.0, 0.0, 0.0);

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
            Point3::new(self.k - 0.0001, self.y0, self.z0),
            Point3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        YZRect {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        }
    }
}
