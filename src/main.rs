use std::{
    error::Error,
    io::{stdout, Write},
};

use hittable::{HitRecord, Hittable};
use rand::random;
use ray::Ray;
use vec3::{Color, Point3, Vec3};

use crate::{
    camera::Camera,
    hittable::{hittable_list::HittableList, sphere::Sphere},
    vec3::VecOps,
};

mod camera;
mod hittable;
mod ray;
mod vec3;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut hit_record = HitRecord::new(); // TODO: Option?
    if world.hit(ray, 0.0, f64::INFINITY, &mut hit_record) {
        return 0.5 * (hit_record.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1f64);

    (1f64 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i64 = 400;
    let image_height: i64 = (image_width as f64 / aspect_ratio) as i64;
    let samples_per_pixel: i64 = 100;

    // World
    let world: HittableList = vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    // Camera
    let camera = Camera::new();

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScnalines remaining: {}", j);
        stdout().flush()?;
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + random::<f64>()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }
            pixel_color.write_color(samples_per_pixel);
        }
    }

    Ok(())
}
