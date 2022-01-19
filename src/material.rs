use crate::{
    hittable::HitRecord,
    ray::Ray,
    utils::{random_in_unit_sphere, random_unit_vector},
    vec3::{Color, Vec3, VecOps},
};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let attenuation = self.albedo;
        let scattered = Ray::new(rec.point, scatter_direction);

        Some((attenuation, scattered))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(r_in.direction.normalize(), rec.normal);
        let scattered = Ray::new(rec.point, reflected + self.fuzz * random_in_unit_sphere());

        let attenuation = self.albedo;
        if scattered.direction.dot(rec.normal) > 0f64 {
            return Some((attenuation, scattered));
        }

        None
    }
}

pub struct Dielectric {
    pub ir: f64,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = match rec.front_face {
            true => 1.0 / self.ir,
            false => self.ir,
        };

        let unit_direction = r_in.direction.normalize();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_reflect = refraction_ratio * sin_theta > 1.0;

        let direction = match cannot_reflect {
            true => Vec3::reflect(unit_direction, rec.normal),
            false => refract(unit_direction, rec.normal, refraction_ratio),
        };

        let scattered = Ray::new(rec.point, direction);
        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = ((-uv).dot(n)).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}
