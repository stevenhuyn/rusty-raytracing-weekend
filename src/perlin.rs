use std::mem;

use rand::{prelude::SliceRandom, thread_rng, Rng};

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
        let mut ran_float: [f64; POINT_COUNT] = [0.0; POINT_COUNT];
        for float in ran_float.iter_mut() {
            *float = random_double(0.0, 1.0);
        }

        Perlin {
            ran_float,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let i = (4.0 * p.x).abs() as usize & 255;
        let j = (4.0 * p.y).abs() as usize & 255;
        let k = (4.0 * p.z).abs() as usize & 255;

        let float_index = (self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize;

        self.ran_float[float_index]
    }

    fn perlin_generate_perm() -> [i64; POINT_COUNT] {
        let mut rng = thread_rng();
        let mut perm = [0i64; POINT_COUNT];

        for (i, p) in perm.iter_mut().enumerate() {
            *p = i as i64;
        }

        perm.shuffle(&mut rng);
        perm
    }

    // TODO: Figure why I don't know how to mutate this slice
    // fn permute(p: &mut [i64], n: usize) {
    //     for i in (n - 1)..0 {
    //         let target = rng.gen_range(0..i);
    //         p.swap(i, target);
    //     }
    // }
}
