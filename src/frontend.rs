use crate::chip8::memory::DisplayBuffer;

use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;

#[derive(Clone, Copy)]
pub struct Key(pub bool);

#[derive(Clone)]
pub struct Keys([Key; 16]);

impl Default for Keys {
    fn default() -> Self {
        Keys([Key(false); 16])
    }
}

impl<Idx> Index<Idx> for Keys
where
    Idx: SliceIndex<[Key]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

impl<Idx> IndexMut<Idx> for Keys
where
    Idx: SliceIndex<[Key]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.0[index]
    }
}

pub trait FrontEnd {
    fn draw(&mut self, data: &DisplayBuffer);

    fn get_keys(&mut self) -> &Keys;

    fn should_quit(&self) -> bool;

    fn update(&mut self);

    fn wait_for_keypress(&mut self) -> u8;
}

pub struct MockFrontEnd {
    keys: Keys,
}

impl Default for MockFrontEnd {
    fn default() -> Self {
        Self {
            keys: Keys::default(),
        }
    }
}

impl FrontEnd for MockFrontEnd {
    fn draw(&mut self, _data: &DisplayBuffer) {}

    fn get_keys(&mut self) -> &Keys {
        &self.keys
    }

    fn should_quit(&self) -> bool {
        false
    }

    fn update(&mut self) {}

    fn wait_for_keypress(&mut self) -> u8 {
        0
    }
}
