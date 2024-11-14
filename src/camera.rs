use crate::degrees_to_radians;
use crate::ray::Ray;
use crate::vec3;

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
