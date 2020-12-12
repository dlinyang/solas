use rmu::vector::Vector3;
use std::f32;
use crate::base::object::Object;
use crate::base::intersect::*;
use crate::base::ray::Ray;

pub struct Sphere{
    pub name: String,
    pub material: String,
    pub center: Vector3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(name: String, material: String) -> Self{
        Sphere{
            name,
            material,
            center: Vector3::default(),
            radius: 1.0,
        }
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn with_center(mut self, center: Vector3) -> Self {
        self.center = center;
        self
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    pub fn set_center(&mut self, center: Vector3) {
        self.center = center;
    }
}

impl Object for Sphere {
    fn name(&self) -> String {
        self.name.clone()
    }
    
    fn material(&self) -> String {
        self.material.clone()
    }
}

impl Intersect for Sphere {
    fn intersect(&self,ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = Vector3::dot(ray.direction, ray.direction);
        let b = Vector3::dot(oc, ray.direction);
        let c = Vector3::dot(oc,oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let time = temp;
                let position = ray.get_a_ray(temp);
                let normal = (position - self.center) / self.radius;
                return Some(Hit::new(time, position, normal, Default::default(), self.material.clone()))
            }

            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let time = temp;
                let position = ray.get_a_ray(temp);
                let normal = (position - self.center) / self.radius;
                return Some(Hit::new(time, position, normal, Default::default(), self.material.clone()))
            }
        } 

        None
    }
}

use crate::base::bound::*;

impl BoundBuilder for Sphere {
    fn get_aabb(&self) -> AABB {
        let x = self.center.x;
        let y = self.center.y;
        let z = self.center.z;
        let r = self.radius;

        AABB::new(x - r, x + r, y - r , y + r, z - r, z + r)
    }
}
