use crate::vec3::{Color, Vec3};
mod noise;

pub mod solid_color;
pub use solid_color::SolidColor;

pub mod checker;
pub use checker::Checker;

pub mod noise_texture;
pub use noise_texture::NoiseTexture;

pub mod image_texture;
pub use image_texture::ImageTexture;

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TextureEnum {
    SolidColor(SolidColor),
    Checker(Checker),
    NoiseTexture(NoiseTexture),
    ImageTexture(ImageTexture),
}

impl TextureEnum {
    pub fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        match self {
            TextureEnum::SolidColor(sc) => sc.value(u, v, p),
            TextureEnum::Checker(c) => c.value(u, v, p),
            TextureEnum::NoiseTexture(nt) => nt.value(u, v, p),
            TextureEnum::ImageTexture(it) => it.value(u, v, p),
        }
    }
}

impl Texture for TextureEnum {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        self.value(u, v, p)
    }
}
