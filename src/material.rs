use crate::hittable;
use crate::ray;
use crate::vec3;

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
        _r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut vec3::Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        let scatter_direction = rec.normal + vec3::Vec3::rand_unit_vector();
        *scattered = ray::Ray::new(&rec.p, &scatter_direction);
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
        );
        *attenuation = self.albedo;
        scattered.direction.dot(&rec.normal) > 0.0
    }
}
