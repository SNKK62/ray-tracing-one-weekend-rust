use super::{HitRecord, Hittable, AABB};
use crate::material::{Isotropic, Material};
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;
use rand::Rng;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<RwLock<dyn Material>>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, tex: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(RwLock::new(Isotropic::new(tex))),
            neg_inv_density: -1.0 / density,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let enable_debug = false;
        let debugging = enable_debug && rand::random::<f64>() < 0.00001;

        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if (!self
            .boundary
            .hit(r, f64::NEG_INFINITY, f64::INFINITY, &mut rec1))
            || (!self
                .boundary
                .hit(r, rec1.t + 0.0001, f64::INFINITY, &mut rec2))
        {
            return false;
        }

        if debugging {
            eprintln!("t0 = {}\nt1 = {}", rec1.t, rec2.t);
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction.len();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance =
            self.neg_inv_density * (rand::thread_rng().gen_range(0.0..1.0) as f64).ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        if debugging {
            eprintln!("hit_distance = {}", hit_distance);
            eprintln!("rec.t = {}", rec.t);
            eprintln!("rec.p = {}", rec.p);
        }

        rec.normal = Vec3::new(1.0, 0.0, 0.0); // this is arbitrary
        rec.front_face = true;
        rec.material = Some(self.phase_function.clone());

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
}
