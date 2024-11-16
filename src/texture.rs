use crate::vec3::{Color, Vec3};
mod noise;

pub mod solid_color;
pub use solid_color::SolidColor;

pub mod checker;
pub use checker::Checker;

pub mod noise_texture;
pub use noise_texture::NoiseTexture;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}
