use crate::hittable;
use crate::ray;
use crate::vec3;

pub mod dielectric;
pub use dielectric::Dielectric;

pub mod lambertian;
pub use lambertian::Lambertian;

pub mod metal;
pub use metal::Metal;

pub trait Material {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut vec3::Color,
        scattered: &mut ray::Ray,
    ) -> bool;
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
