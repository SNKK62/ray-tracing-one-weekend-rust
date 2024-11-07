use std::cell::RefCell;
use std::rc::Rc;

use crate::material;
use crate::ray;
use crate::vec3;

#[derive(Clone)]
pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub material: Option<Rc<RefCell<dyn material::Material>>>,
    pub t: f64,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            p: vec3::Point3::zero(),
            normal: vec3::Vec3::zero(),
            material: None,
            t: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn set_front_face(&self, r: &ray::Ray, outward_normal: &vec3::Vec3, record: &mut HitRecord) {
        let is_front_face = r.direction.dot(outward_normal) <= 0.0;
        record.front_face = is_front_face;
        record.normal = if is_front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}

pub struct Sphere {
    center: vec3::Point3,
    radius: f64,
    material: Rc<RefCell<dyn material::Material>>,
}

impl Sphere {
    pub fn new(
        center: &vec3::Point3,
        radius: f64,
        material: Rc<RefCell<dyn material::Material>>,
    ) -> Self {
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
                let outward_normal = (record.p - self.center) / self.radius;
                self.set_front_face(r, &outward_normal, record);
                record.material = Some(Rc::clone(&self.material));
                return true;
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = r.at(record.t);
                let outward_normal = (record.p - self.center) / self.radius;
                self.set_front_face(r, &outward_normal, record);
                record.material = Some(Rc::clone(&self.material));
                return true;
            }
        }
        false
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
