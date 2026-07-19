use gk_math::base::f32::Vec3;
use gk_math::color::RGB as Color;
use super::ray::Ray;

pub trait Light {
    fn radiation(&self,ray: &Ray) -> Color;
    fn get_ray(&self, to_point: Vec3) -> Ray;
}

pub struct PointLight {
    pub origin: Vec3,
    pub color: Color,
    pub brightness: f32,
}

impl PointLight {
    pub fn new() -> Self {
        PointLight {
            origin: Vec3::new(2f32, 0f32, 1f32),
            color: Vec3::new(1f32, 1f32, 1f32).into(),
            brightness: 1.0,
        }
    }

    pub fn create(origin: Vec3, color: Color, brightness: f32) -> Self {
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
        let cos = Vec3::dot(&light_direction, &ray.direction) / (light_direction.length() * ray.direction.length());
        if cos > 0.0 {
            self.brightness * self.color  * cos / light_direction.length_squared()
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }

    fn get_ray(&self, to_point: Vec3) -> Ray {
        Ray { origin: to_point, direction: (self.origin - to_point ).normalized(), time: 0.001 }
    }
}
