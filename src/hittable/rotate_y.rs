use super::{HitRecord, Hittable, AABB};
use crate::degrees_to_radians;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RotateY {
    ptr: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: AABB,
}

impl RotateY {
    pub fn new(ptr: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);
        let mut bbox = AABB::new(&Vec3::zero(), &Vec3::zero());
        let hasbox = ptr.bounding_box(0.0, 1.0, &mut bbox);
        let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max.x() + (1 - i) as f64 * bbox.min.x();
                    let y = j as f64 * bbox.max.y() + (1 - j) as f64 * bbox.min.y();
                    let z = k as f64 * bbox.max.z() + (1 - k) as f64 * bbox.min.z();
                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);
                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        let bbox = AABB::new(&min, &max);

        Self {
            ptr,
            sin_theta,
            cos_theta,
            hasbox,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let origin = r.origin;
        let direction = r.direction;
        let origin = Vec3::new(
            self.cos_theta * origin.x() - self.sin_theta * origin.z(),
            origin.y(),
            self.sin_theta * origin.x() + self.cos_theta * origin.z(),
        );
        let direction = Vec3::new(
            self.cos_theta * direction.x() - self.sin_theta * direction.z(),
            direction.y(),
            self.sin_theta * direction.x() + self.cos_theta * direction.z(),
        );
        let rotated_r = Ray::new(&origin, &direction, r.time);
        if !self.ptr.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }
        let mut p = rec.p;
        let mut normal = rec.normal;

        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];
        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        self.set_front_face(&rotated_r, &normal, rec);

        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox.clone();
        self.hasbox
    }
}
