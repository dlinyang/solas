pub struct ImageBuff{
    pub width : usize,
    pub height : usize,
    pub data : Vec<u32>,    //RGBA value
}

impl ImageBuff {
    pub fn new(w: usize, h: usize, d: &[u32]) -> Self {
        ImageBuff { 
            width: w,
            height: h,
            data : d.to_vec(),
        }
    }

    //create a empty image
    pub fn create(w: usize,h: usize) -> Self{
        let mut data: Vec<u32> = Vec::new();

        for _i in 0..w * h {
            data.append(&mut vec![0]);
        }

        ImageBuff {
            width: w,
            height: h,
            data,
        }
    }

    //create a pure color image 
    pub fn create_with_a_color(w: usize, h: usize,color: u32) -> Self{
        let mut data: Vec<u32> = Vec::new();

        for _i in 0..w * h {
            data.append(&mut vec![color]);
        }

        ImageBuff {
            width: w,
            height: h,
            data,
        }
    }

    pub fn to_rgb_raw(&self,content: &mut Vec<u8>){
        for x in 0..self.width * self.height {
            let pixel = self.data[x].to_ne_bytes();
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            content.append(&mut vec![r,g,b]);
        }
    }
}