use solas::format::{image_file::ImageFile};
use solas::object::sphere::Sphere;
use solas::scene::{Scene,Canvas};
use solas::base::camera::Camera;
use solas::material::bsdf::*;
use solas::renderer::*;
use rmu::vector::Vector3;

use std::sync::Arc;

use rand::prelude::*;

fn main() {

    println!("This is a Ray Tracing program");

    let w: usize = 800;
    let h: usize = 400;
    let s: usize = 100;
    let canvas = Canvas::new(w, h, 1.0, 2.0);

    let look_from = Vector3::new(-8.0, 5.0, 0.0);
    let look_at   = Vector3::new(2.5, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let camera = Camera::new(look_from,
                             look_at,
                             Vector3::new(0.0, 0.0, 1.0), 20.0,  w as f32/ h as f32, 0.1 , dist_to_focus);
    
    let sphere0 = Sphere::new("sphere0".into(),"material0".into())
        .with_radius(0.5)
        .with_center(Vector3::new(2.5, -1.0, 0.0));

    let sphere1 = Sphere::new("sphere1".into(),"material0".into())
        .with_radius(1000.0)
        .with_center(Vector3::new(2.5, -1.0, -1000.5));

    let sphere2 = Sphere::new("sphere2".into(),"material1".into())
        .with_radius(0.5)
        .with_center(Vector3::new(1.5, -1.0, 0.0));

    let sphere3 = Sphere::new("sphere3".into(),"material2".into())
        .with_radius(0.5)
        .with_center(Vector3::new( 3.5, -1.0, 0.0));

    let mut material0 = Lambertian::new("material0".into());
    material0.albedo = Vector3::new(0.1, 0.2, 0.5);
    let mut material1 = Metal::new("material1".into());
    material1.albedo=  Vector3::new(0.8, 0.6, 0.2);
    material1.fuzz = 0.02;
    let mut material2 = Dielectric::new("material2".into());
    material2.refract_coe = 1.5;
    let mut scene = Scene::new();
    let mut rng = rand::thread_rng();

    for x in 0..20 {
        let choose: f32 = rng.gen();
        let center = Vector3::new(5.0 * rng.gen::<f32>(), 4.0 * rng.gen::<f32>(), -0.2);
        if choose < 0.8 {
            let mut material = Lambertian::new(format!("material{}",x+3));
            material.albedo = Vector3::new(rng.gen::<f32>() * rng.gen::<f32>(),
                                           rng.gen::<f32>() * rng.gen::<f32>(),
                                           rng.gen::<f32>() * rng.gen::<f32>());
            let sphere = Sphere::new(format!("sphere{}", x + 4),format!("material{}", x + 3))
                .with_radius(0.2)
                .with_center(center);
            scene.add_object(Arc::new(sphere));
            scene.add_material(Arc::new(material));
        }
        else if choose < 0.95 {
            let mut material = Metal::new(format!("material{}",x+3));
            material.albedo = Vector3::new(rng.gen(),
                                           rng.gen(),
                                           rng.gen());
            material.fuzz = 0.5 * rng.gen::<f32>();
            let sphere = Sphere::new(format!("sphere{}", x + 4),format!("material{}", x + 3))
               .with_radius(0.2)
               .with_center(center);
            scene.add_object(Arc::new(sphere));
            scene.add_material(Arc::new(material));
        }
        else {
            let mut material = Dielectric::new(format!("material{}",x+3));
            material.refract_coe = 2.0 * rng.gen::<f32>();
            let sphere = Sphere::new(format!("sphere{}", x + 4),format!("material{}", x + 3))
               .with_radius(0.2)
               .with_center(center);
            scene.add_object(Arc::new(sphere));
            scene.add_material(Arc::new(material));
        }
    }
    scene.add_object(Arc::new(sphere0));
    scene.add_object(Arc::new(sphere1));
    scene.add_object(Arc::new(sphere2));
    scene.add_object(Arc::new(sphere3));
    scene.add_material(Arc::new(material0));
    scene.add_material(Arc::new(material1));
    scene.add_material(Arc::new(material2));
    scene.set_camera(camera);
    let renderer = Renderer::new(scene, canvas, s);

    let start = std::time::Instant::now();
    let image = renderer.multi_thread_render(64, 64, 16, 50);

    let end = std::time::Instant::now();
    println!("coast time: {:?}", end.duration_since(start));
    
    let mut file = ImageFile::new("image_out/sphere.png");
    file.add_image_buff(image);
    file.write_rgba();
}