use gk_math::base::f32::{Vec2, Vec3};
use std::f32;
use crate::base::object::{ObjectBase, ObjectTransfrom};
use crate::base::intersect::*;
use crate::base::ray::Ray;

pub struct Sphere{
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new() -> Self{
        Sphere{
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 1.0,
        }
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn with_center(mut self, center: Vec3) -> Self {
        self.center = center;
        self
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    pub fn set_center(&mut self, center: Vec3) {
        self.center = center;
    }
}

impl ObjectBase for Sphere {}

impl ObjectTransfrom for Sphere {
    fn scale(&mut self, s: f32) -> &mut Self {
        self.radius = self.radius * s;
        self
    }

    fn moved(&mut self, dir: Vec3) -> &mut Self {
        self.center += dir;
        self
    }
}

impl Intersect for Sphere {
    fn intersect(&self,ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = Vec3::dot(&ray.direction, &ray.direction);
        let b = Vec3::dot(&oc, &ray.direction);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let time = temp;
                let position = ray.get_a_ray(temp);
                let normal = (position - self.center) / self.radius;
                return Some(Hit::new(time, position, normal, Vec2::new(0.0, 0.0)))
            }

            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let time = temp;
                let position = ray.get_a_ray(temp);
                let normal = (position - self.center) / self.radius;
                return Some(Hit::new(time, position, normal, Vec2::new(0.0, 0.0)))
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
