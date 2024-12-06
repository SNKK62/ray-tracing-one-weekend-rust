use crate::{ray, vec3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AABB {
    pub(super) min: vec3::Point3,
    pub(super) max: vec3::Point3,
}

impl AABB {
    pub fn new(min: &vec3::Point3, max: &vec3::Point3) -> Self {
        Self {
            min: *min,
            max: *max,
        }
    }

    pub fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let mut t0 = (self.min[a] - r.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - r.origin[a]) * inv_d;
            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let tt_min = t0.max(t_min);
            let tt_max = t1.min(t_max);
            if tt_max <= tt_min {
                return false;
            }
        }
        true
    }
}
