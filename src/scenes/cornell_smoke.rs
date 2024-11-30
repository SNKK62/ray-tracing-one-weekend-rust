use crate::hittable::{
    BvhNode, ConstantMedium, Cuboid, HittableEnum, HittableList, RotateY, Translation, XYRect,
    XZRect, YZRect,
};
use crate::material::{DiffuseLight, Lambertian, MaterialEnum};
use crate::texture::{SolidColor, TextureEnum};
use crate::vec3::Color;

pub fn scene() -> HittableEnum {
    let mut world: Vec<HittableEnum> = Vec::new();

    let red = MaterialEnum::Lambertian(Lambertian::new(&TextureEnum::SolidColor(SolidColor::new(
        Color::new(0.65, 0.05, 0.05),
    ))));
    let white = MaterialEnum::Lambertian(Lambertian::new(&TextureEnum::SolidColor(
        SolidColor::new(Color::new(0.73, 0.73, 0.73)),
    )));
    let green = MaterialEnum::Lambertian(Lambertian::new(&TextureEnum::SolidColor(
        SolidColor::new(Color::new(0.12, 0.45, 0.15)),
    )));
    let light = MaterialEnum::DiffuseLight(DiffuseLight::new(&TextureEnum::SolidColor(
        SolidColor::new(Color::new(15.0, 15.0, 15.0)),
    )));

    world.push(HittableEnum::YZRect(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green.clone(),
    )));
    world.push(HittableEnum::YZRect(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red.clone(),
    )));
    world.push(HittableEnum::XZRect(XZRect::new(
        113.0,
        443.0,
        127.0,
        432.0,
        554.0,
        light.clone(),
    )));
    world.push(HittableEnum::XZRect(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.push(HittableEnum::XZRect(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.push(HittableEnum::XYRect(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    // Box
    let cuboid = HittableEnum::Cuboid(Cuboid::new(
        &crate::vec3::Point3::new(0.0, 0.0, 0.0),
        &crate::vec3::Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let cuboid = HittableEnum::RotateY(Box::new(RotateY::new(cuboid, 15.0)));
    let cuboid = HittableEnum::Translation(Translation::new(
        cuboid,
        crate::vec3::Vec3::new(265.0, 0.0, 295.0),
    ));
    world.push(HittableEnum::ConstantMedium(Box::new(ConstantMedium::new(
        cuboid,
        0.01,
        TextureEnum::SolidColor(SolidColor::new(Color::new(0.0, 0.0, 0.0))),
    ))));

    let cuboid = HittableEnum::Cuboid(Cuboid::new(
        &crate::vec3::Point3::new(0.0, 0.0, 0.0),
        &crate::vec3::Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let cuboid = HittableEnum::RotateY(Box::new(RotateY::new(cuboid, -18.0)));
    let cuboid = HittableEnum::Translation(Translation::new(
        cuboid,
        crate::vec3::Vec3::new(130.0, 0.0, 65.0),
    ));
    world.push(HittableEnum::ConstantMedium(Box::new(ConstantMedium::new(
        cuboid,
        0.01,
        TextureEnum::SolidColor(SolidColor::new(Color::new(1.0, 1.0, 1.0))),
    ))));

    let bvh = HittableEnum::BvhNode(Box::new(BvhNode::new(&mut world, 0.0, 0.0)));

    let mut world = HittableList::new();
    world.add(bvh);
    HittableEnum::HittableList(Box::new(world))
}
