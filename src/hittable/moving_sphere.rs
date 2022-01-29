use std::sync::Arc;

use crate::{
    aabb::AABB,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

use super::{HitRecord, Hittable};

pub struct MovingSphere {
    centre0: Point3,
    centre1: Point3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        centre0: Point3,
        centre1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> MovingSphere {
        MovingSphere {
            centre0,
            centre1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn centre(&self, time: f64) -> Point3 {
        self.centre0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.centre1 - self.centre0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.centre(ray.time);
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
        let outward_normal = (point - self.centre(ray.time)) / self.radius;

        Some(HitRecord::new(
            point,
            t,
            ray,
            outward_normal,
            Arc::clone(&self.material),
        ))
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::aabb::AABB> {
        let box0 = AABB::new(
            self.centre(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.centre(time0) + Vec3::new(self.radius, self.radius, self.radius),
        );

        let box1 = AABB::new(
            self.centre(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.centre(time1) + Vec3::new(self.radius, self.radius, self.radius),
        );

        Some(AABB::surrounding_box(box0, box1))
    }
}
