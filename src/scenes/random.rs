use crate::hittable::{BvhNode, HittableEnum, HittableList, Sphere};
use crate::material::{Dielectric, Lambertian, MaterialEnum, Metal};
use crate::texture::{Checker, SolidColor, TextureEnum};
use crate::vec3::{Color, Point3};
use rand::Rng;

pub fn scene() -> HittableEnum {
    let mut world: Vec<HittableEnum> = Vec::new();
    let checker = TextureEnum::Checker(Checker::new(
        TextureEnum::SolidColor(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
        TextureEnum::SolidColor(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
    ));
    let ground_material = MaterialEnum::Lambertian(Lambertian::new(&checker));
    world.push(HittableEnum::Sphere(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material.clone(),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::thread_rng().gen_range(0.0..1.0);
            let center = Point3::new(
                (a as f64) + 0.9 * rand::thread_rng().gen_range(0.0..1.0),
                0.2,
                (b as f64) + 0.9 * rand::thread_rng().gen_range(0.0..1.0),
            );

            let radius = 0.2;
            if (center - Point3::new(4.0, radius, 0.0)).len() > 0.9 {
                let sphere_material: MaterialEnum;
                if choose_mat < 0.7 {
                    // diffuse
                    let albedo = Color::rand() * Color::rand();
                    sphere_material = MaterialEnum::Lambertian(Lambertian::new(
                        &TextureEnum::SolidColor(SolidColor::new(albedo)),
                    ));
                    world.push(HittableEnum::Sphere(Sphere::new(
                        &center,
                        radius,
                        sphere_material,
                    )));
                } else if choose_mat < 0.85 {
                    // metal
                    let albedo = Color::rand_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                    sphere_material = MaterialEnum::Metal(Metal::new(&albedo, fuzz));
                    world.push(HittableEnum::Sphere(Sphere::new(
                        &center,
                        radius,
                        sphere_material,
                    )));
                } else {
                    // glass
                    sphere_material = MaterialEnum::Dielectric(Dielectric::new(1.5));
                    world.push(HittableEnum::Sphere(Sphere::new(
                        &center,
                        radius,
                        sphere_material,
                    )));
                }
            }
        }
    }

    let material1 = MaterialEnum::Dielectric(Dielectric::new(1.5));
    world.push(HittableEnum::Sphere(Sphere::new(
        &Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = MaterialEnum::Lambertian(Lambertian::new(&TextureEnum::SolidColor(
        SolidColor::new(Color::new(0.4, 0.2, 0.1)),
    )));
    world.push(HittableEnum::Sphere(Sphere::new(
        &Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = MaterialEnum::Metal(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.push(HittableEnum::Sphere(Sphere::new(
        &Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    let bvh = HittableEnum::BvhNode(Box::new(BvhNode::new(&mut world, 0.0, 1.0)));
    let mut world = HittableList::new();
    world.add(bvh);
    HittableEnum::HittableList(Box::new(world))
}
