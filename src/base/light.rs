use rmu::vector::{Vector3,Color};
use super::ray::Ray;

pub trait Light {
    fn radiation(&self,ray: &Ray) -> Color;
}

pub struct PointLight {
    pub origin: Vector3,
    pub color: Color,
    pub brightness: f32,
}

impl PointLight {
    pub fn new() -> Self {
        PointLight {
            origin: Vector3::new(2f32, 0f32, 1f32),
            color: Vector3::broadcast(1f32),
            brightness: 1.0,
        }
    }

    pub fn create(origin: Vector3, color: Color, brightness: f32) -> Self {
        PointLight {
            origin,
            color,
            brightness,
        }
    }
}

impl Light for PointLight {
    //get the light r
    fn radiation(&self, ray: &Ray) -> Color {
        let light_direction = self.origin - ray.origin;
        let cos = Vector3::dot(light_direction, ray.direction) / (light_direction.length() * ray.direction.length());
        if cos > 0.0 {
            self.color * cos
        } else {
            Color::default()
        }
    }
}