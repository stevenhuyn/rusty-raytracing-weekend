use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

mod sphere;

pub struct HitRecord {
    point: Point3,
    normal: Vec3,
    t: f64,
}

trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}
