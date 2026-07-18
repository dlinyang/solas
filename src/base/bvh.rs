
use gk_math::base::f32::Vec3;

use crate::base::ray::Ray;
use crate::bound::*;
use crate::intersect::Hit;

pub mod sah;
pub mod mi_sah;
pub use mi_sah::*;

#[derive(Debug)]
pub struct BVHTree<B: Bound = AABB, T = String> {
    pub left: Option<Box<Self>>,
    pub right: Option<Box<Self>>,
    pub object: Option<T>,
    pub node: B,
}

impl<B: Bound, T: Clone> BVHTree<B, T> {
    pub fn new(node: B, object: Option<T>) -> Self {
        Self {
            left: None,
            right: None,
            object,
            node,
        }
    }

    pub fn with_left(mut self, left: Self) -> Self {
        self.left = Some(Box::new(left));
        self
    }

    pub fn with_right(mut self, right: Self) -> Self {
        self.right = Some(Box::new(right));
        self
    }

    #[must_use]
    pub fn intersect_f<F>(&self, ray: &Ray, f: F) -> Option<Hit>
    where F: Fn(&T, &Ray) ->  Option<Hit> + Copy
    {
        if let Some(_) = self.node.intersect(ray) {
            if let Some(idx) = &self.object {
                return f(idx, ray);
            } else {
                let left_hit_opt = if let Some(node) = &self.left {
                    node.intersect_f(ray, f)
                } else { None };

                let right_hit_opt = if let Some(node) = &self.right {
                    if let Some(hit) = &left_hit_opt{
                        if let Some(time) = node.node.intersect(ray) {
                            if time > hit.time {
                                None
                            }
                            else {
                                node.intersect_f(ray, f)
                            }
                        }
                        else {
                            None
                        }
                    } else {
                        node.intersect_f(ray, f)
                    }
                } else { None };

                return match (left_hit_opt, right_hit_opt) {
                    (None, None) => None,
                    (None, Some(hit)) => Some(hit),
                    (Some(hit), None) => Some(hit),
                    (Some(lhit), Some(rhit)) => {
                        if lhit.time < rhit.time {
                            Some(lhit)
                        }
                        else {
                            Some(rhit)
                        }
                    },
                }
            }
        }

        None
    }

    #[must_use]
    pub fn intersect_f_idx<F>(&self, ray: &Ray, f: F) -> Option<(Hit, T)>
    where F: Fn(&T, &Ray) ->  Option<(Hit, T)> + Copy
    {
        if let Some(_) = self.node.intersect(ray) {
            if let Some(idx) = &self.object {
                return f(idx, ray);
            } else {
                let left_hit_opt = if let Some(node) = &self.left {
                    node.intersect_f_idx(ray, f)
                } else { None };

                let right_hit_opt = if let Some(node) = &self.right {
                    if let Some((hit, _)) = &left_hit_opt{
                        if let Some(time) = node.node.intersect(ray) {
                            if time > hit.time {
                                None
                            }
                            else {
                                node.intersect_f_idx(ray, f)
                            }
                        }
                        else {
                            None
                        }
                    } else {
                        node.intersect_f_idx(ray, f)
                    }
                } else { None };

                return match (left_hit_opt, right_hit_opt) {
                    (None, None) => None,
                    (None, Some(hit)) => Some(hit),
                    (Some(hit), None) => Some(hit),
                    (Some(lhit), Some(rhit)) => {
                        if lhit.0.time < rhit.0.time {
                            Some(lhit)
                        }
                        else {
                            Some(rhit)
                        }
                    },
                }
            }
        }
        None
    }
}

#[derive(Debug)]
pub enum TreeNodeType<T = usize> {
    Branch{ left: usize, right: usize},
    Leaf{ object: T},
}

#[derive(Debug)]
pub struct BVHTreeNode<B: Bound = AABB, T = usize> {
    pub tree_n_ty: TreeNodeType<T>,
    pub node: B,
}

