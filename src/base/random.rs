const PCG32_DEFAULT_STATE: u64 =  0xcafef00dd15ea5e5;
const PCG32_DEFAULT_INC: u64 = 0xa02bdbf7bb3c0a7;
const MULT: u64 = 6364136223846793005;

// random number generate
#[derive(Copy,Clone)]
pub struct PCG32 {
    state: u64,
    inc: u64,
}

impl PCG32 {
    pub fn new() -> Self {
        Self {
            state: PCG32_DEFAULT_STATE,
            inc: PCG32_DEFAULT_INC,
        }
    }

    pub fn uniform_u32(&mut self) -> u32 {
        let old_state = self.state;
        
        self.state = self.state
                         .wrapping_mul(MULT)
                         .wrapping_add(self.inc);

        let rot = (old_state >> 59) as u32;
        let xsh = (((old_state >> 18) ^ old_state) >> 17) as u32;

        xsh.rotate_right(rot)
    } 

    pub fn rand(&mut self) -> f32 {
        self.uniform_u32() as f32 * (1.0 / 4294967296.0)
    }
}

pub struct XorShift32 {
    state: u32,
}

use std::time::SystemTime;

impl XorShift32 {
    pub fn new() -> Self {
        let state = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_micros();
        Self {
            state
        }
    }

    pub fn uniform_u32(&mut self) -> u32 {
        let mut x = self.state;

        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 15;

        self.state = x;

        x
    }

    pub fn rand(&mut self) -> f32 {
        self.uniform_u32() as f32 * (1.0 / 4294967296.0)
    }
}

pub enum RNG  {
    PCG32(PCG32),
    XorShift32(XorShift32),
}

impl RNG {
    pub fn new() -> Self {
        RNG::XorShift32(XorShift32::new())
    }

    pub fn rand(&mut self) -> f32 {
        match self {
            Self::PCG32(rng) => rng.rand(),
            Self::XorShift32(rng) => rng.rand(),
        }
    }
}