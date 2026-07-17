use solas::object::sphere::Sphere;
use solas::scene::{Scene,Canvas};
use solas::base::camera::Camera;
use solas::material::bsdf::*;
use solas::renderer::*;
use gk_math::base::f32::Vec3;

use std::sync::Arc;

use solas::base::random::RNG;

fn main() {

    println!("This is a Ray Tracing program");

    let w: usize = 1920;
    let h: usize = 1080;
    let s: usize = 10;
    let canvas = Canvas::new(w, h, 1.0, 2.0);

    let look_from = Vec3::new(-8.0, 5.0, 0.0);
    let look_at   = Vec3::new(2.5, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let camera = Camera::new(look_from,
                             look_at,
                             Vec3::new(0.0, 0.0, 1.0), 20.0,  w as f32/ h as f32, 0.1 , dist_to_focus);

    let sphere0 = Sphere::new("sphere0".into(),"material0".into())
        .with_radius(0.5)
        .with_center(Vec3::new(2.5, -1.0, 0.0));

    let sphere1 = Sphere::new("sphere1".into(),"material0".into())
        .with_radius(1000.0)
        .with_center(Vec3::new(2.5, -1.0, -1000.5));

    let sphere2 = Sphere::new("sphere2".into(),"material1".into())
        .with_radius(0.5)
        .with_center(Vec3::new(1.5, -1.0, 0.0));

    let sphere3 = Sphere::new("sphere3".into(),"material2".into())
        .with_radius(0.5)
        .with_center(Vec3::new( 3.5, -1.0, 0.0));

    let mut material0 = Lambertian::new("material0");
    material0.albedo = Vec3::new(0.1, 0.2, 0.5);
    let mut material1 = Metal::new("material1");
    material1.albedo=  Vec3::new(0.8, 0.6, 0.2);
    material1.fuzz = 0.02;
    let mut material2 = Dielectric::new("material2");
    material2.refract_coe = 1.5;
    let mut scene = Scene::new();
    let mut rng = RNG::new();

    for x in 0..20 {
        let choose: f32 = rng.rand();
        let center = Vec3::new(5.0 * rng.rand(), 4.0 * rng.rand(), -0.2);
        if choose < 0.8 {
            let mut material = Lambertian::new(format!("material{}",x+3));
            material.albedo = Vec3::new(rng.rand() * rng.rand(),
                                           rng.rand() * rng.rand(),
                                           rng.rand() * rng.rand());
            let sphere = Sphere::new(format!("sphere{}", x + 4),format!("material{}", x + 3))
                .with_radius(0.2)
                .with_center(center);
            scene.add_object(Arc::new(sphere));
            scene.add_material(material);
        }
        else if choose < 0.95 {
            let mut material = Metal::new(format!("material{}",x+3));
            material.albedo = Vec3::new(rng.rand(),
                                           rng.rand(),
                                           rng.rand());
            material.fuzz = 0.5 * rng.rand();
            let sphere = Sphere::new(format!("sphere{}", x + 4),format!("material{}", x + 3))
               .with_radius(0.2)
               .with_center(center);
            scene.add_object(Arc::new(sphere));
            scene.add_material(material);
        }
        else {
            let mut material = Dielectric::new(format!("material{}",x+3));
            material.refract_coe = 2.0 * rng.rand();
            let sphere = Sphere::new(format!("sphere{}", x + 4),format!("material{}", x + 3))
               .with_radius(0.2)
               .with_center(center);
            scene.add_object(Arc::new(sphere));
            scene.add_material(material);
        }
    }
    scene.add_object(Arc::new(sphere0));
    scene.add_object(Arc::new(sphere1));
    scene.add_object(Arc::new(sphere2));
    scene.add_object(Arc::new(sphere3));
    scene.add_material(material0);
    scene.add_material(material1);
    scene.add_material(material2);
    scene.set_camera(camera);
    scene.build_bvh_tree();
    let renderer = Renderer::new(scene, canvas, s);

    let start = std::time::Instant::now();
    let image = renderer.multi_thread_render(64, 64, 16, 10);

    let end = std::time::Instant::now();
    println!("coast time: {:?}", end.duration_since(start));

    image.write_to_file(solas::tone_mapping::ToneMapping::Reinhard, "image_out/sphere.png").unwrap();
}
