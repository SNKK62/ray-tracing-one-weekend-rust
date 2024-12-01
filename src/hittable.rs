use crate::material;
use crate::ray;
use crate::vec3;

pub mod aabb;
pub use aabb::AABB;

pub mod bvh;
pub use bvh::BvhNode;

pub mod hittable_list;
pub use hittable_list::HittableList;

pub mod moving_sphere;
pub use moving_sphere::MovingSphere;

pub mod sphere;
pub use sphere::Sphere;

pub mod xy_rect;
pub use xy_rect::XYRect;
pub mod xz_rect;
pub use xz_rect::XZRect;
pub mod yz_rect;
pub use yz_rect::YZRect;

pub mod cuboid;
pub use cuboid::Cuboid;

pub mod translation;
pub use translation::Translation;

pub mod rotate_x;
pub use rotate_x::RotateX;
pub mod rotate_y;
pub use rotate_y::RotateY;
pub mod rotate_z;
pub use rotate_z::RotateZ;

pub mod constant_medium;
pub use constant_medium::ConstantMedium;

use std::boxed::Box;
use std::fmt::Debug;

/// p should be a unit sphere
fn get_sphere_uv(p: &vec3::Point3) -> (f64, f64) {
    let pi = std::f64::consts::PI;
    let phi = f64::atan2(p.z(), p.x());
    let theta = f64::asin(p.y());
    let u = 1.0 - (phi + pi) / (2.0 * pi);
    let v = (theta + pi / 2.0) / pi;
    (u, v)
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub material: Option<material::MaterialEnum>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
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
            u: 0.0,
            v: 0.0,
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

#[derive(Debug, Clone)]
pub enum HittableEnum {
    HittableList(Box<HittableList>),
    BvhNode(Box<BvhNode>),
    Sphere(Sphere),
    MovingSphere(MovingSphere),
    XYRect(XYRect),
    XZRect(XZRect),
    YZRect(YZRect),
    Cuboid(Cuboid),
    Translation(Translation),
    RotateX(Box<RotateX>),
    RotateY(Box<RotateY>),
    RotateZ(Box<RotateZ>),
    ConstantMedium(Box<ConstantMedium>),
}

impl HittableEnum {
    pub fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        match self {
            HittableEnum::HittableList(h) => h.hit(r, t_min, t_max, rec),
            HittableEnum::BvhNode(b) => b.hit(r, t_min, t_max, rec),
            HittableEnum::Sphere(s) => s.hit(r, t_min, t_max, rec),
            HittableEnum::MovingSphere(s) => s.hit(r, t_min, t_max, rec),
            HittableEnum::XYRect(rect) => rect.hit(r, t_min, t_max, rec),
            HittableEnum::XZRect(rect) => rect.hit(r, t_min, t_max, rec),
            HittableEnum::YZRect(rect) => rect.hit(r, t_min, t_max, rec),
            HittableEnum::Cuboid(c) => c.hit(r, t_min, t_max, rec),
            HittableEnum::Translation(t) => t.hit(r, t_min, t_max, rec),
            HittableEnum::RotateX(rotate) => rotate.hit(r, t_min, t_max, rec),
            HittableEnum::RotateY(rotate) => rotate.hit(r, t_min, t_max, rec),
            HittableEnum::RotateZ(rotate) => rotate.hit(r, t_min, t_max, rec),
            HittableEnum::ConstantMedium(c) => c.hit(r, t_min, t_max, rec),
        }
    }

    pub fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        match self {
            HittableEnum::HittableList(h) => h.bounding_box(time0, time1, output_box),
            HittableEnum::BvhNode(b) => b.bounding_box(time0, time1, output_box),
            HittableEnum::Sphere(s) => s.bounding_box(time0, time1, output_box),
            HittableEnum::MovingSphere(s) => s.bounding_box(time0, time1, output_box),
            HittableEnum::XYRect(r) => r.bounding_box(time0, time1, output_box),
            HittableEnum::XZRect(r) => r.bounding_box(time0, time1, output_box),
            HittableEnum::YZRect(r) => r.bounding_box(time0, time1, output_box),
            HittableEnum::Cuboid(c) => c.bounding_box(time0, time1, output_box),
            HittableEnum::Translation(t) => t.bounding_box(time0, time1, output_box),
            HittableEnum::RotateX(r) => r.bounding_box(time0, time1, output_box),
            HittableEnum::RotateY(r) => r.bounding_box(time0, time1, output_box),
            HittableEnum::RotateZ(r) => r.bounding_box(time0, time1, output_box),
            HittableEnum::ConstantMedium(c) => c.bounding_box(time0, time1, output_box),
        }
    }
}

impl Hittable for HittableEnum {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.hit(r, t_min, t_max, rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.bounding_box(time0, time1, output_box)
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
