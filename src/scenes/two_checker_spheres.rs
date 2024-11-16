use crate::hittable::{BvhNode, Hittable, HittableList, Sphere};
use crate::material::Lambertian;
use crate::texture::{Checker, SolidColor};
use crate::vec3::{Color, Point3};
use std::{boxed::Box, cell::RefCell, rc::Rc};

pub fn scene() -> HittableList {
    let mut world: Vec<Rc<dyn Hittable>> = Vec::new();
    let checker = Checker::new(
        Box::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
        Box::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
    );

    let sphere_material = Rc::new(RefCell::new(Lambertian::new(Box::new(checker))));
    world.push(Rc::new(Sphere::new(
        &Point3::new(0.0, -10.0, 0.0),
        10.0,
        sphere_material.clone(),
    )));
    world.push(Rc::new(Sphere::new(
        &Point3::new(0.0, 10.0, 0.0),
        10.0,
        sphere_material.clone(),
    )));

    let bvh = BvhNode::new(&mut world, 0.0, 0.0);

    let mut world = HittableList::new();
    world.add(Rc::new(bvh));
    world
}
