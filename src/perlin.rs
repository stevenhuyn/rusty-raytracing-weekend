use crate::{utils::random_double, vec3::Point3};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ran_float: [f64; POINT_COUNT],
    perm_x: [i64; POINT_COUNT],
    perm_y: [i64; POINT_COUNT],
    perm_z: [i64; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let ran_float: [f64; POINT_COUNT] = [];
        for i in 0..POINT_COUNT {
            ran_float[i] = random_double(0.0, 1.0);
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let i = (4.0 * p.x) as usize & 255;
        let j = (4.0 * p.y) as usize & 255;
        let k = (4.0 * p.z) as usize & 255;

        let float_index = (self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize;
        self.ran_float[float_index]
    }

    fn perlin_generate_perm() {}
}
