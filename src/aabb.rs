use crate::{ray::Ray, vec3::Point3};

struct AABB {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl AABB {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0usize..=2 {
            let t0 = ((self.minimum[a] - r.origin[a]) / r.direction[a])
                .min((self.maximum[a] - r.origin[a]) / r.direction[a]);
            let t1 = ((self.minimum[a] - r.origin[a]) / r.direction[a])
                .max((self.maximum[a] - r.origin[a]) / r.direction[a]);

            let t_min = t0.max(t_min);
            let t_max = t1.min(t_max);

            if t_max <= t_min {
                return false;
            }
        }

        return true;
    }
}
