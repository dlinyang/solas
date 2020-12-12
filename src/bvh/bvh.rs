use std::sync::Arc;
use crate::base::{
    ray::Ray,
    intersect::Hit,
};
use rmu::vector::Vector3;
use crate::object::Object;
use std::collections::HashMap;

//bounding volume hierarchy node
pub trait BVHNode {
    fn intersect(&self, ray :&Ray) -> bool;
    fn contain(&self, node: &Self) -> bool;
    fn point_in(&self, point: &Vector3) -> bool;
    fn sorround(left_box: &Self, right_box: &Self) -> Self;
}

pub trait BVHNodeBuilder<N> {
    fn get_node(&self) -> N;
}

//
pub struct BVHTree<N: BVHNode>{
    pub left: Option<Arc<BVHTree<N>>>,
    pub right: Option<Arc<BVHTree<N>>>, 
    pub object: Option<String>,
    pub node: N,
}

impl<N: BVHNode> BVHTree<N> {
    pub fn new(node: N, object: Option<String>) -> Self {
        Self {
            left: None,
            right: None,
            object,
            node,
        }
    }

    pub fn intersect(
        &self, 
        ray: &Ray, 
        t_min: f32, 
        t_max: f32, 
        objects: &HashMap<String, Arc<dyn Object + Send + Sync>>
    ) -> Option<Hit> {
        let mut temp_hit = Hit::default();
        let mut flag = false;

        if self.node.intersect(ray) {

            if let Some(name) = &self.object {
                if let Some(object) = objects.get(name) {
                    if let Some(hit) = object.intersect(ray, t_min, t_max) {
                        temp_hit = hit;
                        flag = true;
                    }
                }
            }

            if let Some(node) = &self.left {
                if let Some(hit) = node.intersect(ray, t_min, t_max, objects) {
                    if flag {
                        if hit.time < temp_hit.time {
                            temp_hit = hit;
                        }
                    } else {
                        temp_hit = hit;
                        flag = true;
                    }
                }
            } 
            
            if let Some(node) = &self.right {
                if let Some(hit) = node.intersect(ray, t_min, t_max, objects) {
                    if flag {
                        if hit.time < temp_hit.time {
                            temp_hit = hit;
                        }
                    } else {
                        temp_hit = hit;
                        flag = true;
                    }
                }
            }

            if flag {
                Some(temp_hit)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn contain(&mut self, node: N, object: String) -> Option<(N, String)> {
        if self.node.contain(&node) {
            if self.left.is_none() {
                self.left = Some(Arc::new(BVHTree::new(node, Some(object))));
                None
            } else if self.right.is_none() {
                self.right = Some(Arc::new(BVHTree::new(node, Some(object))));
                None
            } else {
                Some((node,object))
            }
        } else {
            Some((node, object))
        }
    }
}