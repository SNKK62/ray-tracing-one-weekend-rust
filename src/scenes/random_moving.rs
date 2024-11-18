use crate::hittable::{hittable_list::HittableList, moving_sphere::MovingSphere, sphere::Sphere};
use crate::material;
use crate::texture::SolidColor;
use crate::vec3::{Color, Point3};
use rand::Rng;
use std::{cell::RefCell, rc::Rc};

pub fn scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Rc::new(RefCell::new(material::Lambertian::new(Rc::new(
        SolidColor::new(Color::new(0.5, 0.5, 0.5)),
    ))));
    world.add(Rc::new(Sphere::new(
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
                let sphere_material: Rc<RefCell<dyn material::Material>>;
                if choose_mat < 0.7 {
                    // diffuse
                    let albedo = Color::rand() * Color::rand();
                    sphere_material = Rc::new(RefCell::new(material::Lambertian::new(Rc::new(
                        SolidColor::new(albedo),
                    ))));
                    let center2 =
                        center + Point3::new(0.0, rand::thread_rng().gen_range(0.0..0.5), 0.0);
                    world.add(Rc::new(MovingSphere::new(
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
                    sphere_material = Rc::new(RefCell::new(material::Metal::new(&albedo, fuzz)));
                    world.add(Rc::new(Sphere::new(&center, radius, sphere_material)));
                } else {
                    // glass
                    sphere_material = Rc::new(RefCell::new(material::Dielectric::new(1.5)));
                    world.add(Rc::new(Sphere::new(&center, radius, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(RefCell::new(material::Dielectric::new(1.5)));
    world.add(Rc::new(Sphere::new(
        &Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Rc::new(RefCell::new(material::Lambertian::new(Rc::new(
        SolidColor::new(Color::new(0.4, 0.2, 0.1)),
    ))));
    world.add(Rc::new(Sphere::new(
        &Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Rc::new(RefCell::new(material::Metal::new(
        &Color::new(0.7, 0.6, 0.5),
        0.0,
    )));
    world.add(Rc::new(Sphere::new(
        &Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    world
}
