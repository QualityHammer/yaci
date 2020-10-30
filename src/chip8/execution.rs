use crate::Chip8Vm;
use crate::sdl2::Sdl2FrontEnd;

use std::fmt::Error;

pub fn run_interpreter() -> Result<(), Error> {
    let mut vm = Chip8Vm::new(Box::new(Sdl2FrontEnd::new()));
    vm.load_game("Pong (1 player).ch8");

    loop {
        vm.execute_cycle();
    }

    Ok(())
}