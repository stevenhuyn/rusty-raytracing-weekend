use clap::Parser;
use image::{save_buffer, ColorType};
use renderer::draw;
use std::time::Instant;
use window::render_window;

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
    /// Width of resulting render in pixels
    #[clap(short, long)]
    width: Option<u32>,

    /// Height of resulting render in pixels
    #[clap(short, long)]
    height: Option<u32>,

    /// Whether to save file
    #[clap(short, long)]
    save: bool,

    /// Specify filename of render if saved
    #[clap(short, long)]
    filename: Option<String>,
}

pub const ASPECT_RATIO: f64 = 3.0 / 2.0;
pub const DEFAULT_WIDTH: u32 = 400;
pub const MAX_DEPTH: u32 = 50;
pub const SAMPLE_PER_PIXELS: u32 = 500;

fn main() {
    let cli = Cli::parse();
    let width = cli.width.unwrap_or(DEFAULT_WIDTH);
    let height = cli.height.unwrap_or((width as f64 / ASPECT_RATIO) as u32);
    let filename = cli.filename.unwrap_or_else(|| "render.png".to_string());

    let now = Instant::now();
    let buffer: Vec<u8> = draw(width, height);
    println!("Rendered in {}", now.elapsed().as_secs_f64());

    if cli.save {
        save_buffer(filename, &buffer, width, height, ColorType::Rgba8).unwrap();
    }

    // How to not use handle here?
    render_window(width, height, &buffer).unwrap();
}
