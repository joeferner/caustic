use crate::{Color, Random, Vector3, texture::Texture};

#[derive(Debug)]
pub struct NoiseTexture {
    noise: Perlin,
}

impl NoiseTexture {
    pub fn new(random: &dyn Random) -> Self {
        Self {
            noise: Perlin::new(random),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, pt: Vector3) -> Color {
        Color::new(1.0, 1.0, 1.0) * self.noise.noise(pt)
    }
}

#[derive(Debug)]
pub struct Perlin {
    rand_float: [f64; Perlin::POINT_COUNT],
    perm_x: [usize; Perlin::POINT_COUNT],
    perm_y: [usize; Perlin::POINT_COUNT],
    perm_z: [usize; Perlin::POINT_COUNT],
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new(random: &dyn Random) -> Self {
        let mut rand_float: [f64; Perlin::POINT_COUNT] = [0.0; Perlin::POINT_COUNT];
        for item in rand_float.iter_mut() {
            *item = random.rand();
        }

        let perm_x = Perlin::generate_perm(random);
        let perm_y = Perlin::generate_perm(random);
        let perm_z = Perlin::generate_perm(random);

        Self {
            rand_float,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, pt: Vector3) -> f64 {
        let i = (((4.0 * pt.x) as i64) & 0xff) as usize;
        let j = (((4.0 * pt.y) as i64) & 0xff) as usize;
        let k = (((4.0 * pt.z) as i64) & 0xff) as usize;

        self.rand_float[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

    fn generate_perm(random: &dyn Random) -> [usize; Perlin::POINT_COUNT] {
        let mut p: [usize; Perlin::POINT_COUNT] = [0; Perlin::POINT_COUNT];
        for (i, v) in p.iter_mut().enumerate() {
            *v = i;
        }

        Perlin::permute(random, &mut p);
        p
    }

    fn permute(random: &dyn Random, p: &mut [usize; Perlin::POINT_COUNT]) {
        for i in (1..Perlin::POINT_COUNT - 1).rev() {
            let target = random.rand_int_interval(0, i as i64) as usize;
            p.swap(i, target);
        }
    }
}
