mod hittable;
mod progress;
mod ray;
mod vec3;
use ray::Ray;
use vec3::{Point3, Vec3};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 384;
    let height = (width as f64 / aspect_ratio) as i64;

    print!("P3\n{} {}\n255\n", width, height);

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut world = hittable::HittableList::new();
    world.add(Box::new(hittable::Sphere::new(
        &Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(hittable::Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let mut pb = progress::ProgressBar::new(width * height as usize);
    for j in (0..height).rev() {
        for i in 0..width {
            pb.update();

            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (height - 1) as f64;

            let r = Ray::new(
                &origin,
                &(lower_left_corner + u * horizontal + v * vertical - origin),
            );

            let color = r.color(&world);
            color.write();
        }
    }

    eprintln!("\nDone.\n"); // indicate completion
}
