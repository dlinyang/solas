use solas::object::sphere::Sphere;
use solas::scene::{Scene,Canvas};
use solas::base::camera::Camera;
use solas::material::bsdf::*;
use solas::renderer::*;
use solas::light::PointLight;
use solas::base::random::RNG;
use gk_math::base::f32::Vec3;

fn main() {

    println!("This is a Ray Tracing program");

    let w: usize = 1920;
    let h: usize = 1080;
    let s: usize = 64;
    let canvas = Canvas::new(w, h, 1.0, 2.0);

    let look_from = Vec3::new(-8.0, 5.0, 0.0);
    let look_at   = Vec3::new(2.5, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let camera = Camera::new(look_from,
                             look_at,
                             Vec3::new(0.0, 0.0, 1.0), 20.0,  w as f32/ h as f32, 0.1 , dist_to_focus);

    let mut scene = Scene::new();
    scene.set_camera(camera);

    let material_idx = scene.add_material(Lambertian::new().with_albedo(Vec3::new(0.1, 0.2, 0.5)));
    let sphere = scene.add_object(
        Sphere::new()
        .with_radius(0.5)
        .with_center(Vec3::new(2.5, -1.0, 0.0))
    );
    sphere.material = material_idx;

    let material_idx = scene.add_material(
        Metal::new()
            .with_albedo(Vec3::new(0.8, 0.6, 0.2))
            .with_fuzz(0.02));
    let sphere = scene.add_object(
        Sphere::new()
        .with_radius(1000.0)
        .with_center(Vec3::new(2.5, -1.0, -1000.5)));
    sphere.material = material_idx;

    let material_idx = scene.add_material(Dielectric::new().with_refract_coe(1.5));
    let sphere = scene.add_object(
        Sphere::new()
        .with_radius(0.5)
        .with_center(Vec3::new(1.5, -1.0, 0.0)));
    sphere.material = material_idx;

    let sphere = scene.add_object(Sphere::new()
        .with_radius(0.5)
        .with_center(Vec3::new( 3.5, -1.0, 0.0)));
    sphere.material = material_idx;

    let mut rng = RNG::new();
    for _ in 0..100{
        let choose: f32 = rng.rand();
        let center = Vec3::new(5.0 * rng.rand(), 4.0 * rng.rand(), -0.2);
        if choose < 0.8 {
            let material_idx = scene.add_material(
                Lambertian::new()
                    .with_albedo(Vec3::new(rng.rand() * rng.rand(),
                                           rng.rand() * rng.rand(),
                                           rng.rand() * rng.rand())));
            let sphere = scene.add_object(
                Sphere::new()
                    .with_radius(0.2)
                    .with_center(center));
            sphere.material = material_idx;
        }
        else if choose < 0.95 {
            let material_idx = scene.add_material(Metal::new()
                .with_albedo(Vec3::new(rng.rand(),rng.rand(),rng.rand()))
                .with_fuzz(0.5 * rng.rand()));
            let sphere = scene.add_object(
                Sphere::new()
               .with_radius(0.2)
               .with_center(center));
            sphere.material = material_idx;
        }
        else {
            let material_idx = scene.add_material(
                Dielectric::new()
                    .with_refract_coe(2.0 * rng.rand()));
            let sphere = scene.add_object(
                Sphere::new()
               .with_radius(0.2)
               .with_center(center));
            sphere.material = material_idx;
        }
    }

    let light = PointLight::create(Vec3::new(5f32, 5f32, 100f32),Vec3::new(1.0, 1.0, 1.0).into(), 100.0 );
    scene.add_light(light);

    scene.build_bvh_tree();

    let renderer = Renderer::new(scene, canvas, s);

    let start = std::time::Instant::now();
    let image = renderer.multi_thread_render(64, 64, 16, 32);

    let end = std::time::Instant::now();
    println!("coast time: {:?}", end.duration_since(start));

    image.write_to_file(solas::tone_mapping::ToneMapping::Linear, "image_out/sphere.png").unwrap();
}
