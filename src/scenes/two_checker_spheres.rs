use crate::hittable::{BvhNode, Hittable, HittableList, Sphere};
use crate::material::Lambertian;
use crate::texture::{Checker, SolidColor};
use crate::vec3::{Color, Point3};
use std::sync::{Arc, Mutex};

pub fn scene() -> HittableList {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();
    let checker = Checker::new(
        Arc::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
        Arc::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
    );

    let sphere_material = Arc::new(Mutex::new(Lambertian::new(Arc::new(checker))));
    world.push(Arc::new(Sphere::new(
        &Point3::new(0.0, -10.0, 0.0),
        10.0,
        sphere_material.clone(),
    )));
    world.push(Arc::new(Sphere::new(
        &Point3::new(0.0, 10.0, 0.0),
        10.0,
        sphere_material.clone(),
    )));

    let bvh = BvhNode::new(&mut world, 0.0, 0.0);

    let mut world = HittableList::new();
    world.add(Arc::new(bvh));
    world
}
