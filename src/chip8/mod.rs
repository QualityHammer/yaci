pub mod execution;
pub mod memory;

use crate::bitwise::*;
use crate::frontend::FrontEnd;
use memory::{DisplayData, Ram};

use rand::prelude::*;
use std::io::Error as IOError;
use std::num::Wrapping;
use std::time::{Duration, Instant};
use std::{fs, thread};

const PROGRAM_START: u16 = 0x200;
const FRAME_DURATION: Duration = Duration::from_micros(1429);
const TIMER_DELAY: Duration = Duration::from_micros(16667);

pub struct Chip8Vm {
    ram: Ram,
    display_data: DisplayData,
    draw_flag: bool,
    jump_flag: bool,
    frontend: Box<dyn FrontEnd>,
    v: [Wrapping<u8>; 16],
    dt: u8,
    st: u8,
    i: u16,
    pc: u16,
    sp: usize,
    stack: [u16; 16],
    rng: ThreadRng,
    prev_timer_delay: Instant,
}

impl Chip8Vm {
    pub fn new(frontend: Box<dyn FrontEnd>) -> Chip8Vm {
        Chip8Vm {
            ram: Ram::default(),
            display_data: DisplayData::default(),
            draw_flag: false,
            jump_flag: false,
            frontend,
            v: [Wrapping(0); 16],
            dt: 0,
            st: 0,
            i: 0,
            pc: PROGRAM_START,
            sp: 0,
            stack: [0; 16],
            rng: rand::thread_rng(),
            prev_timer_delay: Instant::now(),
        }
    }

    pub fn load_game(&mut self, filename: &str) -> Result<(), IOError> {
        let file_contents = fs::read("roms/demos/".to_owned() + filename)?;
        for (i, byte) in file_contents.iter().enumerate() {
            self.ram[i + PROGRAM_START as usize] = *byte;
        }

        Ok(())
    }

    pub fn execute_cycle(&mut self) -> Result<(), &'static str> {
        let time = Instant::now();
        let pc = self.pc as usize;
        let opcode: u16 = (self.ram[pc] as u16) << 8 | (self.ram[pc + 1] as u16);
        self.draw_flag = false;
        self.jump_flag = false;

        let unknown_opcode = Err("Unknown opcode");
        let end = opcode & 0xFFF;
        match (opcode & 0xF000) >> 12 {
            0x0 => match opcode & 0xFF {
                0xE0 => self.clear(end),
                0xEE => self.ret(end),
                _ => return unknown_opcode,
            },
            0x1 => self.jump(end),
            0x2 => self.call(end),
            0x3 => self.skip_b_eq(end),
            0x4 => self.skip_b_ne(end),
            0x5 => self.skip_y_eq(end),
            0x6 => self.put_x_b(end),
            0x7 => self.add_x_b(end),
            0x8 => match opcode & 0xF {
                0x0 => self.put_x_y(end),
                0x1 => self.or(end),
                0x2 => self.and(end),
                0x3 => self.xor(end),
                0x4 => self.add_x_y(end),
                0x5 => self.sub(end),
                0x6 => self.shr(end),
                0x7 => self.subn(end),
                0xE => self.shl(end),
                _ => return unknown_opcode,
            },
            0x9 => self.skip_y_ne(end),
            0xA => self.put_i_addr(end),
            0xB => self.jump_0(end),
            0xC => self.rand(end),
            0xD => self.draw(end),
            0xE => match opcode & 0xFF {
                0x9E => self.skip_key(end),
                0xA1 => self.skip_not_key(end),
                _ => return unknown_opcode,
            },
            0xF => match opcode & 0xFF {
                0x07 => self.put_x_dt(end),
                0x0A => self.wait_for_keypress(end),
                0x15 => self.put_dt_x(end),
                0x18 => self.put_st_x(end),
                0x1E => self.add_i_x(end),
                0x29 => self.sprite_addr(end),
                0x33 => self.bcd(end),
                0x55 => self.store(end),
                0x65 => self.read(end),
                _ => return unknown_opcode,
            },
            _ => return unknown_opcode,
        };

        if self.draw_flag {
            self.frontend.draw(&self.display_data);
        }
        self.frontend.update();

        if !self.jump_flag {
            self.pc += 2;
        }

        if self.prev_timer_delay.elapsed() > TIMER_DELAY {
            self.prev_timer_delay = Instant::now();
            if self.dt > 0 {
                self.dt -= 1;
            }
            if self.st > 0 {
                self.st -= 1;
            }
        }

        if time.elapsed() < FRAME_DURATION {
            thread::sleep(FRAME_DURATION - time.elapsed());
        }

