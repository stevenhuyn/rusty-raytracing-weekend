use crate::{
    camera::Camera,
    hittable::{hittable_list::HittableList, sphere::Sphere},
};
use hittable::{HitRecord, Hittable};
use itertools::Itertools;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use rand::random;
use ray::Ray;
use std::error::Error;
use vec3::{Color, Point3};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

mod camera;
mod hittable;
mod ray;
mod vec3;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 450;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut hit_record = HitRecord::new(); // TODO: Option?
    if world.hit(ray, 0.0, f64::INFINITY, &mut hit_record) {
        return 0.5 * (hit_record.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1f64);

    (1f64 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn draw(image_width: u32, image_height: u32) -> Vec<u8> {
    // World
    let world: HittableList = vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    // Camera
    let camera = Camera::new();

    let samples_per_pixel: u32 = 100;

    (0..image_height)
        .rev()
        .cartesian_product(0..image_width)
        .flat_map(|(j, i)| {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + random::<f64>()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }
            pixel_color /= samples_per_pixel as f64;
            pixel_color *= 255.999f64;

            vec![
                pixel_color.x as u8,
                pixel_color.y as u8,
                pixel_color.z as u8,
                0xff,
            ]
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

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

    let render: Vec<u8> = draw(WIDTH, HEIGHT);
    pixels.get_frame().copy_from_slice(&render[..]);

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
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
