pub enum ToneMapping {
    Linear,
}

use rmu::vector::Color;

impl ToneMapping {
    pub fn tone_mapping(&self, color: Color) -> [u8;3] {
        match &self {
            ToneMapping::Linear => {
                let r = (255f32 * linear_map(color.x)) as u8;
                let g = (255f32 * linear_map(color.y)) as u8;
                let b = (255f32 * linear_map(color.z)) as u8;
                [r,g,b]
            },
        }
    }
}

fn linear_map(a: f32) -> f32 {
    if a > 1.0 {
        1.0
    } else if a < 0.0 {
        0.0
    } else {
        a
    }
}