use super::ray::Ray;
use super::intersect::Hit;
use rmu::vector::Color;

pub struct Scatter {
    pub attenuation: Color,
    pub scattered: Ray,
}

impl Scatter {
    #[inline]
    pub fn new(attenuation: Color, scattered: Ray) -> Self {
        Self {
            attenuation,
            scattered,
        }
    }
}

pub trait Material {
    fn name(&self) -> String;
    /// Material's scatter property
    fn scatter(&self,ray: &Ray, hit: &Hit) -> Scatter;
    /// Luminescent materials 
    fn emit(&self, _ray: &Ray) -> Color { 
        Color::default() 
    }
}

pub trait SkyBox {
    fn ambient(&self, ray: &Ray) -> Color;
}