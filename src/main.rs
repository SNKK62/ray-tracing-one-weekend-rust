use std::{cell::RefCell, rc::Rc};

use rand::Rng;
mod hittable;
mod material;
mod progress;
mod ray;
mod utils;
mod vec3;
use ray::Ray;
use utils::degrees_to_radians;
use vec3::Point3;

pub struct Camera {
    origin: vec3::Point3,
    lower_left_corner: vec3::Point3,
    horizontal: vec3::Vec3,
    vertical: vec3::Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: vec3::Vec3,
        lookat: vec3::Vec3,
        vup: vec3::Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &self.origin,
            &(self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin),
        )
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 384;
    let height = (width as f64 / aspect_ratio) as i64;
    let samples_per_pixel = 100;
    let max_depth = 100;

    print!("P3\n{} {}\n255\n", width, height);

    let mut world = hittable::HittableList::new();
    world.add(Box::new(hittable::Sphere::new(
        &Point3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::new(RefCell::new(material::Lambertian::new(&vec3::Color::new(
            0.1, 0.2, 0.5,
        )))),
    )));
    world.add(Box::new(hittable::Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(RefCell::new(material::Lambertian::new(&vec3::Color::new(
            0.8, 0.8, 0.0,
        )))),
    )));
    world.add(Box::new(hittable::Sphere::new(
        &Point3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(RefCell::new(material::Metal::new(
            &vec3::Color::new(0.8, 0.6, 0.2),
            0.0,
        ))),
    )));
    world.add(Box::new(hittable::Sphere::new(
        &Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(RefCell::new(material::Dielectric::new(1.5))),
    )));
    world.add(Box::new(hittable::Sphere::new(
        &Point3::new(-1.0, 0.0, -1.0),
        -0.45, // negative radius for hollow sphere
        Rc::new(RefCell::new(material::Dielectric::new(1.5))),
    )));
    // let r = f64::cos(std::f64::consts::PI / 4.0);
    // world.add(Box::new(hittable::Sphere::new(
    //     &Point3::new(-r, 0.0, -1.0),
    //     r,
    //     Rc::new(RefCell::new(material::Lambertian::new(&vec3::Color::new(
    //         0.0, 0.0, 1.0,
    //     )))),
    // )));
    // world.add(Box::new(hittable::Sphere::new(
    //     &Point3::new(r, 0.0, -1.0),
    //     r,
    //     Rc::new(RefCell::new(material::Lambertian::new(&vec3::Color::new(
    //         1.0, 0.0, 0.0,
    //     )))),
    // )));

    let cam = Camera::new(
        vec3::Point3::new(-2.0, 2.0, 1.0),
        vec3::Point3::new(0.0, 0.0, -1.0),
        vec3::Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
    );

    let mut pb = progress::ProgressBar::new((width * height * samples_per_pixel) as usize);
    for j in (0..height).rev() {
        for i in 0..width {
            let mut pixel_color = vec3::Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand::thread_rng().gen_range(0.0..1.0)) / (width - 1) as f64;
                let v = (j as f64 + rand::thread_rng().gen_range(0.0..1.0)) / (height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += r.color(&world, max_depth);
                pb.update();
            }
            pixel_color.write(samples_per_pixel);
        }
    }

    eprintln!("\n\nDone.\n"); // indicate completion
}
