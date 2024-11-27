use super::{HitRecord, Hittable, AABB};
use crate::material::Material;
use crate::vec3::Vec3;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct YZRect {
    mp: Arc<Mutex<dyn Material>>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl YZRect {
    pub fn new(
        y0: f64,
        y1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        material: Arc<Mutex<dyn Material>>,
    ) -> Self {
        YZRect {
            mp: material,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for YZRect {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin.x()) / r.direction.x();
        if t < t_min || t > t_max {
            return false;
        }

        let y = r.origin.y() + t * r.direction.y();
        let z = r.origin.z() + t * r.direction.z();
        if (y < self.y0 || y > self.y1) || (z < self.z0 || z > self.z1) {
            return false;
        }
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        self.set_front_face(r, &outward_normal, rec);
        rec.material = Some(self.mp.clone());
        rec.p = r.at(t);
        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        let bbox = AABB::new(
            &Vec3::new(self.k - 0.0001, self.y0, self.z0),
            &Vec3::new(self.k + 0.0001, self.y1, self.z1),
        );
        *output_box = bbox;
        true
    }
}
