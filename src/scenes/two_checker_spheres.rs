use crate::hittable::{BvhNode, HittableEnum, HittableList, Sphere};
use crate::material::{Lambertian, MaterialEnum};
use crate::texture::{Checker, SolidColor, TextureEnum};
use crate::vec3::{Color, Point3};

pub fn scene() -> HittableEnum {
    let mut world: Vec<HittableEnum> = Vec::new();
    let checker = TextureEnum::Checker(Checker::new(
        TextureEnum::SolidColor(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
        TextureEnum::SolidColor(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
    ));

    let sphere_material = MaterialEnum::Lambertian(Lambertian::new(&checker));
    world.push(HittableEnum::Sphere(Sphere::new(
        &Point3::new(0.0, -10.0, 0.0),
        10.0,
        sphere_material.clone(),
    )));
    world.push(HittableEnum::Sphere(Sphere::new(
        &Point3::new(0.0, 10.0, 0.0),
        10.0,
        sphere_material.clone(),
    )));

    let bvh = HittableEnum::BvhNode(Box::new(BvhNode::new(&mut world, 0.0, 0.0)));

    let mut world = HittableList::new();
    world.add(bvh);
    HittableEnum::HittableList(Box::new(world))
}
