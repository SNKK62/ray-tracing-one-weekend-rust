use crate::hittable::{
    BvhNode, ConstantMedium, Cuboid, HittableEnum, HittableList, MovingSphere, RotateY, Sphere,
    Translation, XZRect,
};
use crate::material::{Dielectric, DiffuseLight, Lambertian, MaterialEnum, Metal};
use crate::texture::{ImageTexture, NoiseTexture, SolidColor, TextureEnum};
use crate::vec3::{Color, Point3, Vec3};
use rand::Rng;

use std::boxed::Box;

pub fn scene() -> HittableEnum {
    let mut box_world: Vec<HittableEnum> = Vec::new();

    let ground = MaterialEnum::Lambertian(Lambertian::new(&TextureEnum::SolidColor(
        SolidColor::new(Color::new(0.5, 0.5, 0.5)),
    )));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rand::thread_rng().gen_range(1.0..101.0);
            let z1 = z0 + w;

            let box_obj = Cuboid::new(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                ground.clone(),
            );
            box_world.push(HittableEnum::Cuboid(box_obj));
        }
    }

    let box_bvh = HittableEnum::BvhNode(Box::new(BvhNode::new(&mut box_world, 0.0, 1.0)));
    let mut hlist = HittableList::new();
    hlist.add(box_bvh);

    let intensity = 7.0;
    let light = MaterialEnum::DiffuseLight(DiffuseLight::new(&TextureEnum::SolidColor(
        SolidColor::new(Color::new(1.0, 1.0, 1.0) * intensity),
    )));
    hlist.add(HittableEnum::XZRect(XZRect::new(
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
        light.clone(),
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = MaterialEnum::Lambertian(Lambertian::new(
        &TextureEnum::SolidColor(SolidColor::new(Color::new(0.7, 0.3, 0.1))),
    ));
    // moving sphere
    hlist.add(HittableEnum::MovingSphere(MovingSphere::new(
        &center1,
        &center2,
        50.0,
        moving_sphere_material.clone(),
        0.0,
        1.0,
    )));
    // dielectric sphere
    hlist.add(HittableEnum::Sphere(Sphere::new(
        &Point3::new(260.0, 150.0, 45.0),
        50.0,
        MaterialEnum::Dielectric(Dielectric::new(1.5)),
    )));
    // metal sphere
    hlist.add(HittableEnum::Sphere(Sphere::new(
        &Point3::new(0.0, 150.0, 145.0),
        50.0,
        MaterialEnum::Metal(Metal::new(&Color::new(0.8, 0.8, 0.9), 10.0)),
    )));

    let boundary = HittableEnum::Sphere(Sphere::new(
        &Point3::new(360.0, 150.0, 145.0),
        70.0,
        MaterialEnum::Dielectric(Dielectric::new(1.5)),
    ));
    hlist.add(boundary.clone());
    hlist.add(HittableEnum::ConstantMedium(Box::new(ConstantMedium::new(
        boundary,
        0.2,
        TextureEnum::SolidColor(SolidColor::new(Color::new(0.2, 0.4, 0.9))),
    ))));

    let boundary = HittableEnum::Sphere(Sphere::new(
        &Point3::new(0.0, 0.0, 0.0),
        5000.0,
        MaterialEnum::Dielectric(Dielectric::new(1.5)),
    ));
    hlist.add(HittableEnum::ConstantMedium(Box::new(ConstantMedium::new(
        boundary,
        0.0001,
        TextureEnum::SolidColor(SolidColor::new(Color::new(1.0, 1.0, 1.0))),
    ))));

    // earth sphere
    let emat = MaterialEnum::Lambertian(Lambertian::new(&TextureEnum::ImageTexture(
        ImageTexture::new("images/earth.png"),
    )));
    hlist.add(HittableEnum::Sphere(Sphere::new(
        &Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));

    // perlin sphere
    let pertext = TextureEnum::NoiseTexture(NoiseTexture::new(0.1));
    hlist.add(HittableEnum::Sphere(Sphere::new(
        &Point3::new(220.0, 280.0, 300.0),
        80.0,
        MaterialEnum::Lambertian(Lambertian::new(&pertext)),
    )));

    // random spheres as box
    let mut box_world: Vec<HittableEnum> = Vec::new();
    let white = MaterialEnum::Lambertian(Lambertian::new(&TextureEnum::SolidColor(
        SolidColor::new(Color::new(0.73, 0.73, 0.73)),
    )));
    let ns = 1000;
    for _ in 0..ns {
        box_world.push(HittableEnum::Sphere(Sphere::new(
            &Point3::rand_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }
    hlist.add(HittableEnum::Translation(Translation::new(
        HittableEnum::RotateY(Box::new(RotateY::new(
            HittableEnum::BvhNode(Box::new(BvhNode::new(&mut box_world, 0.0, 1.0))),
            15.0,
        ))),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    HittableEnum::HittableList(Box::new(hlist))
}
