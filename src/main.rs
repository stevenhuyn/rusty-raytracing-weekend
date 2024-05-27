#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

// TODO: Work out best way to do imports?
// Import with crate::{...}? separate?
use clap::Parser;
use image::{save_buffer, ColorType};
use renderer::render;
use std::time::Instant;
use window::render_window;

use crate::scene::{final_scene, smoke_cornell_box};

mod aabb;
mod camera;
mod hittable;
mod material;
mod perlin;
mod ray;
mod renderer;
mod scene;
mod texture;
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

    /// Don't render to a window
    #[clap(short = 'i', long)]
    headless: bool,
}

pub const ASPECT_RATIO: f64 = 1.0;
pub const DEFAULT_WIDTH: u32 = 600;
pub const MAX_DEPTH: u32 = 50;
pub const SAMPLE_PER_PIXELS: u32 = 50;

fn main() {
    let cli = Cli::parse();
    let width = cli.width.unwrap_or(DEFAULT_WIDTH);
    let height = cli.height.unwrap_or((width as f64 / ASPECT_RATIO) as u32);
    let mut filename = cli.filename.unwrap_or_else(|| "render.png".to_string());

    if !filename.ends_with(".png") {
        filename.push_str(".png");
    }

    let now = Instant::now();
    let (world, camera) = final_scene(width, height);
    let buffer: Vec<u8> = render(width, height, &world, camera);
    println!("Rendered in {}", now.elapsed().as_secs_f64());

    if cli.save {
        save_buffer(filename, &buffer, width, height, ColorType::Rgba8).unwrap();
    }

    if !cli.headless {
        render_window(width, height, &buffer).unwrap();
    }
}
