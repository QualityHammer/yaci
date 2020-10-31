extern crate sdl2;

use crate::frontend::{FrontEnd, Keys, Key};
use crate::chip8::memory::display::{ROW_SIZE, COL_SIZE};

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use std::rc::{Rc, Weak};
use crate::chip8::memory::DisplayData;
use self::sdl2::rect::Rect;
use self::sdl2::EventPump;
use std::borrow::BorrowMut;
use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;

pub struct Sdl2FrontEnd {
    canvas: WindowCanvas,
    event_pump: EventPump,
    keys: Keys,
    scale: u32
    // texture: Texture<'a>,
    // texture_creator: TextureCreator<WindowContext>
}

impl Sdl2FrontEnd {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let canvas = sdl_context.video().unwrap()
            .window("chip8", 1280, 640)
            .position_centered().build().unwrap().into_canvas().build().unwrap();
        Sdl2FrontEnd {
            canvas,
            event_pump: sdl_context.event_pump().unwrap(),
            keys: Keys::new(),
            scale: 20
        }
    }
}

impl FrontEnd for Sdl2FrontEnd {
    fn draw(&mut self, data: &DisplayData) {
        self.canvas.clear();
        for (i, pixel) in data.data.iter().enumerate() {
            let color: u8 = 255 * pixel;
            self.canvas.set_draw_color(Color::RGB(color, color, color));
            let x = (i % 64) as i32 * self.scale as i32;
            let y = (i / 64) as i32 * self.scale as i32;
            self.canvas.fill_rect(Rect::new(x, y, self.scale, self.scale));
        }
        self.canvas.present();
    }

    fn get_keys(&mut self) -> &Keys {
        for event in self.event_pump.poll_iter() {
            if let Event::KeyDown {keycode: Some(keycode), ..} = event {
                self.keys[0] = Key(true)
            }
        }
        &self.keys
    }

    fn wait_for_keypress(&self) -> u8 { 0 }
}
