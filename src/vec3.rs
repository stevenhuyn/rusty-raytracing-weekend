use glam::DVec3;

pub type Point3 = DVec3;
pub type Color = DVec3;
pub type Vec3 = DVec3;

pub trait VecOps {
    fn write_color(&self, samples_per_pixel: i64);
}

impl VecOps for Vec3 {
    fn write_color(&self, samples_per_pixel: i64) {
        let mut r = self.x;
        let mut g = self.y;
        let mut b = self.z;

        let scale = 1f64 / samples_per_pixel as f64;
        r *= scale;
        g *= scale;
        b *= scale;

        println!(
            "{} {} {}",
            255.999 * r.clamp(0.0, 0.999),
            255.999 * g.clamp(0.0, 0.999),
            255.999 * b.clamp(0.0, 0.999)
        );
    }
}
