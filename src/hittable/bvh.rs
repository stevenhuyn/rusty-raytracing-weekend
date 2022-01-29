use crate::{aabb::Aabb, ray::Ray};

use super::Hittable;

pub struct BvhNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bound: Aabb,
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<super::HitRecord> {
        if !self.bound.hit(ray, t_min, t_max) {
            return None;
        }

        let left_hit_record = self.left.hit(ray, t_min, t_max);
        let right_hit_record = self.right.hit(ray, t_min, t_max);

        left_hit_record.or(right_hit_record)
    }
}
