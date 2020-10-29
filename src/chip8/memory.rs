pub use ram::Ram;
pub use display::DisplayData;

mod ram {
    use std::ops::{Index, IndexMut};
    use std::slice::SliceIndex;

    const SIZE: u16 = 4096;

    pub struct Ram {
        data: [u8; SIZE as usize]
    }

    impl Ram {
        pub fn new() -> Self {
            Ram{
                data: [0; SIZE as usize]
            }
        }
    }

    impl<Idx> Index<Idx> for Ram
    where Idx: SliceIndex<[u8]> {
        type Output = Idx::Output;

        fn index(&self, index: Idx) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<Idx> IndexMut<Idx> for Ram
        where Idx: SliceIndex<[u8]> {
        fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
            &mut self.data[index]
        }
    }
}

pub mod display {
    use std::num::Wrapping;

    pub const BIT_SIZE: u16 = 2048;
    pub const BYTE_SIZE: u16 = 256;
    pub const ROW_SIZE: u8 = 64;
    pub const COL_SIZE: u8 = 32;

    pub struct DisplayData {
        pub data: [u8; 256]
    }

    impl DisplayData {
        pub fn new() -> Self {
            DisplayData {
                data: [0; 256]
            }
        }

        pub fn clear(&mut self) {
            self.data = [0; 256];
        }

        pub fn draw_pixel(&mut self, index: u16, pixel: u8) {
            assert!(pixel == 0 || pixel == 1);
            self.data[(index / 8) as usize] ^= pixel << (7 - (index % 8));
        }

        pub fn draw_sprite(&mut self, x: u8, y: u8, pixels: &[u8], height: u8) -> Wrapping<u8> {
            assert_eq!(height as usize, pixels.len());
            let index = (x as usize) + (y as usize * ROW_SIZE as usize);
            let mut flag: u8 = 0;
            for i in 0..height as usize {
                let index = (index / 8) + i * 8;
                if self.data[index] & pixels[i] > 0 {
                    flag = 1;
                }
                self.data[index] ^= pixels[i];
            }
            Wrapping(flag)
        }
    }
}