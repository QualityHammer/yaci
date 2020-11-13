pub use display::DisplayBuffer;
pub use ram::Ram;

mod ram {
    use std::ops::{Index, IndexMut};
    use std::slice::SliceIndex;

    const SIZE: usize = 4096;
    const FONT_DATA_SIZE: usize = 16 * 5;
    const FONT_DATA_OFFSET: usize = 0x50;

    const FONT_DATA: [u8; FONT_DATA_SIZE] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];

    pub struct Ram {
        data: [u8; SIZE],
    }

    impl Default for Ram {
        fn default() -> Self {
            let mut data = [0; SIZE];

            for i in 0..FONT_DATA_SIZE {
                data[FONT_DATA_OFFSET + i] = FONT_DATA[i];
            }

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
