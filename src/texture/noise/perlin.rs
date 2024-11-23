use crate::vec3::{Point3, Vec3};

pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

fn trilinear_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut acc = 0.0;
    #[allow(clippy::needless_range_loop)]
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let ii = i as f64;
                let jj = j as f64;
                let kk = k as f64;
                let weight_v = Vec3::new(u - ii, v - jj, w - kk);
                acc += (ii * u + (1.0 - ii) * (1.0 - u))
                    * (jj * v + (1.0 - jj) * (1.0 - v))
                    * (kk * w + (1.0 - kk) * (1.0 - w))
                    * c[i][j][k].dot(&weight_v);
            }
        }
    }
    acc
}

impl Perlin {
    const POINT_COUNT: usize = 256; // 8bit
    pub fn new() -> Self {
        let mut ranvec: Vec<Vec3> = Vec::with_capacity(Self::POINT_COUNT);
        for _ in 0..Self::POINT_COUNT {
            ranvec.push(Vec3::rand_range(-1.0, 1.0).unit());
        }

        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = p.x().floor();
        let j = p.y().floor();
        let k = p.z().floor();
        let mut u = p.x() - i;
        let mut v = p.y() - j;
        let mut w = p.z() - k;
        // Hermite interpolation
        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);
        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::zero(); 2]; 2]; 2];
        #[allow(clippy::needless_range_loop)]
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x
                        [((i as i64 + di as i64) & 255) as usize]
                        ^ self.perm_y[((j as i64 + dj as i64) & 255) as usize]
                        ^ self.perm_z[((k as i64 + dk as i64) & 255) as usize]];
                }
            }
        }
        trilinear_interp(&c, u, v, w)
    }

    /// default depth should be 7
    pub fn turb(&self, p: &Point3, depth: i64) -> f64 {
        let mut acc = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        for _ in 0..depth {
            acc += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        acc.abs()
    }

    fn perlin_generate_perm() -> Vec<usize> {
        let mut p = Vec::with_capacity(Self::POINT_COUNT);
        for i in 0..Self::POINT_COUNT {
            p.push(i);
        }
        Self::permute(&mut p, Self::POINT_COUNT);
        p
    }

    pub fn permute(p: &mut [usize], n: usize) {
        for i in (1..n).rev() {
            let target = rand::random::<u8>() % i as u8;
            p.swap(i, target as usize);
        }
    }
}
