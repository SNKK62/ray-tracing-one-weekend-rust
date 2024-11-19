use super::Material;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::{Color, Vec3};
use std::rc::Rc;

pub struct Isotropic {
    albedo: Rc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(&rec.p, &Vec3::rand_unit_sphere(), r_in.time);
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}
