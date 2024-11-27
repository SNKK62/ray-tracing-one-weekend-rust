pub use super::Material;
pub use crate::texture::Texture;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &crate::ray::Ray,
        _rec: &crate::hittable::HitRecord,
        _attenuation: &mut crate::vec3::Color,
        _scattered: &mut crate::ray::Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: &crate::vec3::Point3) -> crate::vec3::Color {
        self.emit.value(u, v, p)
    }
}
