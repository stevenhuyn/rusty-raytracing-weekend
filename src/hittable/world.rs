use super::{HitRecord, Hittable};
use crate::{aabb::Aabb, ray::Ray};

pub type World = Vec<Box<dyn Hittable>>;

impl Hittable for World {
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

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::aabb::Aabb> {
        if self.is_empty() {
            return None;
        }

        let mut output_box = None;

        for object in self {
            if let Some(temp_box) = object.bounding_box(time0, time1) {
                output_box = match output_box {
                    None => Some(temp_box),
                    Some(output_box) => Some(Aabb::surrounding_box(temp_box, output_box)),
                }
            }
        }

        output_box
    }
}
