use gk_math::base::f32::Vec3;

use crate::base::intersect::Intersect;
use crate::base::bound::BoundBuilder;

pub trait Object: Intersect + BoundBuilder {
    fn name(&self) -> String;
    fn material(&self) -> String;
}

pub trait ObjectTransfrom {
    fn scale(&mut self, s: f32) -> &mut Self;
    fn moved(&mut self, dir: Vec3) -> &mut Self;
}
