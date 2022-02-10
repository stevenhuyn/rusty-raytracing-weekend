use image::io::Reader;

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
    scale: f64,
}

impl NoiseTexture {
    pub fn new_box(scale: f64) -> Box<Self> {
        Box::new(NoiseTexture {
            noise: Perlin::new(),
            scale,
        })
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

// TODO: Switch out and just use Image library?
pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
    bytes_per_pixel: usize,
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip v to image coordinates

        // Clamp integer mapping, since actual coordinates should be less than 1.0
        let mut i = (u * self.width as f64) as usize;
        let mut j = (v * self.height as f64) as usize;

        if i >= self.width {
            i = self.width - 1;
        }

        if j >= self.height {
            j = self.height - 1;
        }

        let color_scale = 1.0 / 255.0;
        let pixel_index = i * self.bytes_per_pixel + j * self.width * self.bytes_per_pixel;
        let pixel = &self.data[pixel_index..pixel_index + self.bytes_per_pixel];

        Color::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64) * color_scale
    }
}

impl ImageTexture {
    pub fn new(data: Vec<u8>, width: usize, height: usize, bytes_per_pixel: usize) -> Self {
        ImageTexture {
            data,
            width,
            height,
            bytes_per_pixel,
        }
    }
}
