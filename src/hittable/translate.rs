use crate::{aabb::Aabb, ray::Ray, vec3::Vec3};

use super::{HitRecord, Hittable};

pub struct Translate {
    obj: Box<dyn Hittable>,
    offset: Vec3,
}

impl Hittable for Translate {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        if let Some(hit_record) = self.obj.hit(&moved_ray, t_min, t_max) {
            return Some(HitRecord::new(
                hit_record.point + self.offset,
                hit_record.t,
                &moved_ray,
                hit_record.normal,
                hit_record.material,
                hit_record.u,
                hit_record.v,
            ));
        }

        None
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if let Some(bbox) = self.obj.bounding_box(time0, time1) {
            return Some(Aabb::new(bbox.min + self.offset, bbox.max + self.offset));
        }

        None
    }
}
