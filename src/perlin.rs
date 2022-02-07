use std::mem;

use itertools::Itertools;
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
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i64;
        let j = p.y.floor() as i64;
        let k = p.z.floor() as i64;

        let mut c: [[[f64; 2]; 2]; 2] = [[[0f64; 2]; 2]; 2];

        for ((di, dj), dk) in (0..2).cartesian_product(0..2).cartesian_product(0..2) {
            let float_index = (self.perm_x[((i + di as i64) & 255) as usize]
                ^ self.perm_y[((j + dj as i64) & 255) as usize]
                ^ self.perm_z[((k + dk as i64) & 255) as usize])
                as usize;

            c[di][dj][dk] = self.ran_float[float_index];
        }

        Self::trilinear_interp(&c, u, v, w)
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

    fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for ((i, j), k) in (0..2).cartesian_product(0..2).cartesian_product(0..2) {
            let i_f = i as f64;
            let j_f = j as f64;
            let k_f = k as f64;
            accum += (i_f * u + (1.0 - i_f) * (1.0 - u))
                * (j_f * v + (1.0 - j_f) * (1.0 - v))
                * (k_f * w + (1.0 - k_f) * (1.0 - w))
                * c[i][j][k];
        }

        accum
    }
}
