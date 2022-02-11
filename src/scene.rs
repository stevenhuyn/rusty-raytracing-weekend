// Should this go under hittables?

use std::sync::Arc;

use image::{io::Reader, ColorType, GenericImageView};

use crate::{
    hittable::{
        bvh::Bvh, moving_sphere::MovingSphere, sphere::Sphere, world::World, xy_rect::XYRect,
    },
    material::{Dielectric, DiffuseLight, Lambertian, Material, Metal},
    texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor},
    utils::random_double,
    vec3::{Color, Point3, Vec3, VecOps},
};

pub fn two_spheres() -> World {
    let mut objects = World::new();

    let checker_texture = Box::new(CheckerTexture::new(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    let checker_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: checker_texture,
    });

    objects.push(Box::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        checker_material.clone(),
    )));
    objects.push(Box::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        checker_material,
    )));

    objects
}

pub fn two_perlin_spheres() -> World {
    let mut objects = World::new();

    let perlin_texture = NoiseTexture::new_box(4.0);

    let perlin_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: perlin_texture,
    });

    objects.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_material.clone(),
    )));
    objects.push(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_material,
    )));

    objects
}

pub fn earth_scene() -> World {
    let mut objects = World::new();

    // TODO: Make helper function
    // TODO: handle missing filename gracefully
    let earth_image = Reader::open("./textures/earthmap.jpg")
        .unwrap()
        .decode()
        .unwrap();

    let image_width = earth_image.width();
    let image_height = earth_image.height();
    let image_data = earth_image.into_rgba8().to_vec();

    let earth_texture = Box::new(ImageTexture::new(
        image_data,
        image_width as usize,
        image_height as usize,
        ColorType::Rgba8.bytes_per_pixel().into(),
    ));

    let earth_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: earth_texture,
    });

    objects.push(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_material.clone(),
    )));

    objects
}

pub fn light_scene() -> World {
    let mut objects = World::new();

    let perlin_texture = NoiseTexture::new_box(4.0);

    let perlin_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: perlin_texture,
    });

    objects.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_material.clone(),
    )));
    objects.push(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_material,
    )));

    let diffuse_light: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    objects.push(Box::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        Arc::clone(&diffuse_light),
    )));

    let red_light: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(4.0, 1.0, 0.0)));

    objects.push(Box::new(Sphere::new(
        Point3::new(0.0, 8.0, 0.0),
        2.0,
        red_light,
    )));

    objects
}

pub fn random_scene() -> Bvh {
    let mut world: World = Vec::new();

    let odd_even_texture = Box::new(CheckerTexture::new(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let ground_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: odd_even_texture,
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
                    sphere_material = Arc::new(Lambertian {
                        albedo: Box::new(SolidColor { color: albedo }),
                    });
                    let centre2 = centre + Vec3::new(0.0, random_double(0.0, 0.5), 0.0);
                    world.push(Box::new(MovingSphere::new(
                        centre,
                        centre2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material.clone(),
                    )));
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
        albedo: Box::new(SolidColor::new(0.4, 0.2, 0.1)),
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

    Bvh::new(world, 0.0, 1.0)
}
