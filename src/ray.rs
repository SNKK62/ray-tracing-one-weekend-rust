use crate::hittable;
use crate::vec3;

pub struct Ray {
    pub origin: vec3::Point3,
    pub direction: vec3::Vec3,
}

impl Ray {
    pub fn new(origin: &vec3::Point3, direction: &vec3::Vec3) -> Self {
        Ray {
            origin: *origin,
            direction: *direction,
        }
    }

    pub fn at(&self, t: f64) -> vec3::Point3 {
        self.origin + self.direction * t
    }

    pub fn color(&self, world: &dyn hittable::Hittable) -> vec3::Color {
        let mut rec = hittable::HitRecord::new();
        if world.hit(self, 0.0, f64::INFINITY, &mut rec) {
            return 0.5 * (rec.normal + vec3::Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = self.direction.unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * vec3::Color::new(1.0, 1.0, 1.0) + t * vec3::Color::new(0.5, 0.7, 1.0)
    }
}
