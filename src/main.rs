use std::io::Cursor;

use renderer::draw;
use window::{render_window, HEIGHT, WIDTH};

use image::{io::Reader, save_buffer, ColorType, ImageBuffer, RgbImage, RgbaImage};

mod camera;
mod hittable;
mod material;
mod ray;
mod renderer;
mod utils;
mod vec3;
mod window;

fn main() {
    // render_window();

    let buffer = draw(WIDTH, HEIGHT);
    save_buffer("image.png", &buffer, WIDTH, HEIGHT, ColorType::Rgba8).unwrap();
}

fn render_to_file(buffer: &[u8]) {
    let img = Reader::new(Cursor::new(buffer)).decode().unwrap();
    img.save("render.png").unwrap();
}
