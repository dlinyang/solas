use std::sync::Arc;
use crate::base::ray::Ray;
use crate::base::bound::Bound;

pub struct BVHTree<B: Bound>{
    pub left: Option<Arc<BVHTree<B>>>,
    pub right: Option<Arc<BVHTree<B>>>, 
    pub object: Option<String>,
    pub node: B,
}

impl<B: Bound> BVHTree<B> {
    pub fn new(node: B, object: Option<String>) -> Self {
        Self {
            left: None,
            right: None,
            object,
            node,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<String> {
        let mut result = Vec::new();

        if self.node.intersect(ray) {
            if let Some(name) = &self.object {
                result.push(name.clone());
            }

            if let Some(node) = &self.left {
                result.append(&mut node.intersect(ray));
            } 
            
            if let Some(node) = &self.right {
                result.append(&mut node.intersect(ray));
            }
        }

        result
    }
}