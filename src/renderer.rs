use crate::{
    camera::Camera,
    hittable::Hittable,
    ray::Ray,
    utils::random_double,
    vec3::{Color, Point3, Vec3},
    MAX_DEPTH, SAMPLE_PER_PIXELS,
};

use rayon::prelude::*;
use std::ops::{Div, Mul};

// TODO: Make this a paramater for scene? Scene struct?
lazy_static! {
    static ref BACKGROUND: Color = Color::new(0.7, 0.8, 1.0);
}

pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.hit(ray, 0.001, f64::INFINITY) {
        let emitted = hit_record
            .material
            .emitted(hit_record.u, hit_record.v, hit_record.point);

        if let Some((attenuation, scattered)) = hit_record.material.scatter(ray, &hit_record) {
            return emitted * attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return emitted;
        }
    }

    *BACKGROUND
}

pub fn render(image_width: u32, image_height: u32, scene: &dyn Hittable) -> Vec<u8> {
    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let aspect_ratio = image_width as f64 / image_height as f64;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (0..image_height)
        .into_par_iter()
        .rev()
        .flat_map(|x| (0..image_width).into_par_iter().map(move |y| (x, y)))
        .flat_map(|(j, i)| {
            let pixel_color = (0..SAMPLE_PER_PIXELS)
                .map(|_| {
                    let u = (i as f64 + random_double(-1.0, 1.0)) / (image_width - 1) as f64;
                    let v = (j as f64 + random_double(-1.0, 1.0)) / (image_height - 1) as f64;
                    let ray = camera.get_ray(u, v);
                    ray_color(&ray, scene, MAX_DEPTH)
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
