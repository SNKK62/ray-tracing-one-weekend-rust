use super::{HitRecord, Hittable, HittableEnum, AABB};
use crate::vec3::Vec3;

use std::boxed::Box;

#[derive(Debug, Clone)]
pub struct Translation {
    offset: Vec3,
    ptr: Box<HittableEnum>,
}

impl Translation {
    pub fn new(ptr: HittableEnum, offset: Vec3) -> Self {
        Translation {
            ptr: Box::new(ptr),
            offset,
        }
    }
}

impl Hittable for Translation {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = crate::ray::Ray::new(&(r.origin - self.offset), &r.direction, r.time);
        if !self.ptr.hit(&moved_r, t_min, t_max, rec) {
            return false;
        }

        let normal = rec.normal;
        rec.p += self.offset;
        self.set_front_face(&moved_r, &normal, rec);
        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if !self.ptr.bounding_box(time0, time1, output_box) {
            return false;
        }

        *output_box = AABB::new(
            &(output_box.min + self.offset),
            &(output_box.max + self.offset),
        );

        true
    }
}
