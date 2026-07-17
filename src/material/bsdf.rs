use gk_math::base::f32::Vec3;
use crate::base::intersect::Hit;
use crate::base::material::*;
use crate::base::ray::Ray;
use crate::base::optics::*;

pub struct Lambertian {
    pub name: String,
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            albedo: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Material for Lambertian {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Scatter {
        let target = hemisphere_suface_distributrion(hit.normal);
        let scattered = Ray::new(hit.position,target, hit.time);
        Scatter::new(self.albedo.into(), scattered)
    }
}

pub struct Metal {
    pub name: String,
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            albedo: Vec3::new(0.0, 0.0, 0.0),
            fuzz: Default::default(),
        }
    }
}

impl Material for Metal {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn scatter(&self, ray: &Ray, hit: &Hit) -> Scatter {
        let reflected = reflect(ray.direction.normalized(), hit.normal);
        let scattered = Ray::new(hit.position, reflected + self.fuzz * hemisphere_suface_distributrion(hit.normal), hit.time);
        Scatter::new(self.albedo.into(), scattered)
    }
}

pub struct Dielectric {
    pub name: String,
    pub albedo: Vec3,
    pub refract_coe: f32,
}

impl Dielectric {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            albedo: Vec3::new(0.8, 0.8, 0.8),
            refract_coe: Default::default(),
        }
    }
}

impl Material for Dielectric {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn scatter(&self, ray: &Ray, hit: &Hit) -> Scatter {

        let (outward_normal, ni_over_nt) = if Vec3::dot(&ray.direction, &hit.normal) > 0.0 {
            (-hit.normal,self.refract_coe)
        } else {
            (hit.normal,1.0 / self.refract_coe)
        };

        let scattered = if let Some(refracted) = refract(ray.direction, outward_normal, ni_over_nt) {
            Ray::new(hit.position, refracted, hit.time)
        } else {
            let reflected = reflect(ray.direction.normalized(), hit.normal);
            Ray::new(hit.position, reflected, ray.time)
        };

        Scatter::new(self.albedo.into(), scattered)
    }
}
