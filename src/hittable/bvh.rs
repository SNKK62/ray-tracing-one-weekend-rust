use super::{surrounding_box, HitRecord, Hittable, AABB};
use crate::{ray, vec3};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: usize) -> std::cmp::Ordering {
    let mut box_a = AABB::new(&vec3::Point3::zero(), &vec3::Point3::zero());
    let mut box_b = AABB::new(&vec3::Point3::zero(), &vec3::Point3::zero());
    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        eprintln!("No bounding box in bvh_node constructor.");
    }
    box_a.min[axis].partial_cmp(&box_b.min[axis]).unwrap()
}

fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 2)
}

impl BvhNode {
    pub fn new(objects: &mut Vec<Rc<dyn Hittable>>, time0: f64, time1: f64) -> Self {
        Self::create(objects, 0, objects.len(), time0, time1)
    }
    pub fn create(
        objects: &mut Vec<Rc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let axis = rand::random::<usize>() % 3;
        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };

        let object_span = end - start;
        let left: Rc<dyn Hittable>;
        let right: Rc<dyn Hittable>;

        if object_span == 1 {
            // assign the same object to both left and right
            left = objects.get(start).unwrap().clone();
            right = objects.get(start).unwrap().clone();
        } else if object_span == 2 {
            // assign the first object to left and the second object to right
            let first = objects.get(start).unwrap().clone();
            let second = objects.get(start + 1).unwrap().clone();
            if comparator(&first, &second) == std::cmp::Ordering::Less {
                left = first;
                right = second;
            } else {
                left = second;
                right = first;
            }
        } else {
            objects[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            left = Rc::new(BvhNode::create(objects, start, mid, time0, time1));
            right = Rc::new(BvhNode::create(objects, mid, end, time0, time1));
        }

        let mut box_left = AABB::new(&vec3::Point3::zero(), &vec3::Point3::zero());
        let mut box_right = AABB::new(&vec3::Point3::zero(), &vec3::Point3::zero());
        if !left.bounding_box(time0, time1, &mut box_left)
            || !right.bounding_box(time0, time1, &mut box_right)
        {
            eprintln!("No bounding box in bvh_node constructor.");
        }
        let bbox = surrounding_box(&box_left, &box_right);

        Self { left, right, bbox }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, t_min, t_max) {
            return false;
        }
        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let tt_max = if hit_left { rec.t } else { t_max };
        let hit_right = self.right.hit(r, t_min, tt_max, rec);
        hit_left || hit_right
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let mut left_box = AABB::new(&vec3::Point3::zero(), &vec3::Point3::zero());
        let mut right_box = AABB::new(&vec3::Point3::zero(), &vec3::Point3::zero());
        if self.left.bounding_box(time0, time1, &mut left_box)
            && self.right.bounding_box(time0, time1, &mut right_box)
        {
            *output_box = surrounding_box(&left_box, &right_box);
            true
        } else {
            panic!("No bounding box in bvh_node::bounding_box.");
        }
    }
}
