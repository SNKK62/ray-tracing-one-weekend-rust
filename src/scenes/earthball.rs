use crate::hittable::{BvhNode, HittableEnum, HittableList, Sphere};
use crate::material::{Lambertian, MaterialEnum};
use crate::texture::{ImageTexture, TextureEnum};
use crate::vec3::Point3;
use std::boxed::Box;

pub fn scene() -> HittableEnum {
    let mut world: Vec<HittableEnum> = Vec::new();
    let texture = TextureEnum::ImageTexture(ImageTexture::new("images/earth.png"));

    let sphere_material = MaterialEnum::Lambertian(Lambertian::new(&texture));
    world.push(HittableEnum::Sphere(Sphere::new(
        &Point3::new(0.0, 0.0, 0.0),
        2.0,
        sphere_material.clone(),
    )));

    let bvh = HittableEnum::BvhNode(Box::new(BvhNode::new(&mut world, 0.0, 0.0)));

    let mut world = HittableList::new();
    world.add(bvh);
    HittableEnum::HittableList(Box::new(world))
}
