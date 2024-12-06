use crate::vec3::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color_value: color }
    }
}

impl super::Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &crate::vec3::Vec3) -> Color {
        self.color_value
    }
}
