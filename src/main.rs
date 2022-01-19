use pixels::{Pixels, SurfaceTexture};
use std::error::Error;
use std::time::Instant;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::renderer::draw;

mod camera;
mod hittable;
mod material;
mod ray;
mod renderer;
mod utils;
mod vec3;

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
pub const MAX_DEPTH: u32 = 50;
pub const SAMPLE_PER_PIXELS: u32 = 100;

fn main() -> Result<(), Box<dyn Error>> {
    let mut now = Instant::now();
    let render: Vec<u8> = draw(WIDTH, HEIGHT);
    println!("Rendered in {}", now.elapsed().as_secs_f64());

    let mut input = WinitInputHelper::new();
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH, HEIGHT);
        WindowBuilder::new()
            .with_title("Rusty Raytracing")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    now = Instant::now();
    pixels.get_frame().copy_from_slice(&render[..]);
    println!("Copied frame buffer in {}", now.elapsed().as_secs_f64());

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            window.request_redraw();
        }
    });
}
