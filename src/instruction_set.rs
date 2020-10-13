use crate::Chip8InterpreterRef;

type Opcode = u16;
type Instruction = fn(Chip8InterpreterRef) -> Option<()>;

use std::collections::HashMap;

pub struct Chip8InstructionSet {
    opcode_map: HashMap<Opcode, Instruction>,
}

impl Chip8InstructionSet {
    pub fn new() -> Self {
        let opcode_map = opcodes::generate_opcodes();
        Chip8InstructionSet { opcode_map }
    }
}

mod opcodes {
    use super::*;

    const OPCODE_COUNT: usize = 36;

    pub fn generate_opcodes() -> HashMap<Opcode, Instruction> {
        let mut map = HashMap::with_capacity(OPCODE_COUNT);

        map
    }

    fn _0nnn(chip: Chip8InterpreterRef) -> Option<()> {
        Some(())
    }
}
