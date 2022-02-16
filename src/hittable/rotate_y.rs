use std::borrow::Borrow;

use crate::{
    aabb::Aabb,
    ray::Ray,
    vec3::{Point3, Vec3},
};

use super::{HitRecord, Hittable};

pub struct RotateY {
    obj: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<Aabb>,
}

impl Hittable for RotateY {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let rotated_ray = Ray::new(origin, direction, ray.time);

        if let Some(hit_record) = self.obj.hit(&rotated_ray, t_min, t_max) {
            let mut p = hit_record.point;
            let mut normal = hit_record.normal;

            p[0] = self.cos_theta * hit_record.point[0] + self.sin_theta * hit_record.point[2];
            p[2] = -self.sin_theta * hit_record.point[0] + self.cos_theta * hit_record.point[2];

            normal[0] =
                self.cos_theta * hit_record.normal[0] + self.sin_theta * hit_record.normal[2];
            normal[2] =
                -self.sin_theta * hit_record.normal[0] + self.cos_theta * hit_record.normal[2];

            return Some(HitRecord::new(
                p,
                hit_record.t,
                &rotated_ray,
                normal,
                hit_record.material,
                hit_record.u,
                hit_record.v,
            ));
        }

        None
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.bbox
    }
}

impl RotateY {
    pub fn new(obj: Box<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = obj.bounding_box(0.0, 1.0).unwrap();

        let mut min = Point3::new(f64::MIN, f64::MIN, f64::MIN);
        let mut max = Point3::new(f64::MAX, f64::MAX, f64::MAX);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;

                    let x = i * bbox.max.x + (1.0 - i) * bbox.min.x;
                    let y = j * bbox.max.y + (1.0 - j) * bbox.min.y;
                    let z = k * bbox.max.z + (1.0 - k) * bbox.min.z;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        let bbox = Some(Aabb::new(min, max));

        RotateY {
            obj,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}
