use gk_math::base::f32::{Vec2,Vec3};
use super::ray::Ray;

pub struct Hit{
    pub time: f32,
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
    pub material_name: String,
}

impl Hit {
    #[inline]
    pub fn new(time: f32, position: Vec3, normal: Vec3, uv: Vec2, material_name: String) -> Self {
        Self {
            time,
            position,
            normal,
            uv,
            material_name,
        }
    }
}

impl Default for Hit {
    fn default() -> Self {
        Self {
            time: 0.0,
            position: Vec3::new(1.0,1.0, 1.0),
            normal: Vec3::new(1.0,1.0, 1.0),
            uv: Vec2::new(1.0, 1.0),
            material_name: String::new(),
        }
    }
}

pub trait Intersect {
    fn intersect(&self, ray: &Ray,t_min: f32,t_max: f32) -> Option<Hit>;
}
