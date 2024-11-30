use crate::hittable::{BvhNode, HittableEnum, HittableList, Sphere};
use crate::material::{Lambertian, MaterialEnum};
use crate::texture::{NoiseTexture, TextureEnum};
use crate::vec3::Point3;

pub fn scene() -> HittableEnum {
    let mut world: Vec<HittableEnum> = Vec::new();
    let pertext = TextureEnum::NoiseTexture(NoiseTexture::new(3.0));

    let sphere_material = MaterialEnum::Lambertian(Lambertian::new(&pertext));
    world.push(HittableEnum::Sphere(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        sphere_material.clone(),
    )));
    world.push(HittableEnum::Sphere(Sphere::new(
        &Point3::new(0.0, 2.0, 0.0),
        2.0,
        sphere_material.clone(),
    )));

    let bvh = HittableEnum::BvhNode(Box::new(BvhNode::new(&mut world, 0.0, 0.0)));

    let mut world = HittableList::new();
    world.add(bvh);
    HittableEnum::HittableList(Box::new(world))
}