        Ok(())
    }

    pub fn should_quit(&self) -> bool {
        self.frontend.should_quit()
    }

    fn clear(&mut self, _: u16) {
        self.display_data.clear();
    }

    fn ret(&mut self, _: u16) {
        self.jump_flag = true;
        self.sp -= 1;
        self.pc = self.stack[self.sp];
        self.stack[self.sp] = 0;
    }

    fn jump(&mut self, op: u16) {
        self.jump_flag = true;
        self.pc = get_address(op);
    }

    fn jump_0(&mut self, op: u16) {
        self.jump_flag = true;
        self.pc = get_address(op) + self.v[0].0 as u16;
    }

    fn call(&mut self, op: u16) {
        self.jump_flag = true;
        self.stack[self.sp] = self.pc;
        self.pc = get_address(op);
        self.sp += 1;
    }

    fn skip_b_eq(&mut self, op: u16) {
        if self.v[get_x(op)].0 == get_byte(op) {
            self.pc += 2;
        }
    }

    fn skip_b_ne(&mut self, op: u16) {
        if self.v[get_x(op)].0 != get_byte(op) {
            self.pc += 2;
        }
    }

    fn skip_y_eq(&mut self, op: u16) {
        if self.v[get_x(op) as usize] == self.v[get_y(op)] {
            self.pc += 2;
        }
    }

    fn skip_y_ne(&mut self, op: u16) {
        if self.v[get_x(op)] != self.v[get_y(op)] {
            self.pc += 2;
        }
    }

    fn skip_key(&mut self, op: u16) {
        if self.frontend.get_keys()[self.v[get_x(op)].0 as usize].0 {
            self.pc += 2;
        }
    }

    fn skip_not_key(&mut self, op: u16) {
        if !self.frontend.get_keys()[self.v[get_x(op)].0 as usize].0 {
            self.pc += 2;
        }
    }

    fn put_x_b(&mut self, op: u16) {
        self.v[get_x(op)] = Wrapping(get_byte(op));
    }

    fn put_x_y(&mut self, op: u16) {
        self.v[get_x(op)] = self.v[get_y(op)];
    }

    fn put_i_addr(&mut self, op: u16) {
        self.i = get_address(op);
    }

    fn put_x_dt(&mut self, op: u16) {
        self.v[get_x(op)] = Wrapping(self.dt);
    }

    fn put_dt_x(&mut self, op: u16) {
        self.dt = self.v[get_x(op)].0;
    }

    fn put_st_x(&mut self, op: u16) {
        self.st = self.v[get_x(op)].0;
    }

    fn add_x_b(&mut self, op: u16) {
        self.v[get_x(op)] += Wrapping(get_byte(op));
    }

    fn add_x_y(&mut self, op: u16) {
        self.v[0xF] = Wrapping(
            (self.v[get_x(op)].0 as u16 + self.v[get_y(op)].0 as u16 > u8::max_value() as u16)
                as u8,
        );
        self.v[get_x(op)] += self.v[get_y(op)];
    }

    fn add_i_x(&mut self, op: u16) {
        self.i += self.v[get_x(op)].0 as u16;
    }

    fn or(&mut self, op: u16) {
        self.v[get_x(op)] |= self.v[get_y(op)];
    }

    fn and(&mut self, op: u16) {
        self.v[get_x(op)] &= self.v[get_y(op)];
    }

    fn xor(&mut self, op: u16) {
        self.v[get_x(op)] ^= self.v[get_y(op)];
    }

    fn sub(&mut self, op: u16) {
        let x = get_x(op);
        let y = get_y(op);
        self.v[0xF] = Wrapping((self.v[x] > self.v[y]) as u8);
        self.v[x] -= self.v[y];
    }

    fn subn(&mut self, op: u16) {
        let x = get_x(op);
        let y = get_y(op);
        self.v[0xF] = Wrapping((self.v[get_x(op)] < self.v[get_y(op)]) as u8);
        self.v[x] = self.v[y] - self.v[x];
    }

    fn shr(&mut self, op: u16) {
        let x = get_x(op);
        self.v[0xF] = Wrapping(self.v[x].0 & 0x1);
        self.v[x] >>= 1;
    }

    fn shl(&mut self, op: u16) {
        let x = get_x(op);
        self.v[0xF] = (self.v[x] & Wrapping(0x80)) >> 7;
        self.v[x] <<= 1;
    }

    fn rand(&mut self, op: u16) {
        self.v[get_x(op)] = Wrapping(self.rng.gen::<u8>() & get_byte(op));
    }

    fn draw(&mut self, op: u16) {
        let length = get_nibble(op) as u8;
        let i = self.i as usize;
        self.v[0xF] = self.display_data.draw_sprite(
            self.v[get_x(op)].0,
            self.v[get_y(op)].0,
            &self.ram[i..i + length as usize],
        );
        self.draw_flag = true;
    }

    fn sprite_addr(&mut self, op: u16) {
        self.i = 0x50 + 5 * self.v[get_x(op)].0 as u16;
    }

    fn bcd(&mut self, op: u16) {
        let i = self.i as usize;
        let vx = self.v[get_x(op)];
        self.ram[i] = (vx / Wrapping(100)).0;
        self.ram[i + 1] = (vx / Wrapping(10)).0 % 10;
        self.ram[i + 2] = vx.0 % 10;
    }

    fn store(&mut self, op: u16) {
        for count in 0..=get_x(op) {
            self.ram[self.i as usize + count] = self.v[count].0;
        }
    }

    fn read(&mut self, op: u16) {
        for count in 0..=get_x(op) {
            self.v[count] = Wrapping(self.ram[self.i as usize + count]);
        }
    }

    fn wait_for_keypress(&mut self, op: u16) {
        self.v[get_x(op)] = Wrapping(self.frontend.wait_for_keypress());
    }
}

#[cfg(test)]
mod tests;
