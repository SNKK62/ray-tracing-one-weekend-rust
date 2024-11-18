use super::Material;
use crate::texture::Texture;
use crate::{hittable, ray, vec3};
use std::rc::Rc;

#[derive(Clone)]
pub struct Lambertian {
    albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut vec3::Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        let scatter_direction = rec.normal + vec3::Vec3::rand_unit_vector();
        *scattered = ray::Ray::new(&rec.p, &scatter_direction, r_in.time);
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}
