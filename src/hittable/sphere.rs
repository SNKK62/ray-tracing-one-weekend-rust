use crate::hittable::{HitRecord, Hittable, AABB};
use crate::material::MaterialEnum;
use crate::ray;
use crate::vec3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sphere {
    center: vec3::Point3,
    radius: f64,
    material: MaterialEnum,
}

impl Sphere {
    pub fn new(center: &vec3::Point3, radius: f64, material: MaterialEnum) -> Self {
        Self {
            center: *center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
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
                let (sphere_u, sphere_v) =
                    super::get_sphere_uv(&((record.p - self.center) / self.radius));
                record.u = sphere_u;
                record.v = sphere_v;
                let outward_normal = (record.p - self.center) / self.radius;
                self.set_front_face(r, &outward_normal, record);
                record.material = Some(self.material.clone());
                return true;
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = r.at(record.t);
                let (sphere_u, sphere_v) =
                    super::get_sphere_uv(&((record.p - self.center) / self.radius));
                record.u = sphere_u;
                record.v = sphere_v;
                let outward_normal = (record.p - self.center) / self.radius;
                self.set_front_face(r, &outward_normal, record);
                record.material = Some(self.material.clone());
                return true;
            }
        }
        false
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let box0 = AABB::new(
            &(self.center - vec3::Point3::new(self.radius, self.radius, self.radius)),
            &(self.center + vec3::Point3::new(self.radius, self.radius, self.radius)),
        );
        *output_box = box0;
        true
    }
}
