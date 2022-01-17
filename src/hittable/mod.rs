use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub mod hittable_list;
pub mod sphere;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new(point: Point3, t: f64, ray: &Ray, outward_normal: Vec3) -> HitRecord {
        let front_face = ray.direction.dot(outward_normal) > 0.0;
        let normal = match front_face {
            true => -outward_normal,
            false => outward_normal,
        };

        HitRecord { point, t, normal }
    }
}
