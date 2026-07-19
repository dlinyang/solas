use gk_math::base::f32::*;
use crate::base::object::*;
use crate::base::intersect::*;
use crate::base::bound::*;
use crate::base::ray::*;

pub struct Plane {
    pub center: Vec3,
    pub normal: Vec3,
    pub u_axis: Vec3,
    pub v_axis: Vec3,
    pub width: f32,
    pub height: f32,
}

impl Plane {
    pub fn new() -> Self {
        Plane {
            center: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            u_axis: Vec3::new(1.0, 0.0, 0.0),
            v_axis: Vec3::new(0.0, 0.0, 1.0),
            width: 1.0,
            height: 1.0,
        }
    }

    pub fn with_center(mut self, center: Vec3) -> Self {
        self.center = center;
        self
    }

    pub fn with_normal(mut self, normal: Vec3) -> Self {
        self.normal = normal.normalized();
        // Recompute u and v axes from the new normal
        let (u, v) = Self::build_axes(self.normal);
        self.u_axis = u;
        self.v_axis = v;
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn set_center(&mut self, center: Vec3) {
        self.center = center;
    }

    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal.normalized();
        let (u, v) = Self::build_axes(self.normal);
        self.u_axis = u;
        self.v_axis = v;
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }

    fn build_axes(n: Vec3) -> (Vec3, Vec3) {
        let up = if n.y.abs() < 0.9 { Vec3::new(0.0, 1.0, 0.0) } else { Vec3::new(1.0, 0.0, 0.0) };
        let u = Vec3::cross(&n, &up).normalized();
        let v = Vec3::cross(&u, &n).normalized();
        (u, v)
    }
}

impl ObjectBase for Plane {}

impl ObjectTransfrom for Plane {
    fn scale(&mut self, s: f32) -> &mut Self {
        self.width *= s;
        self.height *= s;
        self
    }

    fn moved(&mut self, dir: Vec3) -> &mut Self {
        self.center += dir;
        self
    }
}

impl Intersect for Plane {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let denom = Vec3::dot(&self.normal, &ray.direction);
        if denom.abs() > 1e-6 {
            let t = Vec3::dot(&(self.center - ray.origin), &self.normal) / denom;
            if t > t_min && t < t_max {
                let position = ray.get_a_ray(t);
                let hit_vec = position - self.center;
                let u = Vec3::dot(&hit_vec, &self.u_axis);
                let v = Vec3::dot(&hit_vec, &self.v_axis);
                let half_w = self.width / 2.0;
                let half_h = self.height / 2.0;
                if u.abs() <= half_w && v.abs() <= half_h {
                    // Map u,v to [0,1] texture coordinates
                    let tex_u = (u / self.width) + 0.5;
                    let tex_v = (v / self.height) + 0.5;
                    return Some(Hit::new(t, position, self.normal, Vec2::new(tex_u, tex_v)));
                }
            }
        }
        None
    }
}

impl BoundBuilder for Plane {
    fn get_aabb(&self) -> AABB {
        let half_w = self.width / 2.0;
        let half_h = self.height / 2.0;
        // Compute the four corners of the rectangle in local space
        let corners = [
            self.center - self.u_axis * half_w - self.v_axis * half_h,
            self.center + self.u_axis * half_w - self.v_axis * half_h,
            self.center - self.u_axis * half_w + self.v_axis * half_h,
            self.center + self.u_axis * half_w + self.v_axis * half_h,
        ];
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut min_z = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;
        let mut max_z = f32::MIN;
        for c in &corners {
            if c.x < min_x { min_x = c.x; }
            if c.x > max_x { max_x = c.x; }
            if c.y < min_y { min_y = c.y; }
            if c.y > max_y { max_y = c.y; }
            if c.z < min_z { min_z = c.z; }
            if c.z > max_z { max_z = c.z; }
        }
        AABB::new(min_x, max_x, min_y, max_y, min_z, max_z)
    }
}
