use clap::Parser;
use image::{io::Reader, save_buffer, ColorType, ImageBuffer, RgbImage, RgbaImage};
use renderer::draw;
use std::io::Cursor;
use window::{render_window, HEIGHT, WIDTH};

mod camera;
mod hittable;
mod material;
mod ray;
mod renderer;
mod utils;
mod vec3;
mod window;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[clap(short, long)]
    width: Option<u32>,

    /// Number of times to greet
    #[clap(short, long)]
    height: Option<u32>,

    #[clap(short, long)]
    save: bool,

    #[clap(short, long)]
    filename: Option<String>,
}

fn main() {
    // let cli = Cli::parse();

    let buffer = draw(WIDTH, HEIGHT);
    save_buffer("image.png", &buffer, WIDTH, HEIGHT, ColorType::Rgba8).unwrap();
}

fn render_to_file(buffer: &[u8]) {
    let img = Reader::new(Cursor::new(buffer)).decode().unwrap();
    img.save("render.png").unwrap();
}
