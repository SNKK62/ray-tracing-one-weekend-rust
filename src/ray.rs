use crate::hittable;
use crate::vec3;

pub struct Ray {
    pub origin: vec3::Point3,
    pub direction: vec3::Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: &vec3::Point3, direction: &vec3::Vec3, time: f64) -> Self {
        Ray {
            origin: *origin,
            direction: *direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> vec3::Point3 {
        self.origin + self.direction * t
    }

    pub fn color(
        &self,
        background: &vec3::Color,
        world: &dyn hittable::Hittable,
        depth: usize,
    ) -> vec3::Color {
        if depth == 0 {
            return vec3::Color::zero();
        }

        let mut rec = hittable::HitRecord::new();
        if !world.hit(self, 0.001, f64::INFINITY, &mut rec) {
            return *background;
        }

        let mat = rec.clone().material;
        if mat.is_none() {
            panic!("Material is None");
        }
        let mat = mat.unwrap();

        let emitted = mat.read().unwrap().emitted(rec.u, rec.v, &rec.p);
        let mut scattered = Self::new(&rec.p, &rec.normal, 0.0); // Temporary Ray
        let mut attenuation = vec3::Color::zero();

        if !mat
            .read()
            .unwrap()
            .scatter(self, &rec, &mut attenuation, &mut scattered)
        {
            return emitted;
        }

        emitted + attenuation * scattered.color(background, world, depth - 1)
    }
}
