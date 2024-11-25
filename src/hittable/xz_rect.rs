use super::{HitRecord, Hittable, AABB};
use crate::material::Material;
use crate::vec3::Vec3;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct XZRect {
    mp: Rc<RefCell<dyn Material>>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XZRect {
    pub fn new(
        x0: f64,
        x1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        material: Rc<RefCell<dyn Material>>,
    ) -> Self {
        XZRect {
            mp: material,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for XZRect {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin.y()) / r.direction.y();
        if t < t_min || t > t_max {
            return false;
        }

        let x = r.origin.x() + t * r.direction.x();
        let z = r.origin.z() + t * r.direction.z();
        if (x < self.x0 || x > self.x1) || (z < self.z0 || z > self.z1) {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        self.set_front_face(r, &outward_normal, rec);
        rec.material = Some(self.mp.clone());
        rec.p = r.at(t);
        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        let bbox = AABB::new(
            &Vec3::new(self.x0, self.k - 0.0001, self.z0),
            &Vec3::new(self.x1, self.k + 0.0001, self.z1),
        );
        *output_box = bbox;
        true
    }
}
