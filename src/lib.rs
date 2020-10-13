<<<<<<< HEAD
pub mod chip8;
pub mod bitwise;
pub mod frontend;
mod sdl2;

pub use chip8::Chip8Vm;
=======
pub mod instruction_set;

use std::cell::RefCell;
use std::rc::Rc;

pub type Chip8InterpreterRef = Rc<RefCell<dyn Chip8Interpreter>>;

pub struct Chip8Machine {
    ram: [u8; 4096],
    v: [u8; 16],
    i: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    sp: u16,
    stack: [Option<u16>; 16],
}

impl Chip8Interpreter for Chip8Machine {
    fn get_register(&self, address: u16) -> u8 {
        self.v[address as usize]
    }

    fn get_register_mut(&mut self, address: u16) -> &mut u8 {
        &mut self.v[address as usize]
    }

    fn increase_pc(&mut self) {
        self.pc += 1;
    }

    fn jump(&mut self, address: u16) {
        self.pc = address;
    }

    fn pop_stack(&mut self) {
        let sp = self.sp as usize;
        self.pc = self.stack[sp].unwrap();
        self.stack[sp] = None;
        self.sp -= 1;
    }

    fn push_stack(&mut self) {
        self.sp += 1;
        self.stack[self.sp as usize] = Some(self.pc);
    }

    fn set_register(&mut self, address: u16, value: u8) {
        self.v[address as usize] = value;
    }
}

pub trait Chip8Interpreter {
    fn get_register(&self, address: u16) -> u8;

    fn get_register_mut(&mut self, address: u16) -> &mut u8;

    fn increase_pc(&mut self);

    fn jump(&mut self, address: u16);

    fn pop_stack(&mut self);

    fn push_stack(&mut self);

    fn set_register(&mut self, address: u16, value: u8);
}
>>>>>>> Initial structures for the emulator and instruction set.
