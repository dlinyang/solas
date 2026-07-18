use crate::bound::AABB;
use crate::{base::ray::Ray, base::intersect::*, base::material::* ,base::camera::Camera, base::light::Light };
use crate::material::background::*;
use crate::base::object::{Object, ObjectBase};
// use crate::base::bound::*;
use crate::base::bvh::BVHTree;

use std::sync::Arc;

pub type MaterialARef = Arc<dyn Material + Sync + Send>;

pub struct Scene {
    pub objects: Vec<Object>,
    pub bvh_tree: Option<BVHTree<AABB,usize>>, // object contain bvh tree
    pub material: Vec<MaterialARef>,
    pub skybox: Arc<dyn SkyBox + Sync + Send>,
    pub lights: Vec<Arc<dyn Light + Sync + Send>>,
    pub camera: Camera,
}

impl Scene {

    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bvh_tree: None,
            material: Vec::new(),
            skybox: Arc::new(Background::new()),
            lights: Vec::new(),
            camera: Camera::default(),
        }
    }

    pub fn add_object(&mut self, shape: impl ObjectBase + Sync + Send + 'static) -> &mut Object {
        self.objects.push(Object { idx: self.objects.len(), material: 0, base: Box::new(shape) });
        self.objects.last_mut().unwrap()
    }

    pub fn build_bvh_tree(&mut self) {
        let boxes: Vec<_> = self.objects.iter().enumerate().map(|(idx, obj)| (idx, obj.base.get_aabb())).collect();
        self.bvh_tree = Some(BVHTree::sah_build(boxes));
    }

    /// the material buffer vec
    /// - return the material index and ref
    pub fn add_material(&mut self, material: impl Material + Send + Sync + 'static) -> (usize, MaterialARef) {
        let material = Arc::new(material);
        self.material.push(material.clone());
        (self.material.len(), material.clone())
    }

    pub fn add_light(&mut self,light: impl Light + Send + Sync + 'static) {
        self.lights.push(Arc::new(light));
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(Hit, usize)> {
        if let Some(bvh_tree) = &self.bvh_tree {
            bvh_tree.intersect_f_idx(ray, |idx, ray| {
                if let Some(object) = self.objects.get(*idx) {
                    object.base.intersect(&ray, t_min, t_max).map(|hit| (hit, *idx))
                } else {
                    None
                }
            })
        }
        else {
            None
        }
    }
}
