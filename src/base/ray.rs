use gk_math::base::f32::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn default() -> Self {
        Ray {
            origin: Vec3::new(0., 0., 0.),
            direction: Vec3::new(0., 0., 0.),
            time: 0.0,
        }
    }

    pub fn get_a_ray(&self, coefficient: f32) -> Vec3 {
        self.origin + coefficient * self.direction
    }
}
use std::fmt;

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Ray {{ origin: {}, direction: {}, time: {} }}",
            self.origin, self.direction, self.time
        )
    }
}
