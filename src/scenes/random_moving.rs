use crate::hittable::{
    hittable_list::HittableList, moving_sphere::MovingSphere, sphere::Sphere, HittableEnum,
};
use crate::material::{Dielectric, Lambertian, MaterialEnum, Metal};
use crate::texture::{SolidColor, TextureEnum};
use crate::vec3::{Color, Point3};
use rand::Rng;

pub fn scene() -> HittableEnum {
    let mut world = HittableList::new();
    let ground_material = MaterialEnum::Lambertian(Lambertian::new(&TextureEnum::SolidColor(
        SolidColor::new(Color::new(0.5, 0.5, 0.5)),
    )));
    world.add(HittableEnum::Sphere(Sphere::new(
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
                    let center2 =
                        center + Point3::new(0.0, rand::thread_rng().gen_range(0.0..0.5), 0.0);
                    world.add(HittableEnum::MovingSphere(MovingSphere::new(
                        &center,
                        &center2,
                        radius,
                        sphere_material,
                        0.0,
                        1.0,
                    )));
                } else if choose_mat < 0.85 {
                    // metal
                    let albedo = Color::rand_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                    sphere_material = MaterialEnum::Metal(Metal::new(&albedo, fuzz));
                    world.add(HittableEnum::Sphere(Sphere::new(
                        &center,
                        radius,
                        sphere_material,
                    )));
                } else {
                    // glass
                    sphere_material = MaterialEnum::Dielectric(Dielectric::new(1.5));
                    world.add(HittableEnum::Sphere(Sphere::new(
                        &center,
                        radius,
                        sphere_material,
                    )));
                }
            }
        }
    }

    let material1 = MaterialEnum::Dielectric(Dielectric::new(1.5));
    world.add(HittableEnum::Sphere(Sphere::new(
        &Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = MaterialEnum::Lambertian(Lambertian::new(&TextureEnum::SolidColor(
        SolidColor::new(Color::new(0.4, 0.2, 0.1)),
    )));
    world.add(HittableEnum::Sphere(Sphere::new(
        &Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = MaterialEnum::Metal(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(HittableEnum::Sphere(Sphere::new(
        &Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    HittableEnum::HittableList(Box::new(world))
}
