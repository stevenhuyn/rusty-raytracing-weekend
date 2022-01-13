use crate::ray::Ray;

use super::{HitRecord, Hittable};

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_record: HitRecord = HitRecord::new(); // TODO: Option?
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self {
            // Need to add logic to check which was closest
            if object.hit(ray, t_min, t_max, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *hit_record = temp_record;
            }
        }

        hit_anything
    }
}
