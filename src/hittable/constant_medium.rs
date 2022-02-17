use crate::material::Isotropic;

pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    material: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Print occasional samples when debugging
        let enable_debug = false;
        let debugging = enable_debug && random_double() < 0.00001;

        let rec1 = self.boundary(ray, f64::NEG_INFINITY, f64::INFINITY)?;
        let rec2 = self.boundary(ray, rec1.t + 0.0001, f64::INFINITY)?;

        if debugging {
            println!("t_min={} t_max={}", rec1.t, rec2.t);
        }

        rec1.t = rec1.t.max(t_min);
        rec2.t = rec2.min(t_max);

        if rec1.t >= rec2.t {
            return None;
        }

        rec1.t = rec1.t.max(0.0);

        let ray_length = ray.direction.length();
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        todo!()
    }
}

impl ConstantMedium {
    pub fn new(b: &dyn Hittable, d: f64, albedo: Color) -> Self {
        ConstantMedium {
            boundary: b,
            material: Isotropic::new(albedo),
            neg_inv_density: d,
        }
    }
}
