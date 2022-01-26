use crate::{
    camera::Camera,
    hittable::{sphere::Sphere, world::World, Hittable},
    material::{Dielectric, Lambertian, Material, Metal},
    ray::Ray,
    utils::random_double,
    vec3::{Color, Point3, Vec3, VecOps},
    ASPECT_RATIO, MAX_DEPTH, SAMPLE_PER_PIXELS,
};
use rayon::prelude::*;
use std::{
    ops::{Div, Mul},
    sync::Arc,
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
    let mut world: World = Vec::new();

    let ground_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });

    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let glass_material: Arc<dyn Material> = Arc::new(Dielectric { ir: 1.5 });

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let choose_mat = random_double(0.0, 1.0);
            let centre = Point3::new(
                a + 0.9 * random_double(0.0, 1.0),
                0.2,
                b + 0.9 * random_double(0.0, 1.0),
            );

            if (centre - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    let albedo = Vec3::random_color() * Vec3::random_color();
                    sphere_material = Arc::new(Lambertian { albedo });
                    world.push(Box::new(Sphere::new(centre, 0.2, sphere_material.clone())));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_color();
                    let fuzz = random_double(0.0, 0.5);
                    sphere_material = Arc::new(Metal { albedo, fuzz });
                    world.push(Box::new(Sphere::new(centre, 0.2, sphere_material.clone())));
                } else {
                    world.push(Box::new(Sphere::new(centre, 0.2, glass_material.clone())));
                }
            }
        }
    }

    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        glass_material,
    )));

    let big_lamb: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    world.push(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        big_lamb,
    )));

    let big_metal = Arc::new(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.push(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        big_metal,
    )));

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
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
