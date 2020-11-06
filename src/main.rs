use yaci::chip8::execution;

fn main() -> Result<(), &'static str> {
    execution::run_interpreter()?;

    Ok(())
}
