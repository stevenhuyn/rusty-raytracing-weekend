use std::sync::Arc;

use crate::{
    aabb::Aabb,
    material::{Isotropic, Material},
    ray::Ray,
    utils::random_double,
    vec3::{Color, Vec3},
};

use super::{HitRecord, Hittable};

pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    density: f64,
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Print occasional samples when debugging
        let enable_debug = false;
        let debugging = enable_debug && random_double(0.0, 1.0) < 0.00001;

        let mut rec1 = self.boundary.hit(ray, f64::NEG_INFINITY, f64::INFINITY)?;
        let mut rec2 = self.boundary.hit(ray, rec1.t + 0.0001, f64::INFINITY)?;

        if debugging {
            println!("t_min={} t_max={}", rec1.t, rec2.t);
        }

        rec1.t = rec1.t.max(t_min);
        rec2.t = rec2.t.min(t_max);

        if rec1.t >= rec2.t {
            return None;
        }

        rec1.t = rec1.t.max(0.0);

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = -(1.0 / self.density) * random_double(0.0, 1.0).ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + (hit_distance / ray_length);
        Some(HitRecord::new(
            ray.at(t),
            t,
            ray,
            Vec3::new(1.0, 0.0, 0.0), // Arbitarary
            Arc::clone(&self.phase_function),
            0.0,
            0.0,
        ))
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}

impl ConstantMedium {
    pub fn new(b: Box<dyn Hittable>, d: f64, albedo: Color) -> Self {
        ConstantMedium {
            boundary: b,
            phase_function: Arc::new(Isotropic::new(albedo)),
            density: d,
        }
    }
}
