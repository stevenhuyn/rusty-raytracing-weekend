use crate::{
    hittable::HitRecord,
    ray::Ray,
    utils::random_unit_vector,
    vec3::{Color, Vec3, VecOps},
};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let attenuation = self.albedo.clone(); // Cloning bad?
        let scattered = Ray {
            origin: rec.point,
            direction: scatter_direction,
        };

        Some((attenuation, scattered))
    }
}

pub struct Metal {
    albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(r_in.direction.normalize(), rec.normal);
        let scattered = Ray {
            origin: rec.point,
            direction: reflected,
        };

        let attenuation = self.albedo.clone();
        if scattered.direction.dot(rec.normal) > 0f64 {
            return Some((attenuation, scattered));
        }

        None
    }
}
