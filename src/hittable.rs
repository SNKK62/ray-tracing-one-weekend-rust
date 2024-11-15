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

#[derive(Clone)]
pub struct AABB {
    min: vec3::Point3,
    max: vec3::Point3,
}

impl AABB {
    pub fn new(min: &vec3::Point3, max: &vec3::Point3) -> Self {
        Self {
            min: *min,
            max: *max,
        }
    }

    pub fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let mut t0 = (self.min[a] - r.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - r.origin[a]) * inv_d;
            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let tt_min = t0.max(t_min);
            let tt_max = t1.min(t_max);
            if tt_max <= tt_min {
                return false;
            }
        }
        true
    }
}

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
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

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut temp_box = AABB::new(
            &vec3::Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            &vec3::Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY),
        );
        let mut first_box = true;

        for object in &self.objects {
            if !object.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box.clone()
            } else {
                surrounding_box(output_box, &temp_box)
            };
            first_box = false;
        }
        true
    }
}

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
            right = objects.get(start + 1).unwrap().clone();
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
            objects.sort_by(comparator);
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
        let hit_right = self.right.hit(r, t_min, t_max, rec);
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
            false
        }
    }
}
