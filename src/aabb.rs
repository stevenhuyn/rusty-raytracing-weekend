use crate::{ray::Ray, vec3::Point3};

struct AABB {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl AABB {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0usize..=2 {
            let inv_d = 1f64 / r.direction[a];
            let mut t0 = (self.minimum[a] - r.origin[a]) * inv_d;
            let mut t1 = (self.maximum[a] - r.origin[a]) * inv_d;

            if inv_d < 0f64 {
                (t1, t0) = (t0, t1);
            }

            let t_min = t0.max(t_min);
            let t_max = t1.min(t_max);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}
