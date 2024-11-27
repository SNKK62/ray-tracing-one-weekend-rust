use image::RgbImage;
use rand::Rng;
use ray_tracer_rs::{camera, hittable::Hittable, progress, scenes, vec3};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn main() {
    // let aspect_ratio = 16.0 / 9.0;
    let aspect_ratio = 1.0;
    // let width = 384;
    let width = 512;
    let height = (width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 100;

    print!("P3\n{} {}\n255\n", width, height);

    // let world = scenes::cornell_box::scene();

    // NOTE: The following code is for the camera position for random scenes
    // let lookfrom = vec3::Point3::new(13.0, 2.0, 3.0);
    // let lookat = vec3::Point3::new(0.0, 0.0, 0.0);
    // NOTE: The following code is for the camera position for the cornell box scene
    let lookfrom = vec3::Point3::new(278.0, 278.0, -800.0);
    let lookat = vec3::Point3::new(278.0, 278.0, 0.0);
    let vup = vec3::Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    // let cam = camera::Camera::new(
    //     lookfrom,
    //     lookat,
    //     vup,
    //     40.0,
    //     aspect_ratio,
    //     aperture,
    //     dist_to_focus,
    //     0.0,
    //     1.0,
    // );
    let background = vec3::Color::zero();

    // let mut pb = progress::ProgressBar::new((width * height) as usize);
    // for j in (0..height).rev() {
    //     for i in 0..width {
    //         let mut pixel_color = vec3::Color::zero();
    //         for _ in 0..samples_per_pixel {
    //             let u = (i as f64 + rand::thread_rng().gen_range(0.0..1.0)) / (width - 1) as f64;
    //             let v = (j as f64 + rand::thread_rng().gen_range(0.0..1.0)) / (height - 1) as f64;
    //             let r = cam.get_ray(u, v);
    //             pixel_color += r.color(&background, &world, max_depth);
    //         }
    //         pb.update();
    //         pixel_color.write(samples_per_pixel);
    //     }
    // }
    //
    // eprintln!("\n\nDone.\n"); // indicate completion
    let pb = Arc::new(Mutex::new(progress::ProgressBar::new(width * height)));
    let world: Arc<dyn Hittable> = Arc::new(scenes::cornell_box::scene());
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
    let buffer = Arc::new(Mutex::new(vec![0; width * height * 3]));
    let handles: Vec<_> = (0..height)
        .rev()
        .map(|j| {
            let buffer = Arc::clone(&buffer);
            let world = Arc::clone(&world);
            let cam = Arc::clone(&cam); // 各スレッドに`cam`の参照を渡す
            let pb = Arc::clone(&pb);

            thread::spawn(move || {
                let mut rng = rand::thread_rng();
                for i in 0..width {
                    let mut pixel_color = vec3::Color::zero();
                    for _ in 0..samples_per_pixel {
                        let u = (i as f64 + rng.gen_range(0.0..1.0)) / (width - 1) as f64;
                        let v = (j as f64 + rng.gen_range(0.0..1.0)) / (height - 1) as f64;

                        // `cam`を参照として使用
                        let r = cam.get_ray(u, v);
                        pixel_color += r.color(&background, &*world, max_depth);
                    }
                    let mut buf = buffer.lock().unwrap();
                    let (r, g, b) = pixel_color.get_color(samples_per_pixel);
                    buf[j * width * 3 + i * 3] = r;
                    buf[j * width * 3 + i * 3 + 1] = g;
                    buf[j * width * 3 + i * 3 + 2] = b;
                    let mut pb = pb.lock().unwrap();
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
