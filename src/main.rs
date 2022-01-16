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
use std::{
    ops::{Div, Mul},
    time::Instant,
};
use vec3::{Color, Point3, Vec3};
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
const MAX_DEPTH: u32 = 50;
const SAMPLE_PER_PIXELS: u32 = 100;

fn random_double(lower: f64, upper: f64) -> f64 {
    random::<f64>() * (upper - lower) + lower
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::new(
            random_double(-1.0, 1.0),
            random_double(-1.0, 1.0),
            random_double(-1.0, 1.0),
        );
        if p.length_squared() >= 1f64 {
            continue;
        }
        return p;
    }
}

fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalize()
}

fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(*normal) > 0.0 {
        // Same direction as normal
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut hit_record = HitRecord::new(); // TODO: Option?
    if world.hit(ray, 0.001, f64::INFINITY, &mut hit_record) {
        return 0.5
            * ray_color(
                &Ray {
                    origin: hit_record.point,
                    direction: hit_record.normal + random_in_hemisphere(&hit_record.normal),
                },
                world,
                depth - 1,
            );
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

    (0..image_height)
        .rev()
        .cartesian_product(0..image_width)
        .flat_map(|(j, i)| {
            let pixel_color = (0..SAMPLE_PER_PIXELS)
                .map(|_| {
                    let u = (i as f64 + random_double(-1.0, 1.0)) / (image_width - 1) as f64;
                    let v = (j as f64 + random_double(-1.0, 1.0)) / (image_height - 1) as f64;
                    let ray = camera.get_ray(u, v);
                    ray_color(&ray, &world, MAX_DEPTH)
                })
                .fold(Color::new(0.0, 0.0, 0.0), |acc, e| acc + e)
                .div(SAMPLE_PER_PIXELS as f64)
                .powf(0.5)
                .mul(256f64);

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
