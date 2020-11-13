extern crate sdl2;

use crate::chip8::memory::DisplayBuffer;
use crate::frontend::{FrontEnd, Key, Keys};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

pub struct Sdl2FrontEnd {
    canvas: WindowCanvas,
    event_pump: EventPump,
    keys: Keys,
    scale: u32,
    should_quit: bool,
}

fn get_key_index(keycode: &Keycode) -> usize {
    match keycode {
        Keycode::Num1 => 0x1,
        Keycode::Num2 => 0x2,
        Keycode::Num3 => 0x3,
        Keycode::Num4 => 0xC,
        Keycode::Q => 0x4,
        Keycode::W => 0x5,
        Keycode::E => 0x6,
        Keycode::R => 0xD,
        Keycode::A => 0x7,
        Keycode::S => 0x8,
        Keycode::D => 0x9,
        Keycode::F => 0xE,
        Keycode::Z => 0xA,
        Keycode::X => 0x0,
        Keycode::C => 0xB,
        Keycode::V => 0xF,
        _ => 0xFFFF,
    }
}

impl Sdl2FrontEnd {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let canvas = sdl_context
            .video()
            .unwrap()
            .window("chip8", 1280, 640)
            .position_centered()
            .build()
            .unwrap()
            .into_canvas()
            .build()
            .unwrap();
        Sdl2FrontEnd {
            canvas,
            event_pump: sdl_context.event_pump().unwrap(),
            keys: Keys::default(),
            scale: 20,
            should_quit: false,
        }
    }

    fn update_should_quit(&mut self, should_quit: bool) {
        if should_quit {
            self.should_quit = should_quit;
        }
    }
}

impl FrontEnd for Sdl2FrontEnd {
    fn draw(&mut self, data: &DisplayBuffer) {
        self.canvas.clear();

        for (i, pixel) in data.data.iter().enumerate() {
            let color: u8 = 255 * pixel;
            self.canvas.set_draw_color(Color::RGB(0, color, 0));
            let x = (i % 64) as i32 * self.scale as i32;
            let y = (i / 64) as i32 * self.scale as i32;
            self.canvas
                .fill_rect(Rect::new(x, y, self.scale, self.scale))
                .expect("Could not draw on canvas");
        }

        self.canvas.present();
    }

    fn get_keys(&mut self) -> &Keys {
        let mut keys_down: Vec<usize> = vec![];
        let mut keys_up: Vec<usize> = vec![];
        let mut should_quit = false;

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => should_quit = true,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    keys_down.push(get_key_index(&keycode));
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    keys_up.push(get_key_index(&keycode));
                }
                _ => {}
            }
        }

        for key in keys_down {
            self.keys[key] = Key(true);
        }
        for key in keys_up {
            self.keys[key] = Key(false);
        }
        self.update_should_quit(should_quit);

        &self.keys
    }

    fn should_quit(&self) -> bool {
        self.should_quit
    }

    fn update(&mut self) {
        let mut should_quit = false;
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => should_quit = true,
                _ => {}
            }
        }
        self.update_should_quit(should_quit);
    }

    fn wait_for_keypress(&mut self) -> u8 {
        loop {
            let mut should_quit = false;
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => should_quit = true,
                    Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => {
                        let key = get_key_index(&keycode);
                        if key <= 0xF {
                            return key as u8;
                        }
                    }
                    _ => {}
                }
            }
            self.update_should_quit(should_quit);
            if should_quit {
                return 0;
            }
        }
    }
}
