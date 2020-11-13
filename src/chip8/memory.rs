pub use display::DisplayBuffer;
pub use ram::Ram;

mod ram {
    use std::ops::{Index, IndexMut};
    use std::slice::SliceIndex;

    const SIZE: usize = 4096;

    pub struct Ram {
        data: [u8; SIZE],
    }

    impl Default for Ram {
        fn default() -> Self {
            let mut data = [0; SIZE];
            data[0x50] = 0xF0;
            data[0x51] = 0x90;
            data[0x52] = 0x90;
            data[0x53] = 0x90;
            data[0x54] = 0xF0;
            // 1
            data[0x55] = 0x20;
            data[0x56] = 0x60;
            data[0x57] = 0x20;
            data[0x58] = 0x20;
            data[0x59] = 0x70;
            // 2
            data[0x5A] = 0xF0;
            data[0x5B] = 0x10;
            data[0x5C] = 0xF0;
            data[0x5D] = 0x80;
            data[0x5E] = 0xF0;
            // 3
            data[0x5F] = 0xF0;
            data[0x60] = 0x10;
            data[0x61] = 0xF0;
            data[0x62] = 0x10;
            data[0x63] = 0xF0;
            // 4
            data[0x64] = 0x90;
            data[0x65] = 0x90;
            data[0x66] = 0xF0;
            data[0x67] = 0x10;
            data[0x68] = 0x10;
            // 5
            data[0x69] = 0xF0;
            data[0x6A] = 0x80;
            data[0x6B] = 0xF0;
            data[0x6C] = 0x10;
            data[0x6D] = 0xF0;
            // 6
            data[0x6E] = 0xF0;
            data[0x6F] = 0x80;
            data[0x70] = 0xF0;
            data[0x71] = 0x90;
            data[0x72] = 0xF0;
            // 7
            data[0x73] = 0xF0;
            data[0x74] = 0x10;
            data[0x75] = 0x20;
            data[0x76] = 0x40;
            data[0x77] = 0x40;
            // 8
            data[0x78] = 0xF0;
            data[0x79] = 0x90;
            data[0x7A] = 0xF0;
            data[0x7B] = 0x90;
            data[0x7C] = 0xF0;
            // 9
            data[0x7D] = 0xF0;
            data[0x7E] = 0x90;
            data[0x7F] = 0xF0;
            data[0x80] = 0x10;
            data[0x81] = 0xF0;
            // A
            data[0x82] = 0xF0;
            data[0x83] = 0x90;
            data[0x84] = 0xF0;
            data[0x85] = 0x90;
            data[0x86] = 0x90;
            // B
            data[0x87] = 0xE0;
            data[0x88] = 0x90;
            data[0x89] = 0xE0;
            data[0x8A] = 0x90;
            data[0x8B] = 0xE0;
            // C
            data[0x8C] = 0xF0;
            data[0x8D] = 0x80;
            data[0x8E] = 0x80;
            data[0x8F] = 0x80;
            data[0x90] = 0xF0;
            // D
            data[0x91] = 0xE0;
            data[0x92] = 0x90;
            data[0x93] = 0x90;
            data[0x94] = 0x90;
            data[0x95] = 0xE0;
            // E
            data[0x96] = 0xF0;
            data[0x97] = 0x80;
            data[0x98] = 0xF0;
            data[0x99] = 0x80;
            data[0x9A] = 0xF0;
            // F
            data[0x9B] = 0xF0;
            data[0x9C] = 0x80;
            data[0x9D] = 0xF0;
            data[0x9E] = 0x80;
            data[0x9F] = 0x80;

            Self { data }
        }
    }

    impl<Idx> Index<Idx> for Ram
    where
        Idx: SliceIndex<[u8]>,
    {
        type Output = Idx::Output;

        fn index(&self, index: Idx) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<Idx> IndexMut<Idx> for Ram
    where
        Idx: SliceIndex<[u8]>,
    {
        fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
            &mut self.data[index]
        }
    }
}

pub mod display {
    use std::num::Wrapping;

    pub const SIZE: usize = 2048;
    pub const ROW_SIZE: usize = 64;
    pub const COL_SIZE: usize = 32;

    pub struct DisplayBuffer {
        pub data: [u8; SIZE],
    }

    impl DisplayBuffer {
        pub fn clear(&mut self) {
            self.data = [0; SIZE];
        }

        pub fn draw_sprite(&mut self, x: u8, y: u8, pixels: &[u8]) -> Wrapping<u8> {
            assert_eq!(pixels.len(), pixels.len());
            let index = (x as usize) + (y as usize * ROW_SIZE);
            let mut flag: u8 = 0;
            for (i, pixel) in pixels.iter().enumerate() {
                let index = i * ROW_SIZE + index;
                for j in 0..8 {
                    let bit_shift = 7 - j;
                    let bit = (pixel & (0x1 << bit_shift)) >> bit_shift;
                    if self.data[index + j] & bit > 0 {
                        flag = 1;
                    }
                    self.data[index + j] ^= bit;
                }
            }

            Wrapping(flag)
        }
    }

    impl Default for DisplayBuffer {
        fn default() -> Self {
            Self { data: [0; SIZE] }
        }
    }
}
