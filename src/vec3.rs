use glam::DVec3;

pub type Point3 = DVec3;
pub type Color = DVec3;
pub type Vec3 = DVec3;

pub trait VecOps {
    fn write_color(&self);
}

impl VecOps for Vec3 {
    fn write_color(&self) {
        println!(
            "{} {} {}",
            255.999 * self.x,
            255.999 * self.y,
            255.999 * self.z
        );
    }
}
