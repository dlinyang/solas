use std::ops::{Index, IndexMut};
use std::path::Path;

use gk_math::color::RGB as Color;

use crate::tone_mapping::ToneMapping;

pub struct ImageData {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl ImageData {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            width: w,
            height: h,
            data: vec![Color::zero(); w * h],
        }
    }

    //create a pure color image
    pub fn create_with_a_color(w: usize, h: usize, color: Color) -> Self {
        Self {
            width: w,
            height: h,
            data: vec![color; w * h],
        }
    }

    pub fn write_to_file(&self, tone_mapping: ToneMapping, path: impl AsRef<Path>) -> Result<(), String> {
        let mut img_buf = image::ImageBuffer::new(self.width as u32, self.height as u32);
        for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
            let data = tone_mapping.tone_mapping(self[(x as usize,y as usize)]);
            *pixel = image::Rgb(data);
        }

        if let Some(parent) = path.as_ref().parent() {
            if !parent.exists() {
                let dir_builder = std::fs::DirBuilder::new();
                dir_builder.create(parent).unwrap()
            }
        }
        img_buf.save(path).map_err(|err|  err.to_string())?;
        Ok(())
    }

    pub fn raw(&mut self) -> *mut Color {
        self.data.as_mut_ptr()
    }
}

impl Index<(usize, usize)> for ImageData {
    type Output = Color;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        let (x, y) = idx;
        &self.data[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for ImageData {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        let (x, y) = idx;
        &mut self.data[y * self.width + x]
    }
}
