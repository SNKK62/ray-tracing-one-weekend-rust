use crate::hittable::{BvhNode, Hittable, HittableList, Sphere};
use crate::material::Lambertian;
use crate::texture::ImageTexture;
use crate::vec3::Point3;
use std::{cell::RefCell, rc::Rc};

pub fn scene() -> HittableList {
    let mut world: Vec<Rc<dyn Hittable>> = Vec::new();
    let texture = ImageTexture::new("images/earth.png");

    let sphere_material = Rc::new(RefCell::new(Lambertian::new(Rc::new(texture))));
    world.push(Rc::new(Sphere::new(
        &Point3::new(0.0, 0.0, 0.0),
        2.0,
        sphere_material.clone(),
    )));

    let bvh = BvhNode::new(&mut world, 0.0, 0.0);

    let mut world = HittableList::new();
    world.add(Rc::new(bvh));
    world
}
