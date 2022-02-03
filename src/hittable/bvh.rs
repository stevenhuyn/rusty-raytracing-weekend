use std::cmp::Ordering;

use crate::{aabb::Aabb, ray::Ray, utils::random_double};

use super::{world::World, Hittable};

pub struct BvhNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bound: Aabb,
}

impl BvhNode {
    fn new(mut objects: World, time0: f64, time1: f64) -> Self {
        let axis = random_double(0.0, 2.0).floor() as usize;

        let comparator = match axis {
            0 => Self::bool_x_compare,
            1 => Self::bool_y_compare,
            2 => Self::bool_z_compare,
            _ => panic!("Unknown axis supplied"),
        };

        let left: Box<dyn Hittable>;
        let right: Box<dyn Hittable>;
        if objects.len() == 1 {
            left = objects.pop().unwrap();
            right = objects.pop().unwrap();
        } else {
            objects.sort_by(comparator);
            let mid_index = objects.len() / 2;
            let left_half = objects.drain(0..mid_index).collect();
            let right_half = objects;

            left = Box::new(Self::new(left_half, time0, time1));
            right = Box::new(Self::new(right_half, time0, time1));
        }

        let left_box = left.bounding_box(time0, time1);
        let right_box = right.bounding_box(time0, time1);

        if let (Some(left_box), Some(right_box)) = (left_box, right_box) {
            let bound = Aabb::surrounding_box(left_box, right_box);
            return BvhNode { left, right, bound };
        }

        panic!("No bounding box in BvhNode constructor");
    }

    // is &Box a code smell?
    fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis: usize) -> Ordering {
        let box_a = a.bounding_box(0.0, 0.0);
        let box_b = b.bounding_box(0.0, 0.0);

        if let (Some(box_a), Some(box_b)) = (box_a, box_b) {
            return box_a.min[axis].partial_cmp(&box_b.min[axis]).unwrap();
        }

        panic!("No bounding box in BvhNode constructor")
    }

    fn bool_x_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn bool_y_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn bool_z_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
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

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bound)
    }
}
