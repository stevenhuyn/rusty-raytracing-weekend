use std::sync::Arc;

use crate::{aabb::Aabb, material::Material, vec3::Point3};

use super::{
    hittable_list::HittableList, xy_rect::XYRect, xz_rect::XZRect, yz_rect::YZRect, Hittable,
};

pub struct BoxRect {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}

impl Hittable for BoxRect {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<super::HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<crate::aabb::Aabb> {
        Some(Aabb::new(self.box_min, self.box_max))
    }
}

impl BoxRect {
    pub fn new(box_min: Point3, box_max: Point3, mat: Arc<dyn Material>) -> Self {
        let sides: HittableList = vec![
            (Box::new(XYRect::new(
                box_min.x,
                box_max.x,
                box_min.y,
                box_max.y,
                box_min.z,
                Arc::clone(&mat),
            ))),
            Box::new(XYRect::new(
                box_min.x,
                box_max.x,
                box_min.y,
                box_max.y,
                box_max.z,
                Arc::clone(&mat),
            )),
            Box::new(XZRect::new(
                box_min.x,
                box_max.x,
                box_min.z,
                box_max.z,
                box_min.y,
                Arc::clone(&mat),
            )),
            Box::new(XZRect::new(
                box_min.x,
                box_max.x,
                box_min.z,
                box_max.z,
                box_max.y,
                Arc::clone(&mat),
            )),
            Box::new(YZRect::new(
                box_min.y,
                box_max.y,
                box_min.z,
                box_max.z,
                box_min.x,
                Arc::clone(&mat),
            )),
            Box::new(YZRect::new(
                box_min.y,
                box_max.y,
                box_min.z,
                box_max.z,
                box_max.x,
                Arc::clone(&mat),
            )),
        ];

        BoxRect {
            box_min,
            box_max,
            sides,
        }
    }
}
