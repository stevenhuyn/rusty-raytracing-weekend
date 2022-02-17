// TODO: Trait??

use std::sync::Arc;

use glam::Mat2;
use image::{io::Reader, ColorType, GenericImageView};

use crate::{
    camera::Camera,
    hittable::{
        box_rect::BoxRect, bvh::Bvh, constant_medium::ConstantMedium, hittable_list::HittableList,
        moving_sphere::MovingSphere, rotate_y::RotateY, sphere::Sphere, translate::Translate,
        xy_rect::XYRect, xz_rect::XZRect, yz_rect::YZRect, Hittable,
    },
    material::{Dielectric, DiffuseLight, Lambertian, Material, Metal},
    texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor},
    utils::{random_double, random_vec},
    vec3::{Color, Point3, Vec3, VecOps},
};

pub fn two_spheres(image_width: u32, image_height: u32) -> (HittableList, Camera) {
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

    let mut objects = HittableList::new();

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

    (objects, camera)
}

pub fn two_perlin_spheres(image_width: u32, image_height: u32) -> (HittableList, Camera) {
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

    let mut objects = HittableList::new();

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

    (objects, camera)
}

pub fn earth_scene(image_width: u32, image_height: u32) -> (HittableList, Camera) {
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

    let mut objects = HittableList::new();

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

    (objects, camera)
}

pub fn light_scene(image_width: u32, image_height: u32) -> (HittableList, Camera) {
    // Camera
    let lookfrom = Point3::new(26.0, 3.0, 6.0);
    let lookat = Point3::new(0.0, 2.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 50.0;
    let aperture = 0.1;
    let aspect_ratio = image_width as f64 / image_height as f64;
    let vfov = 20.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let mut objects = HittableList::new();

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

    (objects, camera)
}

pub fn cornell_box(image_width: u32, image_height: u32) -> (HittableList, Camera) {
    // Camera
    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let aspect_ratio = image_width as f64 / image_height as f64;
    let vfov = 40.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let mut objects = HittableList::new();

    let red_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Box::new(SolidColor::new(0.65, 0.05, 0.05)),
    });
    let white_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Box::new(SolidColor::new(0.73, 0.73, 0.73)),
    });
    let green_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Box::new(SolidColor::new(0.12, 0.45, 0.15)),
    });
    let light_material: Arc<dyn Material> =
        Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    // Left & Right wall
    objects.push(Box::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&green_material),
    )));
    objects.push(Box::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&red_material),
    )));

    // Light
    objects.push(Box::new(XZRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::clone(&light_material),
    )));

    objects.push(Box::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&white_material),
    )));
    objects.push(Box::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white_material),
    )));
    objects.push(Box::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white_material),
    )));

    // Triple box sus
    let mut box1: Box<dyn Hittable> = Box::new(BoxRect::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        Arc::clone(&white_material),
    ));
    box1 = Box::new(RotateY::new(box1, 15.0));
    box1 = Box::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    objects.push(box1);

    let mut box2: Box<dyn Hittable> = Box::new(BoxRect::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        Arc::clone(&white_material),
    ));
    box2 = Box::new(RotateY::new(box2, -18.));
    box2 = Box::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    objects.push(box2);

    (objects, camera)
}

