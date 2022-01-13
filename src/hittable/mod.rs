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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

impl HitRecord {
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        let front_face = ray.direction.dot(outward_normal) > 0.0;
        self.normal = match front_face {
            true => -outward_normal,
            false => outward_normal,
        }
    }

    pub fn new() -> HitRecord {
        HitRecord {
            point: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
        }
    }
}
