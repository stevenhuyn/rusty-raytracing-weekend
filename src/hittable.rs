use std::sync::Arc;

use crate::{
    aabb::Aabb,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub mod box_rect;
pub mod bvh;
pub mod constant_medium;
pub mod hittable_list;
pub mod moving_sphere;
pub mod rotate_y;
pub mod sphere;
pub mod translate;
pub mod xy_rect;
pub mod xz_rect;
pub mod yz_rect;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material: Arc<dyn Material>,
    pub front_face: bool,
    pub u: f64,
    pub v: f64,
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}

impl HitRecord {
    pub fn new(
        point: Point3,
        t: f64,
        ray: &Ray, // Incoming ray
        outward_normal: Vec3,
        material: Arc<dyn Material>,
        u: f64,
        v: f64,
    ) -> HitRecord {
        let front_face = ray.direction.dot(outward_normal) <= 0.0;
        let normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };

        HitRecord {
            point,
            t,
            normal,
            material,
            front_face,
            u,
            v,
        }
    }
}
