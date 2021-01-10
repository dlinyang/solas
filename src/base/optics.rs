use rmu::vector::Vector3;
use super::random::RNG;
use std::f32::consts::PI;

//calculate  diffuse refkection with Monte Carlo method
pub fn hemisphere_suface_distributrion(normal: Vector3) -> Vector3 {
    use rmu::matrix::Matrix3x3;

    let mut rng = RNG::new();

    let rotation = Matrix3x3::rotation3(rng.rand() * 0.5f32 * PI, rng.rand() * 0.5f32 * PI, rng.rand() * 0.5f32 * PI);

    rotation * normal
}

//reflection
pub fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - 2.0 * Vector3::dot(v,n) * n
}

pub fn refract(v: Vector3, n: Vector3, ni_over_nt: f32) -> Option<Vector3> {
    let uv = v.normalized();
    let dt = Vector3::dot(uv, n);
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