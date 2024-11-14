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
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: vec3::Vec3,
        lookat: vec3::Vec3,
        vup: vec3::Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * vec3::Vec3::rand_in_unit_disk();
        let offset = vec3::Vec3::new(u * rd.x(), v * rd.y(), 0.0);
        Ray::new(
            &(self.origin + offset),
            &(self.lower_left_corner + u * self.horizontal + v * self.vertical
                - (self.origin + offset)),
        )
    }
}

fn random_scene() -> hittable::HittableList {
    let mut world = hittable::HittableList::new();
    let ground_material = Rc::new(RefCell::new(material::Lambertian::new(&vec3::Color::new(
        0.5, 0.5, 0.5,
    ))));
    world.add(Box::new(hittable::Sphere::new(
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
                    let albedo = vec3::Color::rand() * vec3::Color::rand();
                    sphere_material = Rc::new(RefCell::new(material::Lambertian::new(&albedo)));
                    world.add(Box::new(hittable::Sphere::new(
                        &center,
                        radius,
                        sphere_material,
                    )));
                } else if choose_mat < 0.85 {
                    // metal
                    let albedo = vec3::Color::rand_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                    sphere_material = Rc::new(RefCell::new(material::Metal::new(&albedo, fuzz)));
                    world.add(Box::new(hittable::Sphere::new(
                        &center,
                        radius,
                        sphere_material,
                    )));
                } else {
                    // glass
                    sphere_material = Rc::new(RefCell::new(material::Dielectric::new(1.5)));
                    world.add(Box::new(hittable::Sphere::new(
                        &center,
                        radius,
                        sphere_material,
                    )));
                }
            }
        }
    }

    let material1 = Rc::new(RefCell::new(material::Dielectric::new(1.5)));
    world.add(Box::new(hittable::Sphere::new(
        &Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Rc::new(RefCell::new(material::Lambertian::new(&vec3::Color::new(
        0.4, 0.2, 0.1,
    ))));
    world.add(Box::new(hittable::Sphere::new(
        &Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Rc::new(RefCell::new(material::Metal::new(
        &vec3::Color::new(0.7, 0.6, 0.5),
        0.0,
    )));
    world.add(Box::new(hittable::Sphere::new(
        &Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    world
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 512;
    let height = (width as f64 / aspect_ratio) as i64;
    let samples_per_pixel = 500;
    let max_depth = 100;

    print!("P3\n{} {}\n255\n", width, height);

    let world = random_scene();

    let lookfrom = vec3::Point3::new(13.0, 2.0, 3.0);
    let lookat = vec3::Point3::new(0.0, 0.0, 0.0);
    let vup = vec3::Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
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
