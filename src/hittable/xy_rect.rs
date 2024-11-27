use super::{HitRecord, Hittable, AABB};
use crate::material::Material;
use crate::vec3::Vec3;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct XYRect {
    mp: Arc<Mutex<dyn Material>>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl XYRect {
    pub fn new(
        x0: f64,
        x1: f64,
        y0: f64,
        y1: f64,
        k: f64,
        material: Arc<Mutex<dyn Material>>,
    ) -> Self {
        XYRect {
            mp: material,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl Hittable for XYRect {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin.z()) / r.direction.z();
        if t < t_min || t > t_max {
            return false;
        }

        let x = r.origin.x() + t * r.direction.x();
        let y = r.origin.y() + t * r.direction.y();
        if (x < self.x0 || x > self.x1) || (y < self.y0 || y > self.y1) {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        self.set_front_face(r, &outward_normal, rec);
        rec.material = Some(self.mp.clone());
        rec.p = r.at(t);
        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        let bbox = AABB::new(
            &Vec3::new(self.x0, self.y0, self.k - 0.0001),
            &Vec3::new(self.x1, self.y1, self.k + 0.0001),
        );
        *output_box = bbox;
        true
    }
}
