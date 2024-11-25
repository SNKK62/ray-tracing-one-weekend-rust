use super::{reflect, refract, Material};
use crate::{hittable, ray, vec3};
use rand::Rng;

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut vec3::Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        *attenuation = vec3::Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_direction = r_in.direction.unit();
        let cos_theta = (-unit_direction).dot(&rec.normal);
        if cos_theta < 0.0 {
            panic!("cos_theta must be positive!");
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0
            || rand::thread_rng().gen_range(0.0..1.0) < schlick(cos_theta, etai_over_etat)
        {
            // Must reflect if total internal reflection
            // Can reflect if Schlick says so
            let reflected = reflect(&unit_direction, &rec.normal);
            *scattered = ray::Ray::new(&rec.p, &reflected, r_in.time);
            return true;
        }
        let refracted = refract(&unit_direction, &rec.normal, etai_over_etat);
        *scattered = ray::Ray::new(&rec.p, &refracted, r_in.time);
        true
    }
}
