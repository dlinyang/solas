use solas::base::object::ObjectTransfrom;
use solas::object::{Mesh,Sphere};
use solas::scene::{Scene,Canvas};
use solas::base::camera::Camera;
use solas::base::light::PointLight;
use solas::material::bsdf::*;
use solas::renderer::*;
use gk_math::base::f32::Vec3;


fn main() {

    println!("This is a Ray Tracing program");

    let w: usize = 800;
    let h: usize = 400;
    let s: usize = 64;

    let canvas = Canvas::new(w, h, 1.0, 2.0);

    let look_from = Vec3::new(10.0, 10.0, 3.0);
    let look_at   = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let camera = Camera::new(look_from,
                             look_at,
                             Vec3::new(0.0, 0.0, 1.0), 20.0,  w as f32/ h as f32, 0.1, dist_to_focus);


    let mut scene = Scene::new();

    let material_idx = scene.add_material(Dielectric::new());
    let mut cube_mesh = Mesh::cube(2.0);
    cube_mesh.moved(Vec3 { x: -3.0, y: 0.0, z: 0.0 });
    let cube =scene.add_object(cube_mesh);
    cube.material = material_idx;

    let material_idx = scene.add_material(
        Lambertian::new()
            .with_albedo(Vec3::new(0.1, 0.2, 0.8)));
    let plane = scene.add_object(
        solas::object::Plane::new()
            .with_center(Vec3::new(0.0, 0.0, -5.0))
            .with_normal(Vec3::new(0.0, 0.0, 1.0))
            .with_width(100.0)
            .with_height(100.0));
    plane.material = material_idx;

    let material_idx = scene.add_material(
        Metal::new()
        .with_albedo(Vec3::new(0.8, 0.6, 0.2))
        .with_fuzz(0.02));
    let obj = scene.add_object(Mesh::load_obj("test.obj").unwrap());
    obj.material = material_idx;

    let light = PointLight::create(Vec3::new(5f32, 5f32, 5f32),Vec3::new(1.0, 1.0, 1.0).into(), 40.0 );

    scene.set_camera(camera);

    scene.add_light(light);

    scene.build_bvh_tree();

    let renderer = Renderer::new(scene, canvas, s);

    let start = std::time::Instant::now();
    let image = renderer.multi_thread_render(64, 64, 16, 32);

    let end = std::time::Instant::now();
    println!("coast time: {:?}", end.duration_since(start));

    image.write_to_file(solas::tone_mapping::ToneMapping::Linear, "image_out/cube.png").unwrap();
}
