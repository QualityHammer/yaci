use crate::chip8::memory::DisplayData;

use std::ops::{Index, IndexMut};
use std::rc::{Weak, Rc};
use std::slice::SliceIndex;

#[derive(Clone, Copy)]
pub struct Key(pub bool);

#[derive(Clone)]
pub struct Keys([Key; 16]);

impl Keys {
    pub fn new() -> Self {
        Keys([Key(false); 16])
    }
}

impl<Idx> Index<Idx> for Keys
    where Idx: SliceIndex<[Key]> {
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

impl<Idx> IndexMut<Idx> for Keys
    where Idx: SliceIndex<[Key]> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.0[index]
    }
}

pub trait FrontEnd {
    fn draw(&mut self, data: &DisplayData);

    fn get_keys(&self) -> Weak<Keys>;

    fn wait_for_keypress(&self) -> u8;
}

pub struct MockFrontEnd {
    keys: Rc<Keys>
}

impl MockFrontEnd {
    pub fn new() -> Self {
        MockFrontEnd {
            keys: Rc::new(Keys::new())
        }
    }
}

impl FrontEnd for MockFrontEnd {
    fn draw(&mut self, data: &DisplayData) {}

    fn get_keys(&self) -> Weak<Keys> {
        Rc::downgrade(&self.keys)
    }

    fn wait_for_keypress(&self) -> u8 {
        0
    }
}