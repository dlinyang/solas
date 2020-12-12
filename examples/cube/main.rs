use solas::format::{image_file::ImageFile};
use solas::object::{Mesh,Sphere};
use solas::scene::{Scene,Canvas};
use solas::base::camera::Camera;
use solas::base::light::PointLight;
use solas::material::bsdf::*;
use solas::renderer::*;
use rmu::vector::Vector3;

use std::sync::Arc;

fn main() {

    println!("This is a Ray Tracing program");

    let w: usize = 800;
    let h: usize = 400;
    let s: usize = 100;
    
    let canvas = Canvas::new(w, h, 1.0, 2.0);

    let look_from = Vector3::new(10.0, 10.0, 3.0);
    let look_at   = Vector3::new(0.0, 0.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let camera = Camera::new(look_from,
                             look_at,
                             Vector3::new(0.0, 0.0, 1.0), 20.0,  w as f32/ h as f32, 0.1, dist_to_focus);
    
    let cube = Mesh::cube("cube".into(),"red".into(), 2.0);
    let sphere = Sphere::new("sphere".into(),"blue".into())
        .with_radius(1000.0)
        .with_center(Vector3::new(0.0, 0.0, -1001.0));

    let mut blue = Lambertian::new("blue".into());
    blue.albedo = Vector3::new(0.1, 0.2, 0.5);

    let mut red = Lambertian::new("red".into());
    red.albedo = Vector3::new(0.5, 0.2, 0.1);

    let light = PointLight::create(Vector3::new(5f32, 5f32, 5f32),Vector3::new(1.0, 1.0, 1.0), 1.0 );

    let mut scene = Scene::new();

    scene.add_object(Arc::new(cube));
    scene.add_object(Arc::new(sphere));

    scene.add_material(Arc::new(blue));
    scene.add_material(Arc::new(red));

    scene.set_camera(camera);

    scene.add_light(Arc::new(light));

    let renderer = Renderer::new(scene, canvas, s);

    let start = std::time::Instant::now();
    let image = renderer.multi_thread_render(64, 64, 16, 50);

    let end = std::time::Instant::now();
    println!("coast time: {:?}", end.duration_since(start));
    
    let mut file = ImageFile::new("image_out/cube.png");
    file.add_image_buff(image);
    file.write_rgba();
}