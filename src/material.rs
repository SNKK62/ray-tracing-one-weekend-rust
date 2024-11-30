use crate::hittable;
use crate::ray;
use crate::vec3;

pub mod dielectric;
pub use dielectric::Dielectric;

pub mod lambertian;
pub use lambertian::Lambertian;

pub mod metal;
pub use metal::Metal;

pub mod diffuse_light;
pub use diffuse_light::DiffuseLight;

pub mod isotropic;
pub use isotropic::Isotropic;

use std::fmt::Debug;

pub trait Material: Debug + Send + Sync {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut vec3::Color,
        scattered: &mut ray::Ray,
    ) -> bool;
    fn emitted(&self, _u: f64, _v: f64, _p: &crate::vec3::Point3) -> crate::vec3::Color {
        vec3::Color::zero()
    }
}

#[derive(Debug, Clone)]
pub enum MaterialEnum {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
    Isotropic(Isotropic),
}

impl MaterialEnum {
    pub fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut vec3::Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        match self {
            MaterialEnum::Lambertian(l) => l.scatter(r_in, rec, attenuation, scattered),
            MaterialEnum::Metal(m) => m.scatter(r_in, rec, attenuation, scattered),
            MaterialEnum::Dielectric(d) => d.scatter(r_in, rec, attenuation, scattered),
            MaterialEnum::DiffuseLight(dl) => dl.scatter(r_in, rec, attenuation, scattered),
            MaterialEnum::Isotropic(i) => i.scatter(r_in, rec, attenuation, scattered),
        }
    }

    pub fn emitted(&self, u: f64, v: f64, p: &vec3::Point3) -> vec3::Color {
        match self {
            MaterialEnum::DiffuseLight(dl) => dl.emitted(u, v, p),
            _ => vec3::Color::zero(),
        }
    }
}

impl Material for MaterialEnum {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut vec3::Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        self.scatter(r_in, rec, attenuation, scattered)
    }

    fn emitted(&self, u: f64, v: f64, p: &vec3::Point3) -> vec3::Color {
        self.emitted(u, v, p)
    }
}

fn refract(uv: &vec3::Vec3, n: &vec3::Vec3, etai_over_etat: f64) -> vec3::Vec3 {
    let cos_theta = (-*uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -(1.0 - r_out_perp.power()).sqrt() * *n;
    r_out_perp + r_out_parallel
}

fn reflect(v: &vec3::Vec3, n: &vec3::Vec3) -> vec3::Vec3 {
    *v - ((2.0 * v.dot(n)) * *n)
}
