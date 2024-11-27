use crate::hittable::{BvhNode, Hittable, HittableList, Sphere};
use crate::material::Lambertian;
use crate::texture::ImageTexture;
use crate::vec3::Point3;
use std::sync::{Arc, RwLock};

pub fn scene() -> HittableList {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();
    let texture = ImageTexture::new("images/earth.png");

    let sphere_material = Arc::new(RwLock::new(Lambertian::new(Arc::new(texture))));
    world.push(Arc::new(Sphere::new(
        &Point3::new(0.0, 0.0, 0.0),
        2.0,
        sphere_material.clone(),
    )));

    let bvh = BvhNode::new(&mut world, 0.0, 0.0);

    let mut world = HittableList::new();
    world.add(Arc::new(bvh));
    world
}
