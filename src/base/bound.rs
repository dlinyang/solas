use crate::base::ray::Ray;
use rmu::vector::Vector3;

pub trait Bound {
    fn intersect(&self, ray :&Ray) -> bool;
    fn contain(&self, node: &Self) -> bool;
    fn point_in(&self, point: &Vector3) -> bool;
    fn sorround(left_box: &Self, right_box: &Self) -> Self;
}

pub trait BoundBuilder {
    fn get_aabb(&self) -> AABB;
}

//axis aliasing bounding box
#[derive(Debug)]
pub struct AABB {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub z_min: f32,
    pub z_max: f32,
}

impl AABB {
    pub fn new(x_min: f32, x_max: f32, y_min: f32,y_max: f32, z_min: f32,z_max: f32) -> Self {
        AABB {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        }
    }
}

// xₘᵢₙ - α < at < xₘₐₓ - α
#[inline]
fn compute(origin: f32, direction: f32,min: f32,max :f32) -> (f32,f32) {
    let d = 1f32 / direction;
    let t0 = (min - origin) * d;
    let t1 = (max - origin) * d;

    (t0,t1)
}

#[inline]
fn max(x: f32, y: f32) -> f32{
    if x > y  { x }  else { y }
}

#[inline]
fn min(x: f32, y: f32) -> f32 {
    if x < y { x } else { y }
}


impl Bound for AABB {
    fn intersect(&self, ray: &Ray) -> bool {

        let (tx0,tx1) = compute(ray.origin.x, ray.direction.x, self.x_min, self.x_max);
        let (ty0,ty1) = compute(ray.origin.y, ray.direction.y, self.y_min, self.y_max);
        let (tz0,tz1) = compute(ray.origin.z, ray.direction.z, self.z_min, self.z_max);

        let min = min(min(tx1,ty1),tz1);
        let max = max(max(tx0,ty0),tz0);

        if min >= max {
            false
        } else {
            true
        }
    }

    fn contain(&self, node: &AABB) -> bool {
        if node.x_min < self.x_min || node.x_max > self.x_max {
            false
        } else if node.y_min < self.x_min || node.y_max > self.x_max {
            false
        } else if node.z_min < self.z_min && node.z_max > self.z_max {
            false
        } else {
            true
        }
    }

    fn point_in(&self, point: &Vector3) -> bool {
        if self.x_min > point.x || self.x_max < point.x {
            false 
        } else if self.y_min > point.x || self.y_max < point.y {
            false
        } else if self.z_min > point.z || self.z_min < point.z {
            false
        } else {
            true
        }
    }


    fn sorround(left_box: &AABB, right_box: &AABB) -> AABB {
        AABB::new(
            min(left_box.x_min, right_box.x_min),
            max(left_box.x_max, right_box.x_max),
            min(left_box.y_min, right_box.y_min),
            max(left_box.y_max, right_box.y_max),
            min(left_box.z_min, right_box.z_min),
            max(left_box.z_max, right_box.z_max)
        )  
    }
}