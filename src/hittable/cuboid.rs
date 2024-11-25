use super::{HitRecord, Hittable, HittableList, XYRect, XZRect, YZRect, AABB};
use crate::material::Material;
use crate::vec3::Point3;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Cuboid {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}

impl Cuboid {
    pub fn new(p0: &Point3, p1: &Point3, mat: Rc<RefCell<dyn Material>>) -> Self {
        let mut sides = HittableList::new();
        sides.add(Rc::new(XYRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            mat.clone(),
        )));
        sides.add(Rc::new(XYRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            mat.clone(),
        )));
        sides.add(Rc::new(XZRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            mat.clone(),
        )));
        sides.add(Rc::new(XZRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            mat.clone(),
        )));
        sides.add(Rc::new(YZRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            mat.clone(),
        )));
        sides.add(Rc::new(YZRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            mat.clone(),
        )));

        Cuboid {
            box_min: *p0,
            box_max: *p1,
            sides,
        }
    }
}

impl Hittable for Cuboid {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.sides.hit(r, t_min, t_max, rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(&self.box_min, &self.box_max);
        true
    }
}
