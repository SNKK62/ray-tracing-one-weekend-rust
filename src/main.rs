use image::RgbImage;
use rand::Rng;
use ray_tracer_rs::{camera, hittable::HittableEnum, progress, scenes, vec3};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Instant;

fn main() {
    // let aspect_ratio = 16.0 / 9.0;
    let aspect_ratio = 1.0;
    // let width = 384;
    let width = 512;
    let height = (width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 50;
    let max_depth = 100;

    // NOTE: The following code is for the camera position for random scenes
    // let lookfrom = vec3::Point3::new(13.0, 2.0, 3.0);
    // let lookat = vec3::Point3::new(0.0, 0.0, 0.0);
    // NOTE: The following code is for the camera position for the cornell box scene
    let lookfrom = vec3::Point3::new(278.0, 278.0, -800.0);
    let lookat = vec3::Point3::new(278.0, 278.0, 0.0);
    let vup = vec3::Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let background = vec3::Color::zero();

    let pb = Arc::new(RwLock::new(progress::ProgressBar::new(width * height)));
    let world: Arc<HittableEnum> = Arc::new(scenes::final_scene::scene());
    let cam = Arc::new(camera::Camera::new(
        lookfrom,
        lookat,
        vup,
        40.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    ));
    let buffer = Arc::new(RwLock::new(vec![0; width * height * 3]));
    let handles: Vec<_> = (0..height)
        .rev()
        .map(|j| {
            let buffer = Arc::clone(&buffer);
            let world = Arc::clone(&world);
            let cam = Arc::clone(&cam);
            let pb = Arc::clone(&pb);

            thread::spawn(move || {
                let mut rng = rand::thread_rng();
                for i in 0..width {
                    let mut pixel_color = vec3::Color::zero();
                    for _ in 0..samples_per_pixel {
                        let u = (i as f64 + rng.gen_range(0.0..1.0)) / (width - 1) as f64;
                        let v = (j as f64 + rng.gen_range(0.0..1.0)) / (height - 1) as f64;
                        let r = cam.get_ray(u, v);
                        pixel_color += r.color(&background, &world, max_depth);
                    }
                    let mut buf = buffer.write().unwrap();
                    let (r, g, b) = pixel_color.get_color(samples_per_pixel);
                    buf[(height - j - 1) * width * 3 + i * 3] = r;
                    buf[(height - j - 1) * width * 3 + i * 3 + 1] = g;
                    buf[(height - j - 1) * width * 3 + i * 3 + 2] = b;
                    let mut pb = pb.write().unwrap();
                    pb.update();
                }
            })
        })
        .collect();

    let start = Instant::now();
    for handle in handles {
        handle.join().unwrap();
    }
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    let image_buffer = Arc::try_unwrap(buffer).unwrap().into_inner().unwrap();
    let img = RgbImage::from_raw(width as u32, height as u32, image_buffer)
        .expect("incorrect image buffer size");

    img.save("temp.png").expect("failed to save image");
}
