use crate::{base::ray::Ray, base::intersect::*, base::material::* ,base::camera::Camera, base::light::Light };
use crate::material::background::*;
use crate::base::object::Object;
use crate::base::bound::*;
use crate::base::bvh::BVHTree;

use std::sync::Arc;
use std::collections::HashMap;

pub struct Scene {
    pub objects: HashMap<String,Arc<dyn Object + Sync + Send>>,
    pub bvh_tree: Option<BVHTree<AABB>>, // object contain bvh tree
    pub material: HashMap<String,Arc<dyn Material + Sync + Send>>,
    pub skybox: Arc<dyn SkyBox + Sync + Send>,
    pub lights: Vec<Arc<dyn Light + Sync + Send>>,
    pub camera: Camera,
}

impl Scene {

    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            bvh_tree: None,
            material: HashMap::new(),
            skybox: Arc::new(Background::new()),
            lights: Vec::new(),
            camera: Camera::default(),
        }
    }

    pub fn add_object(&mut self, object: Arc<dyn Object + Send + Sync>) {

        if let None = self.bvh_tree  {
            self.bvh_tree = Some(BVHTree::new(object.get_aabb(), Some(object.name())));
        } else {
            let bvh_tree = self.bvh_tree.take().unwrap();
            let node = object.get_aabb();
            let mut parent = BVHTree::new(Bound::sorround(&bvh_tree.node, &node), None);
            parent.left = Some(Arc::new(bvh_tree));
            parent.right = Some(Arc::new(BVHTree::new(node, Some(object.name()))));
            self.bvh_tree = Some(parent);
        }

        self.objects.insert(object.name(),object);
    }

    pub fn add_material(&mut self, material: Arc<dyn Material + Send + Sync>) {
        self.material.insert(material.name(),material);
    }

    pub fn add_light(&mut self,light: Arc<dyn Light + Send + Sync>) {
        self.lights.push(light);
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut result: Option<Hit> = None;

        if let Some(bvh_tree) = &self.bvh_tree {
            let hitable_list = bvh_tree.intersect(&ray);

            for name in hitable_list {
                if let Some(object) = self.objects.get(&name) {
                    if let Some(hit) = object.intersect(&ray, t_min, t_max) {
                        if let Some(hit_temp) = &&result {
                            if hit.time < hit_temp.time {
                                result = Some(hit)
                            }
                        } else {
                            result = Some(hit)
                        }

                    }
                }
            }
        }

        result
    }
}