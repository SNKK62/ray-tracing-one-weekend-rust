use super::noise::perlin::Perlin;
use super::Texture;
use crate::vec3::{Color, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * (1.0 + f64::sin(self.scale * p.z() + 10.0 * self.noise.turb(p, 7)))
            * 0.5
    }
}
