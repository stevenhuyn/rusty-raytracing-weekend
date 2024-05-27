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

pub fn render_window(width: u32, height: u32, buffer: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut input = WinitInputHelper::new();
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(width, height);
        WindowBuilder::new()
            .with_title("Rusty Raytracing")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let surface_texture = SurfaceTexture::new(width, height, &window);
        Pixels::new(width, height, surface_texture)?
    };

    let now = Instant::now();
    pixels.frame_mut().copy_from_slice(buffer);
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
                if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            window.request_redraw();
        }
    });
}
