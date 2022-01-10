use crate::vec3::{Point3, Vec3};

pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn at(&self, s: f64) -> Point3 {
        self.origin.add(&self.dir.scalar_mul(s))
    }
}
