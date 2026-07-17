use gk_math::base::f32::{Vec3,Vec2};
use std::f32;
use crate::base::object::{Object, ObjectTransfrom};
use crate::base::intersect::*;
use crate::base::ray::Ray;

pub mod obj;

pub struct Mesh{
    pub name: String,
    pub material: String,
    pub origin: Vec3,
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub texcoords: Vec<Vec2>,
    pub faces: Vec<[usize;9]>, // vertex | normal | texcoords
    pub bvh_opt: Option<BVHTree<AABB,usize>>,
}

impl Mesh {
    pub fn new<S: Into<String>>(name: S, material: S) -> Self {
        Self {
            name: name.into(),
            material: material.into(),
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

        self.bvh_opt = Some(BVHTree::sah_build(boxes))
    }

    //  create a cube
    pub fn cube(name: impl Into<String>, material: impl Into<String>, length: f32) -> Self {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        let mut normals = Vec::new();
        let mut texcoords = Vec::new();

        let half_length = length / 2.0f32;

        /* xyz coord
           v5----v4
          / |  / |
        v1--|-v0 |
        | v7--|-v6
        | /   | /
        v3----v2 */
        vertices.push(Vec3::new(half_length, half_length, half_length));//v0
        vertices.push(Vec3::new(-half_length, half_length, half_length));//v1
        vertices.push(Vec3::new(half_length, half_length, -half_length));//v2
        vertices.push(Vec3::new(-half_length, half_length, -half_length));//v3
        vertices.push(Vec3::new(half_length, -half_length, half_length));//v4
        vertices.push(Vec3::new(-half_length, -half_length, half_length));//v5
        vertices.push(Vec3::new(half_length, -half_length, -half_length));//v6
        vertices.push(Vec3::new(-half_length, -half_length, -half_length));//v7

        /*
              t
              ^   b
              | /
        l<—— a ——> r
            /|
          f  b
        */

        normals.push(Vec3::new(0.0, 1.0, 0.0));
        normals.push(Vec3::new(0.0, -1.0, 0.0));
        normals.push(Vec3::new(0.0, 0.0, 1.0));
        normals.push(Vec3::new(0.0, 0.0, -1.0));
        normals.push(Vec3::new(-1.0, 0.0, 0.0));
        normals.push(Vec3::new(1.0, 0.0, 0.0));

        texcoords.push(Vec2::new(0.0, 0.0));

        //front
        faces.push([1, 2, 3, 0, 0, 0, 0, 0, 0]);
        faces.push([0, 1, 2, 0, 0, 0, 0, 0, 0]);

        //back
        faces.push([4, 5, 6, 1, 1, 1, 0, 0, 0]);
        faces.push([5, 6, 7, 1, 1, 1, 0, 0, 0]);

        // top
        faces.push([0, 1, 4, 2, 2, 2, 0, 0, 0]);
        faces.push([1, 4, 5, 2, 2, 2, 0, 0, 0]);

        // bottom
        faces.push([2, 3, 6, 3, 3, 3, 0, 0, 0]);
        faces.push([3, 6, 7, 3, 3, 3, 0, 0, 0]);

        // left
        faces.push([1, 3, 5, 4, 4, 4, 0, 0, 0]);
        faces.push([3, 5, 7, 4, 4, 4, 0, 0, 0]);

        // right
        faces.push([0, 2, 4, 5, 5, 5, 0, 0, 0]);
        faces.push([2, 4, 6, 5, 5, 5, 0, 0, 0]);

        Self {
            name: name.into(),
            material: material.into(),
            origin: Vec3::new(0.0, 0.0, 0.0),
            vertices,
            normals,
            texcoords,
            faces,
            bvh_opt: None,
        }
    }
}

impl Object for Mesh {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn material(&self) -> String {
        self.material.clone()
    }
}

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

        let mut flag = false;
        let mut temp_time =  Default::default();
        let mut temp_normal = Vec3::new(0.0, 0.0, 0.0);
        let mut temp_uv = Vec2::new(0.0, 0.0);

        // if let Some(bvh) = &self.bvh_opt {
        //     for idx in bvh.intersect(ray) {
        //         let [a, b, c, an, bn, cn, ac, bc, cc] = &self.faces[idx];
        //         if let Some((time, normal, uv)) = triangle_interset(
        //             ray,
        //             self.vertices[*a], self.vertices[*b], self.vertices[*c],
        //             self.normals[*an], self.normals[*bn], self.normals[*cn],
        //             self.texcoords[*ac], self.texcoords[*bc], self.texcoords[*cc]
        //         ) {
        //             if flag {
        //                 if temp_time > time {
        //                     temp_time = time;
        //                     temp_normal = normal;
        //                     temp_uv = uv;
        //                 }
        //             } else {
        //                 temp_time = time;
        //                 temp_normal = normal;
        //                 temp_uv = uv;
        //                 flag = true;
        //             }
        //         }
        //     }
        // }
        // else {
            for [a,b,c,an,bn,cn,ac,bc,cc] in self.faces.iter() {
                if let Some((time, normal, uv)) = triangle_interset(
                    ray,
                    self.vertices[*a], self.vertices[*b], self.vertices[*c],
                    self.normals[*an], self.normals[*bn], self.normals[*cn],
                    self.texcoords[*ac], self.texcoords[*bc], self.texcoords[*cc]
                ) {
                    if flag {
                        if temp_time > time {
                            temp_time = time;
                            temp_normal = normal;
                            temp_uv = uv;
                        }
                    } else {
                        temp_time = time;
                        temp_normal = normal;
                        temp_uv = uv;
                        flag = true;
                    }
                }
            }
        // }

        if flag {
            Some(Hit::new(temp_time, ray.get_a_ray(temp_time), temp_normal, temp_uv, self.material.clone()))
        } else {
            None
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
use crate::bvh::BVHTree;

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
