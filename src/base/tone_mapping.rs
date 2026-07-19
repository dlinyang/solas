use gk_math::color::RGB as Color;

pub enum ToneMapping {
    Linear,
    Reinhard,
}

impl ToneMapping {
    pub fn tone_mapping(&self, color: Color) -> [u8;3] {
        let [r, g, b] = match &self {
            ToneMapping::Linear => {
                [linear_map(color.r), linear_map(color.g), linear_map(color.b)]
            },
            ToneMapping::Reinhard => {
                let color = reinhard(color);
                [color.r, color.g, color.b]
            }
        };

        let r = (255f32 * r) as u8;
        let g = (255f32 * g) as u8;
        let b = (255f32 * b) as u8;
        [r,g,b]
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

fn reinhard(color: Color) -> Color {
    color / (color + Color::one())
}
