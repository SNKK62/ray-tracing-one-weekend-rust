use crate::hittable::{BvhNode, Hittable, HittableList, Sphere, XYRect};
use crate::material::{DiffuseLight, Lambertian};
use crate::texture::{NoiseTexture, SolidColor};
use crate::vec3::{Color, Point3};
use std::{cell::RefCell, rc::Rc};

pub fn scene() -> HittableList {
    let mut world: Vec<Rc<dyn Hittable>> = Vec::new();
    let pertext = NoiseTexture::new(4.0);

    let sphere_material = Rc::new(RefCell::new(Lambertian::new(Rc::new(pertext))));
    world.push(Rc::new(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        sphere_material.clone(),
    )));
    world.push(Rc::new(Sphere::new(
        &Point3::new(0.0, 2.0, 0.0),
        2.0,
        sphere_material.clone(),
    )));

    let difflight = Rc::new(RefCell::new(DiffuseLight::new(Rc::new(SolidColor::new(
        Color::new(4.0, 4.0, 4.0),
    )))));
    world.push(Rc::new(Sphere::new(
        &Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.push(Rc::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        difflight.clone(),
    )));

    let bvh = BvhNode::new(&mut world, 0.0, 0.0);

    let mut world = HittableList::new();
    world.add(Rc::new(bvh));
    world
}
