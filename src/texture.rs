use crate::{
    perlin::Perlin,
    vec3::{Color, Point3},
};

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}
pub struct SolidColor {
    pub color: Color,
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
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

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(even: Color, odd: Color) -> Self {
        CheckerTexture {
            even: Box::new(SolidColor { color: even }),
            odd: Box::new(SolidColor { color: odd }),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
}

impl NoiseTexture {
    pub fn new_box() -> Box<Self> {
        Box::new(NoiseTexture {
            noise: Perlin::new(),
        })
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        // noise noise tbh weird
        Color::new(1.0, 1.0, 1.0) * self.noise.noise(p)
    }
}
