use crate::{ray::Ray, vec3::Point3};

use super::{HitRecord, Hittable};

struct Sphere {
    centre: Point3,
    radius: f64,
}

impl Sphere {
    fn new(centre: Point3, radius: f64) -> Sphere {
        Sphere { centre, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.centre;
        let a = ray.direction.dot(ray.direction);
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        let sqrt_disc = discriminant.sqrt();

        let root = (-half_b - discriminant.sqrt()) / a;
        if !sqrt_disc.is_nan() && (root < t_min || t_max < root) {
            return false;
        }

        rec.t = root;
        rec.point = ray.at(rec.t);
        rec.normal = (rec.point - self.centre) / self.radius;

        true
    }
}