pub struct FlatBVHTree<B: Bound = AABB, T = usize> {
    pub top: Option<usize>,
    pub nodes: Vec<BVHTreeNode<B, T>>
}

impl<B: Bound, T: Clone> BVHTreeNode <B, T> {
    pub fn leaf(node: B, object: T) -> Self {
        Self {
            tree_n_ty: TreeNodeType::Leaf { object },
            node,
        }
    }

    pub fn branch(node: B, left: usize, right: usize) -> Self {
        Self {
            tree_n_ty: TreeNodeType::Branch { left, right },
            node,
        }
    }
}

impl<B: Bound, T: Clone> FlatBVHTree<B, T> {
    pub fn new() -> Self {
        Self { top: None, nodes: Vec::new() }
    }

    pub fn push(&mut self, node: BVHTreeNode<B, T>) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    pub fn intersect_f<F>(&self, ray: &Ray, f: F) -> Option<Hit>
    where F: Fn(&T, &Ray) ->  Option<Hit> + Copy
    {
        if self.top.is_none() { panic!("FlatBVHTree Not exist a top node") }
        self.intersect_f_inner(ray, self.top.unwrap(), f)
    }

    pub fn intersect_f_inner<F>(&self, ray: &Ray, index: usize, f: F) -> Option<Hit>
    where F: Fn(&T, &Ray) ->  Option<Hit> + Copy
    {
        if self.nodes.get(index).is_none() {panic!("shoulden't access node that not exsit")}

        let node = &self.nodes[index];

        if let Some(_) = node.node.intersect(ray) {
            match &node.tree_n_ty {
                TreeNodeType::Leaf { object } => return f(object, ray),
                TreeNodeType::Branch { left, right } => {
                    let left_hit_opt = self.intersect_f_inner(ray, *left, f);
                    let right_hit_opt = self.intersect_f_inner(ray, *right, f);
                    return match (left_hit_opt, right_hit_opt) {
                        (None, None) => None,
                        (None, Some(hit)) => Some(hit),
                        (Some(hit), None) => Some(hit),
                        (Some(lhit), Some(rhit)) => {
                            if lhit.time < rhit.time {
                                Some(lhit)
                            }
                            else {
                                Some(rhit)
                            }
                        },
                    }
                }
            }
        }

        None
    }

    #[must_use]
    pub fn intersect_f_idx<F>(&self, ray: &Ray, f: F) -> Option<(Hit, T)>
    where F: Fn(&T, &Ray) ->  Option<(Hit, T)> + Copy {
        if self.top.is_none() { panic!("FlatBVHTree Not exist a top node") }
        self.intersect_f_idx_inner(ray, self.top.unwrap(), f)
    }

    pub fn intersect_f_idx_inner<F>(&self, ray: &Ray, index: usize, f: F) -> Option<(Hit, T)>
    where F: Fn(&T, &Ray) ->  Option<(Hit, T)> + Copy
    {
        if self.nodes.get(index).is_none() {panic!("shoulden't access node that not exsit")}

        let node = &self.nodes[index];

        if let Some(_) = node.node.intersect(ray) {
            match &node.tree_n_ty {
                TreeNodeType::Leaf { object } => return f(object, ray),
                TreeNodeType::Branch { left, right } => {
                    let left_hit_opt = self.intersect_f_idx_inner(ray, *left, f);
                    let right_hit_opt = self.intersect_f_idx_inner(ray, *right, f);
                    return match (left_hit_opt, right_hit_opt) {
                        (None, None) => None,
                        (None, Some(hit)) => Some(hit),
                        (Some(hit), None) => Some(hit),
                        (Some(lhit), Some(rhit)) => {
                            if lhit.0.time < rhit.0.time {
                                Some(lhit)
                            }
                            else {
                                Some(rhit)
                            }
                        },
                    }
                }
            }
        }

        None
    }
}
