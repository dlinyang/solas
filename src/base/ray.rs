use rmu::vector::Vector3;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3, time: f32) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn default() -> Self {
        Ray {
            origin: Vector3::default(),
            direction: Vector3::default(),
            time: 0.0,
        }
    }

    pub fn get_a_ray(&self, coefficient: f32) -> Vector3 {
        self.origin + coefficient * self.direction
    }
}
