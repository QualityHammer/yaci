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
    pc: usize,
    sp: usize,
    stack: [Option<usize>; 16],
}

impl Chip8Interpreter for Chip8Machine {
    fn get_register(&self, address: usize) -> u8 {
        self.v[address]
    }

    fn increase_pc(&mut self) {
        self.pc += 1;
    }

    fn jump(&mut self, address: usize) {
        self.pc = address;
    }

    fn pop_stack(&mut self) {
        self.pc = self.stack[self.sp].unwrap();
        self.stack[self.sp] = None;
        self.sp -= 1;
    }

    fn push_stack(&mut self) {
        self.sp += 1;
        self.stack[self.sp] = Some(self.pc);
    }
}

pub trait Chip8Interpreter {
    fn get_register(&self, address: usize) -> u8;

    fn increase_pc(&mut self);

    fn jump(&mut self, address: usize);

    fn pop_stack(&mut self);

    fn push_stack(&mut self);
}
>>>>>>> Initial structures for the emulator and instruction set.
