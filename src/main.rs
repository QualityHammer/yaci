use yaci::chip8::Chip8Vm;
use yaci::sdl2::Sdl2FrontEnd;

fn main() -> Result<(), &'static str> {
    let mut vm = Chip8Vm::new(Box::new(Sdl2FrontEnd::new()));

    if vm
        .load_game("games/Brix [Andreas Gustafsson, 1990].ch8")
        .is_err()
    {
        return Err("Failed to load game.");
    }

    while !vm.should_quit() {
        if vm.execute_cycle().is_err() {
            return Err("Crash at execution cycle.");
        }
    }

    Ok(())
}
