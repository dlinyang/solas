// lower left corner coordinate
#[derive(Copy,Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub vertical: f32,
    pub horizontal: f32,
}

impl Canvas {
    pub fn new(width: usize,
               height: usize,
               vertical: f32,
               horizontal: f32) -> Self{
                   Canvas{
                       width,
                       height,
                       vertical,
                       horizontal,
                   }
               }
}