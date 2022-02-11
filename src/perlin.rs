use itertools::Itertools;
use rand::{prelude::SliceRandom, thread_rng};

use crate::{
    utils::random_unit_vector,
    vec3::{Point3, Vec3},
};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ran_vec: [Vec3; POINT_COUNT],
    perm_x: [i64; POINT_COUNT],
    perm_y: [i64; POINT_COUNT],
    perm_z: [i64; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut ran_vec: [Vec3; POINT_COUNT] = [Vec3::default(); POINT_COUNT];
        for vec in ran_vec.iter_mut() {
            *vec = random_unit_vector();
        }

        Perlin {
            ran_vec,
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

        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::default(); 2]; 2]; 2];

        for ((di, dj), dk) in (0..2).cartesian_product(0..2).cartesian_product(0..2) {
            let vec_index = (self.perm_x[((i + di as i64) & 255) as usize]
                ^ self.perm_y[((j + dj as i64) & 255) as usize]
                ^ self.perm_z[((k + dk as i64) & 255) as usize])
                as usize;

            c[di][dj][dk] = self.ran_vec[vec_index];
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

    fn trilinear_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for ((i, j), k) in (0..2).cartesian_product(0..2).cartesian_product(0..2) {
            let i_f = i as f64;
            let j_f = j as f64;
            let k_f = k as f64;
            let weight_v = Vec3::new(u - i_f, v - j_f, w - k_f);
            accum += (i_f * uu + (1.0 - i_f) * (1.0 - uu))
                * (j_f * vv + (1.0 - j_f) * (1.0 - vv))
                * (k_f * ww + (1.0 - k_f) * (1.0 - ww))
                * c[i][j][k].dot(weight_v);
        }

        accum
    }

    pub fn turb(&self, p: Point3, depth: i64) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}
