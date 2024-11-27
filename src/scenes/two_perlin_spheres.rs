use crate::hittable::{BvhNode, Hittable, HittableList, Sphere};
use crate::material::Lambertian;
use crate::texture::NoiseTexture;
use crate::vec3::Point3;
use std::sync::{Arc, Mutex};

pub fn scene() -> HittableList {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();
    let pertext = NoiseTexture::new(3.0);

    let sphere_material = Arc::new(Mutex::new(Lambertian::new(Arc::new(pertext))));
    world.push(Arc::new(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        sphere_material.clone(),
    )));
    world.push(Arc::new(Sphere::new(
        &Point3::new(0.0, 2.0, 0.0),
        2.0,
        sphere_material.clone(),
    )));

    let bvh = BvhNode::new(&mut world, 0.0, 0.0);

    let mut world = HittableList::new();
    world.add(Arc::new(bvh));
    world
}
