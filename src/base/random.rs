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

pub struct XorShift32 {}

use std::{cell::RefCell, time::SystemTime};

thread_local! {
    static RNG: RefCell<u32> = RefCell::new(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_micros());
}

impl XorShift32 {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn uniform_u32(&mut self) -> u32 {
        let mut x = Default::default();

        RNG.with(|rng|{
            let mut rng = rng.borrow_mut();
            x = *rng;
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 15;
            (*rng) = x;
        });

        x
    }

    pub fn rand(&mut self) -> f32 {
        self.uniform_u32() as f32 * (1.0 / 4294967296.0)
    }
}

// pub struct Sobol {
//     dimension: usize,
//     index: u64,
//     direction_vectors: Vec<Vec<u32>>,
// }

// impl Sobol {
//     pub fn new(dimension: usize) -> Self {
//         // Initialize direction vectors using the Sobol sequence primitive polynomials
//         // For simplicity, we use a default set for low dimensions (up to 5)
//         let d = dimension.max(1);
//         let mut direction_vectors = vec![vec![0u32; 32]; d];
//         // Initialize direction vectors with standard Sobol parameters
//         // Using primitive polynomials and initial direction numbers
//         let poly: [u32; 5] = [1, 3, 7, 11, 13]; // primitive polynomials for dims 1-5
//         let init: [[u32; 32]; 5] = [
//             [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
//             [1, 3, 5, 15, 17, 51, 85, 255, 257, 771, 1285, 3855, 4369, 13107, 21845, 65535, 65537, 196611, 327685, 983055, 1114129, 3342387, 5570645, 16711935, 16843009, 50529027, 84215045, 252645135, 286331153, 858993459, 1431655765, 4294967295],
//             [7, 11, 19, 55, 91, 179, 455, 747, 1459, 3755, 6107, 12115, 30635, 50099, 98235, 248347, 407483, 794675, 2013515, 3324659, 6427891, 16317387, 26781811, 51479827, 131041043, 214505811, 410105427, 1046675219, 1712818387, 3251585731, 8321461331, 13635454355],
//             [11, 29, 47, 115, 229, 499, 1003, 2035, 4083, 8179, 16371, 32755, 65523, 131059, 262131, 524275, 1048563, 2097139, 4194291, 8388595, 16777203, 33554419, 67108851, 134217715, 268435443, 536870899, 1073741811, 2147483635, 4294967283, 8589934579, 17179869171, 34359738355],
//             [13, 37, 91, 203, 469, 1003, 2119, 4387, 9035, 18387, 37035, 74355, 149227, 298835, 598247, 1197355, 2396491, 4794795, 9591403, 19184627, 38372395, 76747923, 153499691, 307004715, 614013739, 1228032403, 2456073515, 4912157323, 9824324491, 19648660499, 39297344875, 78594713675],
//         ];

//         for i in 0..d.min(5) {
//             for j in 0..32 {
//                 direction_vectors[i][j] = init[i][j];
//             }
//         }

//         Sobol {
//             dimension: d,
//             index: 0,
//             direction_vectors,
//         }
//     }

//     pub fn uniform_u32(&mut self) -> u32 {
//         let mut result = 0u32;
//         let idx = self.index;
//         self.index += 1;

//         // Find the rightmost zero bit in the index
//         let mut c = 0;
//         let mut tmp = idx;
//         while (tmp & 1) != 0 {
//             tmp >>= 1;
//             c += 1;
//         }

//         // XOR the direction vectors for each dimension
//         // For simplicity, we use dimension 0 (first Sobol dimension)
//         result ^= self.direction_vectors[0][c as usize];

//         result
//     }

//     pub fn rand(&mut self) -> f32 {
//         self.uniform_u32() as f32 * (1.0 / 4294967296.0)
//     }
// }

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
