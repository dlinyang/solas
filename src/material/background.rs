use rmu::vector::{Vector3,Color};
use crate::base::ray::Ray;
use crate::base::material::SkyBox;

pub struct Background {
}

impl Background {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl SkyBox for Background {
    fn ambient(&self, ray: &Ray) -> Color {
        let normal = ray.direction.normalized();
        let t = 0.5 * (normal.y + 1.0);
        (1.0 - t) * Vector3::broadcast(1.0) + t*Vector3::new(0.5, 0.7, 1.0)
    }
}