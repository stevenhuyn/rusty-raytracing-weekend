use itertools::Itertools;
use std::{
    ops::{Div, Mul},
    rc::Rc,
};

use crate::{
    camera::Camera,
    hittable::{hittable_list::HittableList, sphere::Sphere, Hittable},
    material::{Dielectric, Lambertian, Material, Metal},
    ray::Ray,
    utils::random_double,
    vec3::{Color, Point3},
    MAX_DEPTH, SAMPLE_PER_PIXELS,
};

pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = hit_record.material.scatter(ray, &hit_record) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1f64);

    (1f64 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

pub fn draw(image_width: u32, image_height: u32) -> Vec<u8> {
    // Materials
    let material_ground: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let diffuse_blue: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    });
    let glass_material: Rc<dyn Material> = Rc::new(Dielectric { ir: 1.5 });

    let gold_material: Rc<dyn Material> = Rc::new(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    });
    let light_metal: Rc<dyn Material> = Rc::new(Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
        fuzz: 0.3,
    });

    // World
    let world: HittableList = vec![
        Box::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Rc::clone(&diffuse_blue),
        )),
        Box::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Rc::clone(&material_ground),
        )),
        Box::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            Rc::clone(&glass_material),
        )),
        Box::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            -0.4,
            Rc::clone(&glass_material),
        )),
        Box::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            Rc::clone(&gold_material),
        )),
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
