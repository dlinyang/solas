use solas::object::{Mesh,Sphere};
use solas::scene::{Scene,Canvas};
use solas::base::camera::Camera;
use solas::base::light::PointLight;
use solas::material::bsdf::*;
use solas::renderer::*;
use gk_math::base::f32::Vec3;

use std::sync::Arc;

fn main() {

    println!("This is a Ray Tracing program");

    let w: usize = 800;
    let h: usize = 400;
    let s: usize = 10;

    let canvas = Canvas::new(w, h, 1.0, 2.0);

    let look_from = Vec3::new(10.0, 10.0, 3.0);
    let look_at   = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let camera = Camera::new(look_from,
                             look_at,
                             Vec3::new(0.0, 0.0, 1.0), 20.0,  w as f32/ h as f32, 0.1, dist_to_focus);

    let cube = Mesh::cube("cube","red", 2.0);
    let sphere = Sphere::new("sphere".into(),"blue".into())
        .with_radius(1000.0)
        .with_center(Vec3::new(0.0, 0.0, -1001.0));
    let mut obj = Mesh::load_obj("test.obj").unwrap();
    obj.material = "metal".into();

    let mut blue = Lambertian::new("blue");
    blue.albedo = Vec3::new(0.1, 0.2, 0.5);

    let mut red = Lambertian::new("red");
    red.albedo = Vec3::new(0.5, 0.2, 0.1);

    let mut metal = Metal::new("metal");
    metal.albedo=  Vec3::new(0.8, 0.6, 0.2);
    metal.fuzz = 0.02;

    let light = PointLight::create(Vec3::new(5f32, 5f32, 5f32),Vec3::new(1.0, 1.0, 1.0).into(), 1.0 );


    let mut scene = Scene::new();

    scene.add_object(Arc::new(cube));
    scene.add_object(Arc::new(sphere));
    scene.add_object(Arc::new(obj));

    scene.add_material(blue);
    scene.add_material(red);
    scene.add_material(metal);

    scene.set_camera(camera);

    scene.add_light(Arc::new(light));
    scene.build_bvh_tree();

    let renderer = Renderer::new(scene, canvas, s);

    let start = std::time::Instant::now();
    let image = renderer.multi_thread_render(64, 64, 16, 10);

    let end = std::time::Instant::now();
    println!("coast time: {:?}", end.duration_since(start));

    image.write_to_file(solas::tone_mapping::ToneMapping::Reinhard, "image_out/cube.png").unwrap();
}
