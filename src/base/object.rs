use gk_math::base::f32::Vec3;

use crate::base::intersect::Intersect;
use crate::base::bound::BoundBuilder;

pub struct Object {
    pub idx: usize,
    pub material: usize,
    pub base: Box<dyn ObjectBase + Sync + Send>,
}

pub trait ObjectBase: Intersect + BoundBuilder {}

pub trait ObjectTransfrom {
    fn scale(&mut self, s: f32) -> &mut Self;
    fn moved(&mut self, dir: Vec3) -> &mut Self;
}
