use crate::{
    base::{ray::Ray, tone_mapping::*},
    format::image_buff::ImageBuff,
    scene::*,
};
use rand::prelude::*;
use rmu::vector::{Color, Vector3};
use std::f32::MAX;
use std::sync::Arc;
use std::collections::VecDeque;
use std::thread;

pub struct Renderer {
    pub scene: Arc<Scene>,
    pub canvas: Canvas,
    pub depth: usize,
    pub tone_mapping: ToneMapping,
}

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
            tone_mapping: ToneMapping::Linear,
        }
    }

    //render a picture
    pub fn render(&self, sample_number: usize) -> ImageBuff {
        let w = self.canvas.width;
        let h = self.canvas.height;
        let mut image_buff = ImageBuff::create(w, h);

        for x in 0..w {
            for y in 0..h {
                let mut pixel = Color::default();
                let mut rng = rand::thread_rng();

                for _ in 0..sample_number {
                    let u = ((x as f32) + rng.gen_range(0f32, 1f32)) / w as f32;
                    let v = ((y as f32) + rng.gen_range(0f32, 1f32)) / h as f32;
                    let ray = self.scene.camera.get_ray(u, v);
                    pixel = pixel + Renderer::shade(&self.scene, &ray, 0, self.depth);
                }

                pixel = pixel / (sample_number as f32);

                let [r, g, b] = self.tone_mapping.tone_mapping(pixel);
                image_buff.data[y * w + x] =u32::from_ne_bytes([r, g, b, 255]);
            }
        }

        image_buff
    }

    pub fn multi_thread_render(&self, tile_w: usize, tile_h: usize, thread_number: usize, sample_number: usize) -> ImageBuff {

        let mut tile_queue: VecDeque<Tile> = VecDeque::new();

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

                tile_queue.push_back(Tile::new(x, y, w, h));
                x += tile_w;
            }
            x = 0;
            w = tile_w;
            h = tile_h;
            y += tile_h;
        }

        let mut render_queue = VecDeque::new();

        let mut image_buff = ImageBuff::create(self.canvas.width, self.canvas.height);

        while !tile_queue.is_empty() {
            if render_queue.len() <= thread_number {
                let scene = self.scene.clone();
                let canvas = self.canvas.clone();
                let max_depth =  self.depth;
                let tile = tile_queue.pop_front().unwrap();

                let render_handle = thread::spawn(move || {
                    Renderer::render_tile(&scene, canvas, tile, max_depth, sample_number)
                });
                render_queue.push_back(render_handle);
            } else {
                if let Some(render_handle) = render_queue.pop_front() {
                    let (tile,colors):(Tile,Vec<Color>) = render_handle.join().unwrap();
                    Renderer::write_tile(tile, colors, &self.canvas, &self.tone_mapping, &mut image_buff);
                }
            }
        }

        while !render_queue.is_empty() {
            if let Some(render_handle) = render_queue.pop_front() {
                    let (tile,colors):(Tile,Vec<Color>) = render_handle.join().unwrap();
                    Renderer::write_tile(tile, colors, &self.canvas, &self.tone_mapping, &mut image_buff);
            }
        }

        image_buff
    }

    fn render_tile(scene: &Arc<Scene>, canvas: Canvas, tile: Tile, max_depth: usize, sample_number: usize) -> (Tile, Vec<Color>){
        let w = canvas.width;
        let h = canvas.height;
        let mut result: Vec<Color> =  Vec::new();

        for y in tile.y..(tile.y + tile.h) {
           for x in tile.x..(tile.x + tile.w) {
               let mut pixel = Color::default();
               let mut rng = rand::thread_rng();

               for _ in 0..sample_number {
                   let u = ((x as f32) + rng.gen_range(0f32, 1f32)) / w as f32;
                   let v = ((y as f32) + rng.gen_range(0f32, 1f32)) / h as f32;
                   let ray = scene.camera.get_ray(u, v);
                   pixel = pixel + Renderer::shade(&scene,&ray, 0, max_depth);
               }

               pixel = pixel / (sample_number as f32);

               result.push(pixel);
           }
       }

       (tile, result)
    }

    fn shade(scene: &Arc<Scene>, ray: &Ray, depth: usize, max_depth: usize) -> Color {
        // ray tracing depth
        if depth < max_depth {
            if let Some(hit) = scene.intersect(ray, 0.001, MAX) {
                // material
                if let Some(material) = scene.material.get(&hit.material_name) {
                    let scatter = material.scatter(ray, &hit);
                    return scatter.attenuation * Renderer::shade(scene, &scatter.scattered, depth + 1, max_depth);
                }
            }
        }
        let mut attenuation = scene.skybox.ambient(ray);

        for light in scene.lights.iter() {
            attenuation = attenuation + light.radiation(ray);
        }
        
        attenuation
    }

    fn write_tile(tile: Tile, colors: Vec<Color>, canvas: &Canvas, tone_mapping: &ToneMapping, image_buff: &mut ImageBuff) {
        let w = canvas.width;
        for y in 0..tile.h {
            for x in 0..tile.w {
                let [r, g, b] = tone_mapping.tone_mapping(colors[y * tile.w + x]);
                image_buff.data[(y + tile.y) * w + (x + tile.x)] =u32::from_ne_bytes([r, g, b, 255]);
            }
        }
    }
}
