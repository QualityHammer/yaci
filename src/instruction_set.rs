mod opcodes {
    use crate::Chip8InterpreterRef;

    type Opcode = u16;
    type Instruction = fn(Chip8InterpreterRef, u16) -> Option<()>;

    const FIRST_DIGIT_OPCODE_COUNT: usize = 12;
    const DIGIT_0_OPCODE_COUNT: usize = 3;
    const DIGIT_8_OPCODE_COUNT: usize = 9;
    const DIGIT_E_OPCODE_COUNT: usize = 2;
    const DIGIT_F_OPCODE_COUNT: usize = 9;

    use digit_0::*;

    const DIGIT_0_OPCODES: [Opcode; DIGIT_0_OPCODE_COUNT] = [0x0, 0xE0, 0xEE];
    const DIGIT_0_INSTRUCTIONS: [Instruction; DIGIT_0_OPCODE_COUNT] = [_00, _E0, _EE];

    use digit_8::*;

    //const DIGIT_8_OPCODES: [Opcode; DIGIT_8_OPCODE_COUNT] = [];
    //const DIGIT_8_INSTRUCTIONS: [Instruction; DIGIT_8_OPCODE_COUNT] = [];

    use digit_e::*;

    //const DIGIT_E_OPCODES: [Opcode; DIGIT_E_OPCODE_COUNT] = [];
    //const DIGIT_E_INSTRUCTIONS: [Instruction; DIGIT_E_OPCODE_COUNT] = [];

    use digit_f::*;

    //const DIGIT_F_OPCODES: [Opcode; DIGIT_F_OPCODE_COUNT] = [];
    //const DIGIT_F_INSTRUCTIONS: [Instruction; DIGIT_F_OPCODE_COUNT] = [];

    use std::collections::HashMap;

    pub struct Chip8InstructionSet {
        first_digit: HashMap<Opcode, Instruction>,
        digit_0: HashMap<Opcode, Instruction>,
        digit_8: HashMap<Opcode, Instruction>,
        digit_e: HashMap<Opcode, Instruction>,
        digit_f: HashMap<Opcode, Instruction>,
    }

    impl Chip8InstructionSet {
        pub fn new() -> Self {
            let mut first_digit = HashMap::with_capacity(FIRST_DIGIT_OPCODE_COUNT);
            let mut digit_0 = HashMap::with_capacity(DIGIT_0_OPCODE_COUNT);
            for (op, func) in DIGIT_0_OPCODES
                .iter()
                .map(|o| *o as Opcode)
                .zip(DIGIT_0_INSTRUCTIONS.iter().map(|f| *f as Instruction))
            {
                digit_0.insert(op, func);
            }

            let mut digit_8 = HashMap::with_capacity(DIGIT_8_OPCODE_COUNT);
            let mut digit_e = HashMap::with_capacity(DIGIT_E_OPCODE_COUNT);
            let mut digit_f = HashMap::with_capacity(DIGIT_F_OPCODE_COUNT);

            Chip8InstructionSet {
                first_digit,
                digit_0,
                digit_8,
                digit_e,
                digit_f,
            }
        }
    }

    mod digit_0 {
        use super::*;

        pub fn _00(chip: Chip8InterpreterRef, address: u16) -> Option<()> {
            chip.borrow_mut().PC = address;

            Some(())
        }

        pub fn _E0(chip: Chip8InterpreterRef, _: u16) -> Option<()> {
            Some(())
        }

        pub fn _EE(chip: Chip8InterpreterRef, _: u16) -> Option<()> {
            let mut chip = chip.borrow_mut();
            let sp = chip.SP as usize;

            chip.PC = chip.stack[sp].unwrap();
            chip.stack[sp] = None;
            chip.SP -= 1;

            Some(())
        }
    }

    mod digit_8 {}

    mod digit_e {}

    mod digit_f {}
}
