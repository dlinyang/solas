use gk_math::base::f32::{Vec3,Vec2};
use std::f32;
use crate::base::object::{ObjectBase, ObjectTransfrom};
use crate::base::intersect::*;
use crate::base::ray::Ray;

pub mod obj;
pub mod cube;

pub struct Mesh {
    pub origin: Vec3,
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub texcoords: Vec<Vec2>,
    pub faces: Vec<[usize;9]>, // vertex | normal | texcoords
    // pub bvh_opt: Option<BVHTree<AABB,FixedVec<usize,32>>>,
    pub bvh_opt: Option<FlatBVHTree<AABB, FixedVec<usize,16>>>
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            origin: Vec3::new(0.0, 0.0, 0.0),
            vertices: Vec::new(),
            normals: Vec::new(),
            texcoords: Vec::new(),
            faces: Vec::new(),
            bvh_opt: None,
        }
    }

    pub fn build_bvh(&mut self){
        let boxes: Vec<(usize, AABB)> = self.faces.iter().enumerate().map( |(idx, face)| {
            let [a, b, c,..] = face;
            let a = &self.vertices[*a];
            let b = &self.vertices[*b];
            let c = &self.vertices[*c];
            let x_min = a.x.min(b.x).min(c.x);
            let x_max = a.x.max(b.x).max(c.x);
            let y_min = a.y.min(b.y).min(c.y);
            let y_max = a.y.max(b.y).max(c.y);
            let z_min = a.z.min(b.z).min(c.z);
            let z_max = a.z.max(b.z).max(c.z);
            (idx, AABB::new(x_min, x_max, y_min, y_max, z_min, z_max))
        }).collect();

        self.bvh_opt = Some(FlatBVHTree::sah_build_multi_index_in_leaf(boxes))
    }

    pub fn intersect_triganle_nth(&self, idx: usize, ray: &Ray) -> Option<Hit> {
        let [a, b, c, an, bn, cn, ac, bc, cc] = &self.faces[idx];
        triangle_interset(
            ray,
            self.vertices[*a], self.vertices[*b], self.vertices[*c],
            self.normals[*an], self.normals[*bn], self.normals[*cn],
            self.texcoords[*ac], self.texcoords[*bc], self.texcoords[*cc]
        )
        .map(
            |(time, normal, uv)|
            Hit::new(time,
                ray.get_a_ray(time),
                normal,
                uv,
            ))
    }
}

impl ObjectBase for Mesh {}

impl ObjectTransfrom for Mesh {
    fn scale(&mut self, s: f32) -> &mut Self {
        for v in self.vertices.iter_mut() {
            *v = *v * s;
        }
        self
    }

    fn moved(&mut self, dir: Vec3) -> &mut Self {
        for v in self.vertices.iter_mut() {
            *v = *v + dir;
        }
        self
    }
}

impl Intersect for Mesh {
    fn intersect(&self,ray: &Ray, _t_min: f32, _t_max: f32) -> Option<Hit> {
        if let Some(bvh) = &self.bvh_opt {
            return bvh.intersect_f(ray, move |idx_array, ray| {
                let mut ret: Option<Hit> = None;
                for i in 0..idx_array.len() {
                    let [a, b, c, an, bn, cn, ac, bc, cc] = &self.faces[idx_array.data[i]];
                    if let Some((time, normal, uv)) = triangle_interset(
                        ray,
                        self.vertices[*a], self.vertices[*b], self.vertices[*c],
                        self.normals[*an], self.normals[*bn], self.normals[*cn],
                        self.texcoords[*ac], self.texcoords[*bc], self.texcoords[*cc]
                    ) {
                        if let Some(pre_ret) = &ret {
                            if pre_ret.time > time {
                                ret = Some(Hit::new(time, ray.get_a_ray(time), normal, uv))
                            }
                        } else {
                            ret = Some(Hit::new(time, ray.get_a_ray(time), normal, uv))
                        }
                    }
                }
                ret
            });
        }
        else {
            let mut ret: Option<Hit> = None;

            for [a,b,c,an,bn,cn,ac,bc,cc] in self.faces.iter() {
                if let Some((time, normal, uv)) = triangle_interset(
                    ray,
                    self.vertices[*a], self.vertices[*b], self.vertices[*c],
                    self.normals[*an], self.normals[*bn], self.normals[*cn],
                    self.texcoords[*ac], self.texcoords[*bc], self.texcoords[*cc]
                ) {
                    if let Some(pre_ret) = &ret {
                        if pre_ret.time > time {
                            ret = Some(Hit::new(time, ray.get_a_ray(time), normal, uv))
                        }
                    } else {
                        ret = Some(Hit::new(time, ray.get_a_ray(time), normal, uv))
                    }
                }
            }

            return ret;
        }
    }
}

/*
A
|\    d
B_C  ← p
p + td = (1-u-v)A + uB + vC
p - A = - td + u(B-A) + v(C-A)
(p - A) (-d, B-A, C-A)⁻¹  = (t,u,v)
*/
// ray -> triagnle -> (time, normal, uv)
fn triangle_interset(
    ray: &Ray,
    a:  Vec3, b:  Vec3, c:  Vec3,
    an: Vec3, bn: Vec3, cn: Vec3,
    ac: Vec2, bc: Vec2, cc: Vec2
) -> Option<(f32, Vec3, Vec2)> {
    let e1 = b - a;

    let e2 = c - a;

    let plane_normal  =  Vec3::cross(&e1, &e2);

    if Vec3::dot(&ray.direction, &plane_normal) != 0.0 {
        let e = ray.origin - a;
        let d = ray.direction;
        let m = Vec3::cross(&d, &e2);
        let rev_det = Vec3::dot(&m, &e1).recip();
        let k = Vec3::cross(&e, &e1);

        let t = Vec3::dot(&k, &e2) * rev_det;
        let u = Vec3::dot(&m, &e) * rev_det;
        let v = Vec3::dot(&k, &d) * rev_det;

        if u >= 0.0 && v >= 0.0 && u + v <= 1.0 && t > 0.0 {
            let normal = (1.0 - u - v) * an + u * bn + v *cn;
            let uv = (1.0 - u - v) * ac + u * bc + v * cc;
            return Some((t, normal, uv));
        }
    }

    None
}

use crate::base::bound::*;
use crate::bvh::*;

impl BoundBuilder for Mesh {
    fn get_aabb(&self) -> AABB {
        let mut x_min = Default::default();
        let mut x_max = Default::default();
        let mut y_min = Default::default();
        let mut y_max = Default::default();
        let mut z_min = Default::default();
        let mut z_max = Default::default();
        let mut flag = false;

        for v in self.vertices.iter() {
            if flag {
                if v.x < x_min {
                    x_max = v.x
                } else if v.x > x_max {
                    x_max = v.x
                }

                if v.y < y_min {
                    y_min = v.y;
                } else if v.y > y_max {
                    y_max = v.y;
                }

                if v.z < z_min {
                    z_max = v.z;
                } else if v.z > z_max {
                    z_max = v.z;
                }

            } else {
                x_min = v.x;
                x_max = v.x;
                y_min = v.y;
                y_max = v.y;
                z_min = v.z;
                z_max = v.z;
                flag = true;
            }
        }

        AABB::new(x_min, x_max, y_min, y_max, z_min, z_max)
    }
}
