use rmu::vector::Vector3;
use std::f32::consts::PI;
use num::Float;
use super::ray::Ray;

pub struct Camera{
    pub origin: Vector3,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    pub upper_left_corner: Vector3,
    pub lens_radius: f32,
    pub u: Vector3,
    pub w: Vector3,
    pub v: Vector3,
}

impl Camera {
    pub fn new(look_from: Vector3, look_at: Vector3, vup: Vector3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Self {
           
        let theta = vfov * PI/180f32;
        let half_height = Float::tan(theta/2f32);
        let half_width  = aspect * half_height;
        let w = (look_from - look_at).normalized();
        let u = Vector3::cross(vup,w).normalized();
        let v = Vector3::cross(w,u);
        Self {
            origin: look_from,
            upper_left_corner: look_from - half_width * focus_dist * u + half_height * focus_dist * v - focus_dist * w,
            horizontal: 2f32 * half_width * focus_dist * u,
            vertical: 2f32 * half_height * focus_dist * v,
            lens_radius: aperture/2f32,
            w: w,
            u: u,
            v: v, 
        }
        
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset, 
                 self.upper_left_corner + u * self.horizontal - v * self.vertical  - self.origin - offset,
                 0.0)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            origin: Vector3::new(10.0, 2.0, 0.0),
            horizontal: Vector3::new(0.0, 4.0, 0.0),
            vertical: Vector3::new(0.0, 0.0, -2.0),
            upper_left_corner: Vector3::new(0.0, -2.0, 1.0),
            lens_radius: 1.0,
            u: Vector3::new(-1.0,0.0,0.0),
            v: Vector3::new(0.0,0.0,0.0),
            w: Vector3::new(0.0,0.0,0.0),
        }
    }
}

use rand::prelude::*;

fn random_in_unit_disk() -> Vector3 {
    let mut rng = thread_rng();
    let mut p = Vector3::broadcast(1.0);

    while Vector3::dot(p,p) >= 1.0 {
        p = 2.0 * Vector3::new(rng.gen_range(0f32,1f32), rng.gen_range(0f32,1f32), 0.0) - Vector3::new(1.0, 1.0, 0.0);
    }

    p
}