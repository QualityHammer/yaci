extern crate sdl2;

use crate::frontend::{FrontEnd, Keys};
use crate::chip8::memory::display::{ROW_SIZE, COL_SIZE};

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use std::rc::{Rc, Weak};
use crate::chip8::memory::DisplayData;

pub struct Sdl2FrontEnd {
    canvas: WindowCanvas,
    keys: Rc<Keys>,
    // texture: Texture<'a>,
    // texture_creator: TextureCreator<WindowContext>
}

impl Sdl2FrontEnd {
    pub fn new() -> Self {
        let canvas = sdl2::init().unwrap().video().unwrap()
            .window("chip8", 800, 600)
            .position_centered().build().unwrap().into_canvas().build().unwrap();
        Sdl2FrontEnd {
            canvas,
            keys: Rc::new(Keys::new()),
        }
    }
}

impl FrontEnd for Sdl2FrontEnd {
    fn draw(&mut self, data: &DisplayData) {
        // let mut texture_creator = self.canvas.texture_creator();
        // let mut texture = texture_creator
        //     .create_texture_target(PixelFormatEnum::RGBA8888, ROW_SIZE as u32, COL_SIZE as u32)
        //     .unwrap();
        // texture.update(None, &data.data, ROW_SIZE as usize / 8);
        // self.canvas.fill_rect(None);
    }

    fn get_keys(&self) -> Weak<Keys> { Weak::new() }

    fn wait_for_keypress(&self) -> u8 { 0 }
}
