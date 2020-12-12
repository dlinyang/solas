use rmu::vector::{Vector2,Vector3};
use super::ray::Ray;

pub struct Hit{
    pub time: f32,
    pub position: Vector3,
    pub normal: Vector3,
    pub uv: Vector2,
    pub material_name: String,
}

impl Hit {
    #[inline]
    pub fn new(time: f32, position: Vector3, normal: Vector3, uv: Vector2, material_name: String) -> Self {
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
            position: Vector3::broadcast(0.0),
            normal: Vector3::broadcast(0.0),
            uv: Vector2::broadcast(0.0),
            material_name: String::new(),
        }
    }
}

pub trait Intersect { 
    fn intersect(&self, ray: &Ray,t_min: f32,t_max: f32) -> Option<Hit>;
}