pub fn smoke_cornell_box(image_width: u32, image_height: u32) -> (HittableList, Camera) {
    // Camera
    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let aspect_ratio = image_width as f64 / image_height as f64;
    let vfov = 40.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let mut objects = HittableList::new();

    let red_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Box::new(SolidColor::new(0.65, 0.05, 0.05)),
    });
    let white_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Box::new(SolidColor::new(0.73, 0.73, 0.73)),
    });
    let green_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Box::new(SolidColor::new(0.12, 0.45, 0.15)),
    });
    let light_material: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));

    // Left & Right wall
    objects.push(Box::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&green_material),
    )));
    objects.push(Box::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&red_material),
    )));

    // Light
    objects.push(Box::new(XZRect::new(
        113.0,
        443.0,
        127.0,
        432.0,
        554.0,
        Arc::clone(&light_material),
    )));

    objects.push(Box::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&white_material),
    )));
    objects.push(Box::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white_material),
    )));
    objects.push(Box::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white_material),
    )));

    // Triple Box::new() sus
    let mut box1: Box<dyn Hittable> = Box::new(BoxRect::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        Arc::clone(&white_material),
    ));
    box1 = Box::new(RotateY::new(box1, 15.0));
    box1 = Box::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    box1 = Box::new(ConstantMedium::new(box1, 0.01, Color::new(0.0, 0.0, 0.0)));
    objects.push(box1);

    let mut box2: Box<dyn Hittable> = Box::new(BoxRect::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        Arc::clone(&white_material),
    ));
    box2 = Box::new(RotateY::new(box2, -18.));
    box2 = Box::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    box2 = Box::new(ConstantMedium::new(box2, 0.01, Color::new(1.0, 1.0, 1.0)));
    objects.push(box2);

    (objects, camera)
}

pub fn final_scene(image_width: u32, image_height: u32) -> (HittableList, Camera) {
    // Camera
    let lookfrom = Point3::new(478.0, 278.0, -600.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let aspect_ratio = image_width as f64 / image_height as f64;
    let vfov = 40.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let mut boxes = HittableList::new();
    let ground: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Box::new(SolidColor::new(0.48, 0.83, 0.53)),
    });

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let i = i as f64;
            let j = j as f64;
            let w = 100.0;
            let x0 = -1000.0 + i * w;
            let z0 = -1000.0 + j * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double(1.0, 101.0);
            let z1 = z0 + w;

            boxes.push(Box::new(BoxRect::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                Arc::clone(&ground),
            )));
        }
    }

    let mut objects = HittableList::new();

    objects.push(Box::new(Bvh::new(boxes, 0.0, 1.0)));

    let light_material: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));
    objects.push(Box::new(XZRect::new(
        123.0,
        432.0,
        147.0,
        412.0,
        554.0,
        Arc::clone(&light_material),
    )));

    let centre1 = Point3::new(400.0, 400.0, 200.0);
    let centre2 = centre1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Box::new(SolidColor::new(0.7, 0.3, 0.1)),
    });
    objects.push(Box::new(MovingSphere::new(
        centre1,
        centre2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    objects.push(Box::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric { ir: 1.5 }),
    )));

    objects.push(Box::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal {
            albedo: Color::new(0.8, 0.8, 0.9),
            fuzz: 1.0,
        }),
    )));

    objects.push(Box::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric { ir: 1.5 }),
    )));
    objects.push(Box::new(ConstantMedium::new(
        Box::new(Sphere::new(
            Point3::new(360.0, 150.0, 145.0),
            70.0,
            Arc::new(Dielectric { ir: 1.5 }),
        )),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));

    // Super cool fog effect
    let boundary = Box::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric { ir: 1.5 }),
    ));
    objects.push(Box::new(ConstantMedium::new(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

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

    let emat = Arc::new(Lambertian {
        albedo: earth_texture,
    });
    objects.push(Box::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));

    let pertext = Arc::new(Lambertian {
        albedo: NoiseTexture::new_box(0.1),
    });
    objects.push(Box::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        pertext,
    )));

    let mut boxes = HittableList::new();
    let white_material: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Box::new(SolidColor::new(0.73, 0.73, 0.73)),
    });
    let ns = 1000;
    for _ in 0..ns {
        boxes.push(Box::new(Sphere::new(
            random_vec(0.0, 165.0),
            10.0,
            Arc::clone(&white_material),
        )));
    }

    objects.push(Box::new(Translate::new(
        Box::new(RotateY::new(Box::new(Bvh::new(boxes, 0.0, 1.0)), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    (objects, camera)
}

pub fn random_scene(image_width: u32, image_height: u32) -> (Bvh, Camera) {
    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 40.0;
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let aspect_ratio = image_width as f64 / image_height as f64;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let mut world: HittableList = Vec::new();

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

    (Bvh::new(world, 0.0, 1.0), camera)
}
