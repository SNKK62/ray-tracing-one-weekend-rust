use crate::texture::TextureEnum;
use std::boxed::Box;

#[derive(Debug, Clone)]
pub struct Checker {
    odd: Box<TextureEnum>,
    even: Box<TextureEnum>,
}

impl Checker {
    pub fn new(odd: TextureEnum, even: TextureEnum) -> Self {
        Self {
            odd: Box::new(odd),
            even: Box::new(even),
        }
    }
}

impl super::Texture for Checker {
    fn value(&self, u: f64, v: f64, p: &crate::vec3::Vec3) -> crate::vec3::Color {
        let sines = f64::sin(10.0 * p.x()) * f64::sin(10.0 * p.y()) * f64::sin(10.0 * p.z());
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
