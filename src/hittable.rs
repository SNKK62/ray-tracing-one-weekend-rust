use std::cell::RefCell;
use std::rc::Rc;

use crate::material;
use crate::ray;
use crate::vec3;

pub mod aabb;
pub mod bvh;
pub mod hittable_list;
pub mod moving_sphere;
pub mod sphere;

use aabb::AABB;

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
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
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

fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = vec3::Point3::new(
        box0.min.x().min(box1.min.x()),
        box0.min.y().min(box1.min.y()),
        box0.min.z().min(box1.min.z()),
    );
    let big = vec3::Point3::new(
        box0.max.x().max(box1.max.x()),
        box0.max.y().max(box1.max.y()),
        box0.max.z().max(box1.max.z()),
    );
    AABB::new(&small, &big)
}
