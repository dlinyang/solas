use crate::base::intersect::Intersect;
use crate::base::bound::BoundBuilder;

pub trait Object: Intersect + BoundBuilder {
    fn name(&self) -> String;
    fn material(&self) -> String;
}