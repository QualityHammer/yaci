use crate::sdl2::Sdl2FrontEnd;
use crate::Chip8Vm;

pub fn run_interpreter() -> Result<(), &'static str> {
    let mut vm = Chip8Vm::new(Box::new(Sdl2FrontEnd::new()));

    if vm.load_game("IBM Logo.ch8").is_err() {
        return Err("Failed to load game.");
    }

    while !vm.should_quit() {
        if vm.execute_cycle().is_err() {
            return Err("Crash at execution cycle.");
        }
    }

    Ok(())
}
