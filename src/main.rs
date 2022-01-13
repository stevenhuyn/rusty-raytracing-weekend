use std::{
    error::Error,
    io::{stdout, Write},
};

use hittable::{HitRecord, Hittable};
use ray::Ray;
use vec3::{Color, Point3, Vec3};

use crate::{
    hittable::{hittable_list::HittableList, sphere::Sphere},
    vec3::VecOps,
};

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

    // World
    let world: HittableList = vec![
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
    ];

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height as f64;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScnalines remaining: {}", j);
        stdout().flush()?;
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let ray = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };

            let pixel_color = ray_color(&ray, &world);

            pixel_color.write_color();
        }
    }

    Ok(())
}
