use gk_math::base::f32::Vec3;
use std::f32::consts::PI;
use num::Float;
use super::ray::Ray;

pub struct Camera{
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub upper_left_corner: Vec3,
    pub lens_radius: f32,
    pub u: Vec3,
    pub w: Vec3,
    pub v: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Self {

        let theta = vfov * PI/180f32;
        let half_height = Float::tan(theta/2f32);
        let half_width  = aspect * half_height;
        let w = (look_from - look_at).normalized();
        let u = Vec3::cross(&vup,&w).normalized();
        let v = Vec3::cross(&w,&u);
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
            origin: Vec3::new(10.0, 2.0, 0.0),
            horizontal: Vec3::new(0.0, 4.0, 0.0),
            vertical: Vec3::new(0.0, 0.0, -2.0),
            upper_left_corner: Vec3::new(0.0, -2.0, 1.0),
            lens_radius: 1.0,
            u: Vec3::new(-1.0,0.0,0.0),
            v: Vec3::new(0.0,0.0,0.0),
            w: Vec3::new(0.0,0.0,0.0),
        }
    }
}

use crate::base::random::*;

fn random_in_unit_disk() -> Vec3 {
    let mut rng = PCG32::new();
    let mut p = Vec3::new(1., 1., 1.);

    while Vec3::dot(&p,&p) >= 1.0 {
        p = 2.0 * Vec3::new(rng.rand(), rng.rand(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
    }

    p
}
