use crate::{
    ImageData, base::ray::Ray,  scene::*
};
use crate::base::random::*;
use gk_math::color::RGB as Color;
use std::f32::MAX;
use std::sync::Arc;
use rayon::prelude::*;

pub struct Renderer {
    pub scene: Arc<Scene>,
    pub canvas: Canvas,
    pub depth: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

impl Tile {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self {
            x,
            y,
            w,
            h,
        }
    }
}

//  depth mean reflect number
impl Renderer {
    pub fn new(scene: Scene, canvas: Canvas, depth: usize) -> Self {
        Renderer {
            scene: Arc::new(scene),
            canvas,
            depth,
        }
    }

    //render a picture
    pub fn render(&self, sample_number: usize) -> ImageData {
        let w = self.canvas.width;
        let h = self.canvas.height;
        let mut image_buff = ImageData::new(w, h);

        for x in 0..w {
            for y in 0..h {
                let mut pixel = Color::new(1.0, 1.0, 1.0);
                let mut rng = PCG32::new();

                for _ in 0..sample_number {
                    let u = ((x as f32) + rng.rand()) / w as f32;
                    let v = ((y as f32) + rng.rand()) / h as f32;
                    let ray = self.scene.camera.get_ray(u, v);
                    pixel = pixel + Renderer::shade(&self.scene, &ray, 0, self.depth);
                }

                pixel = pixel / (sample_number as f32);

                image_buff[(x,y)] = pixel;
            }
        }

        image_buff
    }

    // pub fn multi_thread_render(&self, tile_w: usize, tile_h: usize, thread_number: usize, sample_number: usize) -> imagedata {

    //     let mut tile_queue: vecdeque<tile> = vecdeque::new();

    //     let mut x: usize = 0;
    //     let mut y: usize = 0;
    //     let mut w: usize = tile_w;
    //     let mut h: usize = tile_h;

    //     while y < self.canvas.height {
    //         while x < self.canvas.width {

    //             if  (x + w) > self.canvas.width {
    //                 w = self.canvas.width - x;
    //             }

    //             if (y + h) > self.canvas.height {
    //                 h = self.canvas.height - y;
    //             }

    //             tile_queue.push_back(tile::new(x, y, w, h));
    //             x += tile_w;
    //         }
    //         x = 0;
    //         w = tile_w;
    //         h = tile_h;
    //         y += tile_h;
    //     }

    //     let mut render_queue = vecdeque::new();

    //     let mut image_buff = imagedata::new(self.canvas.width, self.canvas.height);

    //     while !tile_queue.is_empty() {
    //         if render_queue.len() <= thread_number {
    //             let scene = self.scene.clone();
    //             let canvas = self.canvas.clone();
    //             let max_depth =  self.depth;
    //             let tile = tile_queue.pop_front().unwrap();

    //             let render_handle = thread::spawn(move || {
    //                 renderer::render_tile(&scene, canvas, tile, max_depth, sample_number)
    //             });
    //             render_queue.push_back(render_handle);
    //         } else {
    //             if let some(render_handle) = render_queue.pop_front() {
    //                 let (tile,colors):(tile,vec<color>) = render_handle.join().unwrap();
    //                 renderer::write_tile(&tile, &colors, &mut image_buff);
    //             }
    //         }
    //     }

    //     while !render_queue.is_empty() {
    //         if let some(render_handle) = render_queue.pop_front() {
    //                 let (tile,colors):(tile,vec<color>) = render_handle.join().unwrap();
    //                 renderer::write_tile(&tile, &colors, &mut image_buff);
    //         }
    //     }

    //     image_buff
    // }

    pub fn multi_thread_render(&self, tile_w: usize, tile_h: usize, thread_number: usize, sample_number: usize) -> ImageData {

        let mut tile_queue: Vec<Tile> = Vec::new();

        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut w: usize = tile_w;
        let mut h: usize = tile_h;

        while y < self.canvas.height {
            while x < self.canvas.width {

                if  (x + w) > self.canvas.width {
                    w = self.canvas.width - x;
                }

                if (y + h) > self.canvas.height {
                    h = self.canvas.height - y;
                }

                tile_queue.push(Tile::new(x, y, w, h));
                x += tile_w;
            }
            x = 0;
            w = tile_w;
            h = tile_h;
            y += tile_h;
        }

        let tiled_image: Vec<(Tile, Vec<Color>)> = tile_queue.par_iter().map(|tile| {
            let scene = self.scene.clone();
            let canvas = self.canvas.clone();
            let max_depth =  self.depth;
            Renderer::render_tile(&scene, canvas, tile.clone(), max_depth, sample_number)
        }).collect();

        let mut img_buf = ImageData::new(self.canvas.width, self.canvas.height);
        for (tile, colors) in tiled_image {
            Renderer::write_tile(&tile, &colors, &mut img_buf);
        }
        img_buf
    }

