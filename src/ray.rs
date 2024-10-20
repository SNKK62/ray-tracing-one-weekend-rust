use crate::vec3;

pub struct Ray {
    origin: vec3::Point3,
    direction: vec3::Vec3,
}

impl Ray {
    pub fn new(origin: &vec3::Point3, direction: &vec3::Vec3) -> Self {
        Ray {
            origin: *origin,
            direction: *direction,
        }
    }

    pub fn origin(&self) -> &vec3::Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &vec3::Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> vec3::Point3 {
        self.origin + self.direction * t
    }

    pub fn hit_sphere(center: &vec3::Point3, radius: f64, r: &Ray) -> bool {
        let oc = r.origin - *center;
        let a = r.direction.dot(&r.direction);
        let b = 2.0 * oc.dot(&r.direction);
        let c = oc.dot(&oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0
    }

    pub fn color(&self) -> vec3::Color {
        if Self::hit_sphere(&vec3::Point3::new(0.0, 0.0, -1.0), 0.5, self) {
            return vec3::Color::new(1.0, 0.0, 0.0); // red
        }
        let unit_direction = self.direction.unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * vec3::Color::new(1.0, 1.0, 1.0) + t * vec3::Color::new(0.5, 0.7, 1.0)
    }
}
