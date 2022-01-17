use super::{HitRecord, Hittable};
use crate::ray::Ray;

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(hit_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                closest_hit = Some(hit_record);
            }
        }

        closest_hit
    }
}
