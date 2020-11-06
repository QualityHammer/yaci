pub use display::DisplayData;
pub use ram::Ram;

mod ram {
    use std::ops::{Index, IndexMut};
    use std::slice::SliceIndex;

    const SIZE: u16 = 4096;

    pub struct Ram {
        data: [u8; SIZE as usize],
    }

    impl Default for Ram {
        fn default() -> Self {
            Self {
                data: [0; SIZE as usize],
            }
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
    pub const ROW_SIZE: u8 = 64;
    pub const COL_SIZE: u8 = 32;

    pub struct DisplayData {
        pub data: [u8; SIZE],
    }

    impl DisplayData {
        pub fn clear(&mut self) {
            self.data = [0; SIZE];
        }

        pub fn draw_pixel(&mut self, index: u16, pixel: u8) {
            assert!(pixel == 0 || pixel == 1);
            self.data[(index / 8) as usize] ^= pixel << (7 - (index % 8));
        }

        pub fn draw_sprite(&mut self, x: u8, y: u8, pixels: &[u8]) -> Wrapping<u8> {
            assert_eq!(pixels.len(), pixels.len());
            let index = (x as usize) + (y as usize * ROW_SIZE as usize);
            let mut flag: u8 = 0;
            for (i, pixel) in pixels.iter().enumerate() {
                let index = i * ROW_SIZE as usize + index;
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

    impl Default for DisplayData {
        fn default() -> Self {
            Self { data: [0; SIZE] }
        }
    }
}
