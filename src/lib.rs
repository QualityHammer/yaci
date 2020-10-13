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
    RAM: [u8; 4096],
    V: [u8; 16],
    I: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    PC: u16,
    SP: u8,
    stack: [u16; 16],
}
>>>>>>> Initial structures for the emulator and instruction set.
