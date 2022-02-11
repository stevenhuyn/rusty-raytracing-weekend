use std::cmp::Ordering;

use crate::{aabb::Aabb, ray::Ray, utils::random_double};

use super::{hittable_list::HittableList, Hittable};

pub enum Bvh {
    TwinNode {
        left: Box<dyn Hittable>,
        right: Box<dyn Hittable>,
        bound: Aabb,
    },
    SingNode {
        only: Box<dyn Hittable>,
        bound: Aabb,
    },
}

impl Bvh {
    pub fn new(mut objects: HittableList, time0: f64, time1: f64) -> Self {
        // Why random here I wonder?
        let axis = random_double(0.0, 3.0).floor() as usize;

        let comparator = match axis {
            0 => Self::bool_x_compare,
            1 => Self::bool_y_compare,
            2 => Self::bool_z_compare,
            _ => panic!("Unknown axis supplied"),
        };

        // Base case
        if objects.len() == 1 {
            let only = objects.pop().unwrap();
            let bound = only.bounding_box(time0, time1).unwrap();
            return Bvh::SingNode { only, bound };
        }

        objects.sort_by(|a, b| comparator(a.as_ref(), b.as_ref()));
        let mid_index = objects.len() / 2;
        let left_half = objects.drain(0..mid_index).collect();
        let right_half = objects;

        let left = Box::new(Self::new(left_half, time0, time1));
        let right = Box::new(Self::new(right_half, time0, time1));

        let left_box = left.bounding_box(time0, time1);
        let right_box = right.bounding_box(time0, time1);

        if let (Some(left_box), Some(right_box)) = (left_box, right_box) {
            let bound = Aabb::surrounding_box(left_box, right_box);
            return Bvh::TwinNode { left, right, bound };
        }

        panic!("No bounding box in BvhNode constructor");
    }

    fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis: usize) -> Ordering {
        let box_a = a.bounding_box(0.0, 0.0);
        let box_b = b.bounding_box(0.0, 0.0);

        if let (Some(box_a), Some(box_b)) = (box_a, box_b) {
            return box_a.min[axis].partial_cmp(&box_b.min[axis]).unwrap();
        }

        panic!("No bounding box in BvhNode constructor")
    }

    fn bool_x_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn bool_y_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn bool_z_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for Bvh {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<super::HitRecord> {
        match self {
            Bvh::TwinNode { left, right, bound } => {
                if !bound.hit(ray, t_min, t_max) {
                    return None;
                }

                let left_hit = left.hit(ray, t_min, t_max);
                let new_t_max = left_hit.as_ref().map(|hr| hr.t).unwrap_or(t_max);
                let right_hit = right.hit(ray, t_min, new_t_max);

                right_hit.or(left_hit)
            }
            Bvh::SingNode { only, bound: _ } => only.hit(ray, t_min, t_max),
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        match self {
            Bvh::TwinNode {
                left: _,
                right: _,
                bound,
            } => Some(*bound),
            Bvh::SingNode { only: _, bound } => Some(*bound),
        }
    }
}
