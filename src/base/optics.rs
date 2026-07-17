use gk_math::base::f32::Vec3;
use gk_math::transform::rotate::rotate3d;
use super::random::RNG;
use std::f32::consts::PI;

//calculate  diffuse refkection with Monte Carlo method
pub fn hemisphere_suface_distributrion(normal: Vec3) -> Vec3 {

    let mut rng = RNG::new();

    let rotation = rotate3d(rng.rand() * 0.5f32 * PI, rng.rand() * 0.5f32 * PI, rng.rand() * 0.5f32 * PI);

    rotation * normal
}

//reflection
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(&n) * n
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalized();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - dt * n) - discriminant.sqrt() * n)
    }
    else {
        None
    }
}

pub fn shlick(cosine: f32, refract_coe: f32) -> f32 {
    let mut r = (1.0 - refract_coe) / (1.0 + refract_coe);
    r = r * r;
    r + (1.0 - r) * (1.0 - cosine).powf(5.0)
}
