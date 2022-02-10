use std::{f64::consts::PI, sync::Arc};

use super::{HitRecord, Hittable};
use crate::{
    aabb::Aabb,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

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

    fn get_sphere_uv(p: Point3) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        (phi / (2f64 * PI), theta / PI)
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
        let (u, v) = Self::get_sphere_uv(outward_normal);

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
        let output_box = Aabb::new(
            self.centre - Vec3::new(self.radius, self.radius, self.radius),
            self.centre + Vec3::new(self.radius, self.radius, self.radius),
        );

        Some(output_box)
    }
}
