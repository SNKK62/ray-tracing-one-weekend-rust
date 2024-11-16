use crate::vec3::{Color, Vec3};

pub mod solid_color;
pub use solid_color::SolidColor;

pub mod checker;
pub use checker::Checker;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}