    fn render_tile(scene: &Arc<Scene>, canvas: Canvas, tile: Tile, max_depth: usize, sample_number: usize) -> (Tile, Vec<Color>){
        let w = canvas.width;
        let h = canvas.height;
        let mut result: Vec<Color> =  Vec::with_capacity(tile.h * tile.w);

        for y in tile.y..(tile.y + tile.h) {
           for x in tile.x..(tile.x + tile.w) {
               let mut pixel = Color::new(1.0, 1.0, 1.0);
               let mut rng = XorShift32::new();

               for _ in 0..sample_number {
                   let u = ((x as f32) + rng.rand()) / w as f32;
                   let v = ((y as f32) + rng.rand()) / h as f32;
                   let ray = scene.camera.get_ray(u, v);
                   pixel = pixel + Renderer::shade(&scene, &ray, 0, max_depth);
               }

               pixel = pixel / (sample_number as f32);

               result.push(pixel);
           }
       }

       (tile, result)
    }

    // fn shade(scene: &Arc<Scene>, ray: &Ray, depth: usize, max_depth: usize) -> Color {
    //     // ray tracing depth
    //     if depth < max_depth {
    //         if let Some((hit, obj_idx)) = scene.intersect(ray, 0.001, MAX) {
    //             let obj_ref = &scene.objects[obj_idx];
    //             // material
    //             if obj_ref.material < scene.material.len() {
    //                 let material = &scene.material[obj_ref.material];
    //                 let scatter = material.scatter(ray, &hit);
    //                 if beat_y(scatter.attenuation) < XorShift32::new().rand() {
    //                     return Color::zero();
    //                 }
    //                 let mut l = Color::zero();
    //                 // direct light
    //                 for light in scene.lights.iter() {
    //                     let nee_ray = light.get_ray(hit.position);
    //                     if scene.intersect(&nee_ray, 0.001, MAX).is_none() {
    //                         l = l + light.radiation(&nee_ray);
    //                     }
    //                 }
    //                 return l + scatter.attenuation * Renderer::shade(scene, &scatter.scattered, depth + 1, max_depth);
    //             }
    //             else {
    //                 return Color::zero();
    //             }
    //         } else {
    //             let mut l = scene.skybox.ambient(ray);
    //             // let mut l = Color::zero();
    //             for light in scene.lights.iter() {
    //                 l = l + light.radiation(ray);
    //             }
    //             return l;
    //         }
    //     }
    //     else {
    //         Color::zero()
    //     }
    // }

    fn shade(scene: &Arc<Scene>, ray: &Ray, depth: usize, max_depth: usize) -> Color {
        let mut current_ray = *ray;
        let mut a_l = Vec::with_capacity(max_depth);
        for _ in depth..max_depth {
            if let Some((hit, obj_idx)) = scene.intersect(&current_ray, 0.001, MAX) {
                let obj_ref = &scene.objects[obj_idx];
                // material
                if obj_ref.material < scene.material.len() {
                    let material = &scene.material[obj_ref.material];
                    let scatter = material.scatter(&current_ray, &hit);
                    if beat_y(scatter.attenuation) < XorShift32::new().rand() {
                        return Color::zero();
                    }
                    current_ray = scatter.scattered;
                    let mut l = material.emit(&current_ray);
                    // direct light
                    for light in scene.lights.iter() {
                        let nee_ray = light.get_ray(hit.position);
                        if scene.intersect(&nee_ray, 0.001, MAX).is_none() {
                            l = l + light.radiation(&nee_ray);
                        }
                    }
                    a_l.push((scatter.attenuation,l));
                } else {
                    return Color::zero();
                }
            }
            else{
                let mut l = scene.skybox.ambient(ray);
                for light in scene.lights.iter() {
                    l = l + light.radiation(ray);
                }
                a_l.reverse();
                for (attenuation, li) in a_l.iter() {
                    l = l * *attenuation + *li;
                }
                return l;
            }
        }

        return Color::zero();
    }

    fn write_tile(tile: &Tile, colors: &Vec<Color>, img_buf: &mut ImageData) {
        for y in 0..tile.h {
            for x in 0..tile.w {
                img_buf[((x + tile.x), (tile.y + y))] = colors[y * tile.w + x];
            }
        }
    }
}

#[inline]
fn beat_y(beta: Color) -> f32 {
    0.2126 * beta.r+0.7152 * beta.g+0.0722 * beta.b
}
