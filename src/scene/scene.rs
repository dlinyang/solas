use crate::bound::AABB;
use crate::{base::ray::Ray, base::intersect::*, base::material::* ,base::camera::Camera, base::light::Light };
use crate::material::background::*;
use crate::base::object::{Object, ObjectBase};
// use crate::base::bound::*;
use crate::base::bvh::BVHTree;

pub type MaterialARef = Box<dyn Material + Sync + Send>;

pub struct Scene {
    pub objects: Vec<Object>,
    pub bvh_tree: Option<BVHTree<AABB,usize>>, // object contain bvh tree
    pub material: Vec<MaterialARef>,
    pub skybox: Box<dyn SkyBox + Sync + Send>,
    pub lights: Vec<Box<dyn Light + Sync + Send>>,
    pub camera: Camera,
}

impl Scene {

    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bvh_tree: None,
            material: Vec::new(),
            skybox: Box::new(Background::new()),
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
    pub fn add_material(&mut self, material: impl Material + Send + Sync + 'static) -> usize {
        let material = Box::new(material);
        self.material.push(material);
        return self.material.len() - 1
    }

    pub fn add_light(&mut self,light: impl Light + Send + Sync + 'static) {
        self.lights.push(Box::new(light));
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
