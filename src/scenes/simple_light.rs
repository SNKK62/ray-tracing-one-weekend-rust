use crate::hittable::{BvhNode, Hittable, HittableList, Sphere, XYRect};
use crate::material::{DiffuseLight, Lambertian};
use crate::texture::{NoiseTexture, SolidColor};
use crate::vec3::{Color, Point3};
use std::sync::{Arc, RwLock};

pub fn scene() -> HittableList {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();
    let pertext = NoiseTexture::new(4.0);

    let sphere_material = Arc::new(RwLock::new(Lambertian::new(Arc::new(pertext))));
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

    let difflight = Arc::new(RwLock::new(DiffuseLight::new(Arc::new(SolidColor::new(
        Color::new(4.0, 4.0, 4.0),
    )))));
    world.push(Arc::new(Sphere::new(
        &Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.push(Arc::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        difflight.clone(),
    )));

    let bvh = BvhNode::new(&mut world, 0.0, 0.0);

    let mut world = HittableList::new();
    world.add(Arc::new(bvh));
    world
}
