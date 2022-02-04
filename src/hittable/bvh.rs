use std::cmp::Ordering;

use crate::{aabb::Aabb, ray::Ray, utils::random_double};

use super::{world::World, Hittable};

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
    fn new(mut objects: World, time0: f64, time1: f64) -> Self {
        let axis = random_double(0.0, 2.0).floor() as usize;

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

        objects.sort_by(comparator);
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

impl Hittable for Bvh {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<super::HitRecord> {
        match self {
            Bvh::TwinNode { left, right, bound } => {
                let left_hit_record = left.hit(ray, t_min, t_max);
                let right_hit_record = right.hit(ray, t_min, t_max);

                left_hit_record.or(right_hit_record)
            }
            Bvh::SingNode { only, bound } => only.hit(ray, t_min, t_max),
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
