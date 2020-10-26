use crate::bitwise::*;

use std::num::Wrapping;
use rand::prelude::*;

pub struct Chip8Vm {
    ram: [u8; 4096],
    pixel_data: [u8; 2048],
    v: [Wrapping<u8>; 16],
    dt: u8,
    st: u8,
    i: u16,
    keys: [bool; 16],
    pc: u16,
    sp: usize,
    stack: [u16; 16],
    rng: ThreadRng,
}

impl Chip8Vm {
    pub fn new() -> Chip8Vm {
        Chip8Vm {
            ram: [0; 4096],
            pixel_data: [0; 2048],
            v: [Wrapping(0); 16],
            dt: 0,
            st: 0,
            i: 0,
            keys: [false; 16],
            pc: 0,
            sp: 0,
            stack: [0; 16],
            rng: rand::thread_rng(),
        }
    }

    pub fn clear(&mut self, _: u16) {}

    pub fn ret(&mut self, _: u16) {
        self.sp -= 1;
        self.pc = self.stack[self.sp];
        self.stack[self.sp] = 0;
    }

    pub fn jump(&mut self, op: u16) {
        self.pc = get_address(op);
    }

    pub fn jump_0(&mut self, op: u16) {
        self.pc = get_address(op) + self.v[0].0 as u16;
    }

    pub fn call(&mut self, op: u16) {
        self.stack[self.sp] = self.pc;
        self.pc = get_address(op);
        self.sp += 1;
    }

    pub fn skip_b_eq(&mut self, op: u16) {
        if self.v[get_x(op)].0 == get_byte(op) {
            self.pc += 2;
        }
    }

    pub fn skip_b_ne(&mut self, op: u16) {
        if self.v[get_x(op)].0 != get_byte(op) {
            self.pc += 2;
        }
    }

    pub fn skip_y_eq(&mut self, op: u16) {
        if self.v[get_x(op) as usize] == self.v[get_y(op)] {
            self.pc += 2;
        }
    }

    pub fn skip_y_ne(&mut self, op: u16) {
        if self.v[get_x(op)] != self.v[get_y(op)] {
            self.pc += 2;
        }
    }

    pub fn skip_key(&mut self, op: u16) {
        if self.keys[get_x(op)] {
            self.pc += 2;
        }
    }

    pub fn skip_not_key(&mut self, op: u16) {
        if !self.keys[get_x(op)] {
            self.pc += 2;
        }
    }

    pub fn put_x_b(&mut self, op: u16) {
        self.v[get_x(op)] = Wrapping(get_byte(op));
    }

    pub fn put_x_y(&mut self, op: u16) {
        self.v[get_x(op)] = self.v[get_y(op).clone()];
    }

    pub fn put_i_addr(&mut self, op: u16) {
        self.i = get_address(op);
    }

    pub fn put_x_dt(&mut self, op: u16) {
        self.v[get_x(op)] = Wrapping(self.dt);
    }

    pub fn put_dt_x(&mut self, op: u16) {
        self.dt = self.v[get_x(op)].0;
    }

    pub fn put_st_x(&mut self, op: u16) {
        self.st = self.v[get_x(op)].0;
    }

    pub fn add_x_b(&mut self, op: u16) {
        self.v[get_x(op)] += Wrapping(get_byte(op));
    }

    pub fn add_x_y(&mut self, op: u16) {
        self.v[0xF] = Wrapping((self.v[get_x(op)].0 as u16 + self.v[get_y(op)].0 as u16
            > u8::max_value() as u16) as u8);
        self.v[get_x(op)] += self.v[get_y(op)];
    }

    pub fn add_i_x(&mut self, op: u16) {
        self.i += self.v[get_x(op)].0 as u16;
    }

    pub fn or(&mut self, op: u16) {
        self.v[get_x(op)] |= self.v[get_y(op)];
    }

    pub fn and(&mut self, op: u16) {
        self.v[get_x(op)] &= self.v[get_y(op)];
    }

    pub fn xor(&mut self, op: u16) {
        self.v[get_x(op)] ^= self.v[get_y(op)];
    }

    pub fn sub(&mut self, op: u16) {
        let x = get_x(op);
        let y = get_y(op);
        self.v[0xF] = Wrapping((self.v[x] > self.v[y]) as u8);
        self.v[x] -= self.v[y];
    }

    pub fn subn(&mut self, op: u16) {
        let x = get_x(op);
        let y = get_y(op);
        self.v[0xF] = Wrapping((self.v[get_x(op)] < self.v[get_y(op)]) as u8);
        self.v[x] = self.v[y] - self.v[x];
    }

    pub fn shr(&mut self, op: u16) {
        let x = get_x(op);
        self.v[0xF] = Wrapping(self.v[x].0 & 0x1);
        self.v[x] >>= 1;
    }

    pub fn shl(&mut self, op: u16) {
        let x = get_x(op);
        self.v[0xF] = (self.v[x] & Wrapping(0x80)) >> 7;
        self.v[x] <<= 1;
    }

    pub fn rand(&mut self, op: u16) {
        self.v[get_x(op)] = Wrapping(self.rng.gen::<u8>() & get_byte(op));
    }

    pub fn draw(&mut self, op: u16) {}

    pub fn sprite(&mut self, op: u16) {}

    pub fn bcd(&mut self, op: u16) {
        let i = self.i as usize;
        let vx = self.v[get_x(op)];
        self.ram[i] = (vx / Wrapping(100)).0;
        self.ram[i + 1] = (vx / Wrapping(10)).0 % 10;
        self.ram[i + 2] = vx.0 % 10;
    }

    pub fn store(&mut self, op: u16) {
        for count in 0..=get_x(op) {
            self.ram[self.i as usize + count] = self.v[count].0;
        }
    }

    pub fn read(&mut self, op: u16) {
        for count in 0..=get_x(op) {
            self.v[count] = Wrapping(self.ram[self.i as usize + count]);
        }
    }

    pub fn wait_for_keypress(&mut self, op: u16) {}
}

#[cfg(test)]
mod tests;
