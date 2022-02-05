use crate::vec3::{Color, Point3};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}
pub struct SolidColor {
    color: Color,
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        self.color
    }
}

impl SolidColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        SolidColor {
            color: Color::new(r, g, b),
        }
    }
}
