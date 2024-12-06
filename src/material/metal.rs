use super::{reflect, Material};
use crate::{hittable, ray, vec3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metal {
    albedo: vec3::Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &vec3::Color, fuzz: f64) -> Self {
        Self {
            albedo: *albedo,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut vec3::Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        let reflected = reflect(&r_in.direction.unit(), &rec.normal);
        *scattered = ray::Ray::new(
            &rec.p,
            &(reflected + self.fuzz * vec3::Vec3::rand_unit_sphere()),
            r_in.time,
        );
        *attenuation = self.albedo;
        scattered.direction.dot(&rec.normal) > 0.0
    }
}
