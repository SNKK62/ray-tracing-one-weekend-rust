pub mod camera;
pub mod hittable;
pub mod material;
pub mod progress;
pub mod ray;
pub mod scene;
pub mod vec3;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
