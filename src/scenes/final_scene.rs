use crate::hittable::{
    BvhNode, ConstantMedium, Cuboid, Hittable, HittableList, MovingSphere, RotateY, Sphere,
    Translation, XZRect,
};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::texture::{ImageTexture, NoiseTexture, SolidColor};
use crate::vec3::{Color, Point3, Vec3};
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

pub fn scene() -> HittableList {
    let mut box_world: Vec<Rc<dyn Hittable>> = Vec::new();

    let ground = Lambertian::new(Rc::new(SolidColor::new(Color::new(0.5, 0.5, 0.5))));
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
                Rc::new(RefCell::new(ground.clone())),
            );
            box_world.push(Rc::new(box_obj));
        }
    }

    let box_bvh = BvhNode::new(&mut box_world, 0.0, 1.0);
    let mut hlist = HittableList::new();
    hlist.add(Rc::new(box_bvh));

    let intensity = 7.0;
    let light = DiffuseLight::new(Rc::new(SolidColor::new(
        Color::new(1.0, 1.0, 1.0) * intensity,
    )));
    hlist.add(Rc::new(XZRect::new(
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
        Rc::new(RefCell::new(light)),
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material =
        Lambertian::new(Rc::new(SolidColor::new(Color::new(0.7, 0.3, 0.1))));
    // moving sphere
    hlist.add(Rc::new(MovingSphere::new(
        &center1,
        &center2,
        50.0,
        Rc::new(RefCell::new(moving_sphere_material)),
        0.0,
        1.0,
    )));
    // dielectric sphere
    hlist.add(Rc::new(Sphere::new(
        &Point3::new(260.0, 150.0, 45.0),
        50.0,
        Rc::new(RefCell::new(Dielectric::new(1.5))),
    )));
    // metal sphere
    hlist.add(Rc::new(Sphere::new(
        &Point3::new(0.0, 150.0, 145.0),
        50.0,
        Rc::new(RefCell::new(Metal::new(&Color::new(0.8, 0.8, 0.9), 10.0))),
    )));

    let boundary = Sphere::new(
        &Point3::new(360.0, 150.0, 145.0),
        70.0,
        Rc::new(RefCell::new(Dielectric::new(1.5))),
    );
    hlist.add(Rc::new(boundary.clone()));
    hlist.add(Rc::new(ConstantMedium::new(
        Rc::new(boundary),
        0.2,
        Rc::new(SolidColor::new(Color::new(0.2, 0.4, 0.9))),
    )));

    let boundary = Sphere::new(
        &Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Rc::new(RefCell::new(Dielectric::new(1.5))),
    );
    hlist.add(Rc::new(ConstantMedium::new(
        Rc::new(boundary),
        0.0001,
        Rc::new(SolidColor::new(Color::new(1.0, 1.0, 1.0))),
    )));

    // earth sphere
    let emat = Lambertian::new(Rc::new(ImageTexture::new("images/earth.png")));
    hlist.add(Rc::new(Sphere::new(
        &Point3::new(400.0, 200.0, 400.0),
        100.0,
        Rc::new(RefCell::new(emat)),
    )));

    // perlin sphere
    let pertext = NoiseTexture::new(0.1);
    hlist.add(Rc::new(Sphere::new(
        &Point3::new(220.0, 280.0, 300.0),
        80.0,
        Rc::new(RefCell::new(Lambertian::new(Rc::new(pertext)))),
    )));

    // random spheres as box
    let mut box_world: Vec<Rc<dyn Hittable>> = Vec::new();
    let white = Lambertian::new(Rc::new(SolidColor::new(Color::new(0.73, 0.73, 0.73))));
    let ns = 1000;
    for _ in 0..ns {
        box_world.push(Rc::new(Sphere::new(
            &Point3::rand_range(0.0, 165.0),
            10.0,
            Rc::new(RefCell::new(white.clone())),
        )));
    }
    hlist.add(Rc::new(Translation::new(
        Rc::new(RotateY::new(
            Rc::new(BvhNode::new(&mut box_world, 0.0, 1.0)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    hlist
}
