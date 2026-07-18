use gk_math::base::f32::Vec3;
use crate::base::intersect::Hit;
use crate::base::material::*;
use crate::base::ray::Ray;
use crate::base::optics::*;

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new() -> Self {
        Self {
            albedo: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn with_albedo(mut self, albedo: Vec3) -> Self {
        self.albedo = albedo;
        self
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Scatter {
        let target = hemisphere_suface_distributrion(hit.normal);
        let scattered = Ray::new(hit.position,target, hit.time);
        Scatter::new(self.albedo.into(), scattered)
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new() -> Self {
        Self {
            albedo: Vec3::new(0.0, 0.0, 0.0),
            fuzz: Default::default(),
        }
    }

    pub fn with_albedo(mut self, albedo: Vec3) -> Self {
        self.albedo = albedo;
        self
    }

    pub fn with_fuzz(mut self, fuzz: f32) -> Self {
        self.fuzz = fuzz;
        self
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Scatter {
        let reflected = reflect(ray.direction.normalized(), hit.normal);
        let scattered = Ray::new(hit.position, reflected + self.fuzz * hemisphere_suface_distributrion(hit.normal), hit.time);
        Scatter::new(self.albedo.into(), scattered)
    }
}

pub struct Dielectric {
    pub albedo: Vec3,
    pub refract_coe: f32,
}

impl Dielectric {
    pub fn new() -> Self {
        Self {
            albedo: Vec3::new(0.8, 0.8, 0.8),
            refract_coe: Default::default(),
        }
    }

    pub fn with_albedo(mut self, albedo: Vec3) -> Self {
        self.albedo = albedo;
        self
    }

    pub fn with_refract_coe(mut self, refract_coe: f32) -> Self {
        self.refract_coe = refract_coe;
        self
    }
}

impl Material for Dielectric {
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
