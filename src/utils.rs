use rand::random;

use crate::vec3::Vec3;

pub fn random_vec(lower: f64, upper: f64) -> Vec3 {
    Vec3::new(
        random_double(lower, upper),
        random_double(lower, upper),
        random_double(lower, upper),
    )
}

pub fn random_double(lower: f64, upper: f64) -> f64 {
    random::<f64>() * (upper - lower) + lower
}

pub fn random_in_unit_sphere() -> Vec3 {
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

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalize()
}

#[allow(dead_code)]
pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(*normal) > 0.0 {
        // Same direction as normal
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_in_unit_disc() -> Vec3 {
    loop {
        let p = Vec3::new(random_double(-1.0, 1.0), random_double(-1.0, 1.0), 0.0);
        if p.length_squared() <= 1.0 {
            return p;
        }
    }
}
