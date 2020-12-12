use rmu::vector::{Vector3,Vector2};
use std::f32;
use crate::base::object::Object;
use crate::base::intersect::*;
use crate::base::ray::Ray;

pub struct Mesh{
    pub name: String,
    pub material: String,
    pub origin: Vector3,
    pub vertices: Vec<Vector3>, 
    pub normals: Vec<Vector3>,
    pub uv: Vec<Vector2>,
    pub faces: Vec<[usize;9]>, // vertex | normal | uv
}

impl Mesh {
    pub fn new(name: String, material: String) -> Self {
        Self {
            name,
            material,
            origin: Vector3::default(),
            vertices: Vec::new(),
            normals: Vec::new(),
            uv: Vec::new(),
            faces: Vec::new(),
        }
    }

    //  create a cube
    pub fn cube(name: String, material: String, length: f32) -> Self {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        let mut normals = Vec::new();
        let mut uv = Vec::new();
        
        let half_length = length / 2.0f32;

        /* xyz coord
           v5----v4
          / |  / |
        v1--|-v0 |
        | v7--|-v6
        | /   | /
        v3----v2 */ 
        vertices.push(Vector3::new(half_length, half_length, half_length));//v0
        vertices.push(Vector3::new(-half_length, half_length, half_length));//v1
        vertices.push(Vector3::new(half_length, half_length, -half_length));//v2
        vertices.push(Vector3::new(-half_length, half_length, -half_length));//v3
        vertices.push(Vector3::new(half_length, -half_length, half_length));//v4
        vertices.push(Vector3::new(-half_length, -half_length, half_length));//v5
        vertices.push(Vector3::new(half_length, -half_length, -half_length));//v6
        vertices.push(Vector3::new(-half_length, -half_length, -half_length));//v7

        /*
              t
              ^   b
              | /
        l<—— a ——> r
            /|
          f  b
        */ 

        normals.push(Vector3::new(0.0, 1.0, 0.0));
        normals.push(Vector3::new(0.0, -1.0, 0.0));
        normals.push(Vector3::new(0.0, 0.0, 1.0));
        normals.push(Vector3::new(0.0, 0.0, -1.0));
        normals.push(Vector3::new(-1.0, 0.0, 0.0));
        normals.push(Vector3::new(1.0, 0.0, 0.0));

        uv.push(Vector2::new(0.0, 0.0));

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
            name,
            material,
            origin: Vector3::default(),
            vertices,
            normals,
            uv,
            faces,
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

impl Intersect for Mesh {
    fn intersect(&self,ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut flag = false;
        let mut temp_time =  Default::default();
        let mut temp_normal = Vector3::default();
        let mut temp_uv = Vector2::default();

        let o = self.origin;

        for [a,b,c,an,bn,cn,ac,bc,cc] in self.faces.iter() {
            if let Some((time, normal, uv)) = triangle_interset(
                ray, 
                self.vertices[*a], self.vertices[*b], self.vertices[*c],
                self.normals[*an], self.normals[*bn], self.normals[*cn],
                self.uv[*ac], self.uv[*bc], self.uv[*cc]
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
    a: Vector3, b: Vector3, c: Vector3, 
    an: Vector3, bn: Vector3, cn: Vector3,
    ac: Vector2, bc: Vector2, cc: Vector2) -> Option<(f32, Vector3, Vector2)> {
    let e1 = b - a;
    
    let e2 = c - a;

    let plane_normal  =  Vector3::cross(e1, e2);

    if Vector3::dot(ray.direction, plane_normal) != 0.0 {
        let e = ray.origin - a;
        let d = ray.direction;
        let m = Vector3::cross(d, e2);
        let det = Vector3::dot(m, e1);
        let k = Vector3::cross(e, e1);
        
        let t = Vector3::dot(k,e2)/det;
        let u = Vector3::dot(m,e)/det;
        let v = Vector3::dot(k,d)/det;

        if u >= 0.0 && v >= 0.0 && u + v <= 1.0 && t > 0.0 {
            let normal = (1.0 - u - v) * an + u * bn + v *cn;
            let uv = (1.0 - u - v) * ac + u * bc + v * cc;
            return Some((t, normal, uv));
        }
    } 

    None
}

use crate::base::bound::*;

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