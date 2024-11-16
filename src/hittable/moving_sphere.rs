use crate::hittable::{surrounding_box, HitRecord, Hittable, AABB};
use crate::material;
use crate::ray;
use crate::vec3;
use std::{cell::RefCell, rc::Rc};

pub struct MovingSphere {
    center0: vec3::Point3,
    center1: vec3::Point3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Rc<RefCell<dyn material::Material>>,
}

impl MovingSphere {
    pub fn new(
        center0: &vec3::Point3,
        center1: &vec3::Point3,
        radius: f64,
        material: Rc<RefCell<dyn material::Material>>,
        time0: f64,
        time1: f64,
    ) -> Self {
        Self {
            center0: *center0,
            center1: *center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> vec3::Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc = r.origin - self.center(r.time);
        let a = r.direction.power();
        let half_b = oc.dot(&r.direction);
        let c = oc.power() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = r.at(record.t);
                let outward_normal = (record.p - self.center(r.time)) / self.radius;
                self.set_front_face(r, &outward_normal, record);
                record.material = Some(Rc::clone(&self.material));
                return true;
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = r.at(record.t);
                let outward_normal = (record.p - self.center(r.time)) / self.radius;
                self.set_front_face(r, &outward_normal, record);
                record.material = Some(Rc::clone(&self.material));
                return true;
            }
        }
        false
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let box0 = AABB::new(
            &(self.center(time0) - vec3::Vec3::new(self.radius, self.radius, self.radius)),
            &(self.center(time0) + vec3::Vec3::new(self.radius, self.radius, self.radius)),
        );
        let box1 = AABB::new(
            &(self.center(time1) - vec3::Vec3::new(self.radius, self.radius, self.radius)),
            &(self.center(time1) + vec3::Vec3::new(self.radius, self.radius, self.radius)),
        );
        *output_box = surrounding_box(&box0, &box1);
        true
    }
}
