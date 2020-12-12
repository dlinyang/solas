use image;
use super::image_buff::ImageBuff;

pub struct ImageFile{
    pub name: String,
    pub image_buff: Option<ImageBuff>,
}

impl ImageFile {
    pub fn new(name: &str) -> Self{
        ImageFile{
            name: String::from(name),
            image_buff: None,
        }
    }

    pub fn add_image_buff(&mut self,imgae_buff: ImageBuff) {
        self.image_buff = Some(imgae_buff);
    }

    pub fn write_rgba(&self){
        match &self.image_buff {
            Some(color) => {
                let mut imgbuff = image::ImageBuffer::new(color.width as u32, color.height as u32);
                for (x,y,pixel) in imgbuff.enumerate_pixels_mut() {
                    let rgba = color.data[(y as usize) * color.width + (x as usize)].to_ne_bytes();
                    *pixel = image::Rgba(rgba);
                }
                imgbuff.save(&self.name).unwrap();
            },
            None => () ,
        }
    }
}