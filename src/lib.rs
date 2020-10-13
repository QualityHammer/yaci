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

pub type Chip8InterpreterRef = Rc<RefCell<Chip8Interpreter>>;

pub struct Chip8Interpreter {
    pub ram: [u8; 4096],
    pub v: [u8; 16],
    pub i: [u8; 16],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub pc: u16,
    pub sp: u8,
    pub stack: [Option<u16>; 16],
}
>>>>>>> Initial structures for the emulator and instruction set.
