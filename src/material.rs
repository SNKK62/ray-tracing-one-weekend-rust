use crate::hittable;
use crate::ray;
use crate::vec3;
use rand::Rng;

pub trait Material {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut vec3::Color,
        scattered: &mut ray::Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: vec3::Color,
}

impl Lambertian {
    pub fn new(albedo: &vec3::Color) -> Self {
        Self { albedo: *albedo }
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
        *attenuation = self.albedo;
        true
    }
}

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

fn reflect(v: &vec3::Vec3, n: &vec3::Vec3) -> vec3::Vec3 {
    *v - ((2.0 * v.dot(n)) * *n)
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

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

fn refract(uv: &vec3::Vec3, n: &vec3::Vec3, etai_over_etat: f64) -> vec3::Vec3 {
    let cos_theta = (-*uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -(1.0 - r_out_perp.power()).sqrt() * *n;
    r_out_perp + r_out_parallel
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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
