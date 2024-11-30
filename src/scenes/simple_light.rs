use crate::hittable::{BvhNode, HittableEnum, HittableList, Sphere, XYRect};
use crate::material::{DiffuseLight, Lambertian, MaterialEnum};
use crate::texture::{NoiseTexture, SolidColor, TextureEnum};
use crate::vec3::{Color, Point3};

pub fn scene() -> HittableEnum {
    let mut world: Vec<HittableEnum> = Vec::new();
    let pertext = TextureEnum::NoiseTexture(NoiseTexture::new(4.0));

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

    let difflight = MaterialEnum::DiffuseLight(DiffuseLight::new(&TextureEnum::SolidColor(
        SolidColor::new(Color::new(4.0, 4.0, 4.0)),
    )));
    world.push(HittableEnum::Sphere(Sphere::new(
        &Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.push(HittableEnum::XYRect(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        difflight.clone(),
    )));

    let bvh = HittableEnum::BvhNode(Box::new(BvhNode::new(&mut world, 0.0, 0.0)));

    let mut world = HittableList::new();
    world.add(bvh);
    HittableEnum::HittableList(Box::new(world))
}
