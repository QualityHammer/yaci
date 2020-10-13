mod opcodes {
    use crate::Chip8InterpreterRef;

    type Instruction = fn(Chip8InterpreterRef, usize) -> Option<()>;

    const FIRST_DIGIT_OPCODE_COUNT: usize = 12;
    const DIGIT_0_OPCODE_COUNT: usize = 3;
    const DIGIT_8_OPCODE_COUNT: usize = 9;
    const DIGIT_E_OPCODE_COUNT: usize = 2;
    const DIGIT_F_OPCODE_COUNT: usize = 9;

    use first_digit as d;

    const FIRST_DIGIT_OPCODES: [usize; 1] = [0x1];
    const FIRST_DIGIT_INSTRUCTIONS: [Instruction; 1] = [d::_1];

    use digit_0 as d0;

    const DIGIT_0_OPCODES: [usize; DIGIT_0_OPCODE_COUNT] = [0x0, 0xE0, 0xEE];
    const DIGIT_0_INSTRUCTIONS: [Instruction; DIGIT_0_OPCODE_COUNT] = [d0::_0, d0::_e0, d0::_ee];

    use digit_8 as d8;

    //const DIGIT_8_OPCODES: [usize; DIGIT_8_OPCODE_COUNT] = [];
    //const DIGIT_8_INSTRUCTIONS: [Instruction; DIGIT_8_OPCODE_COUNT] = [];

    use digit_e as de;

    //const DIGIT_E_OPCODES: [usize; DIGIT_E_OPCODE_COUNT] = [];
    //const DIGIT_E_INSTRUCTIONS: [Instruction; DIGIT_E_OPCODE_COUNT] = [];

    use digit_f as df;

    //const DIGIT_F_OPCODES: [usize; DIGIT_F_OPCODE_COUNT] = [];
    //const DIGIT_F_INSTRUCTIONS: [Instruction; DIGIT_F_OPCODE_COUNT] = [];

    use std::collections::HashMap;

    pub struct Chip8InstructionSet {
        first_digit: HashMap<usize, Instruction>,
        digit_0: HashMap<usize, Instruction>,
        digit_8: HashMap<usize, Instruction>,
        digit_e: HashMap<usize, Instruction>,
        digit_f: HashMap<usize, Instruction>,
    }

    impl Chip8InstructionSet {
        pub fn new() -> Self {
            let mut first_digit = HashMap::with_capacity(FIRST_DIGIT_OPCODE_COUNT);
            let mut digit_0 = HashMap::with_capacity(DIGIT_0_OPCODE_COUNT);
            for (op, func) in DIGIT_0_OPCODES
                .iter()
                .map(|o| *o as usize)
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

    mod first_digit {
        use super::*;

        // Jump to address
        pub fn _1(chip: Chip8InterpreterRef, address: usize) -> Option<()> {
            chip.borrow_mut().jump(address);

            Some(())
        }

        // Call subroutine at address
        pub fn _2(chip: Chip8InterpreterRef, address: usize) -> Option<()> {
            let mut chip = chip.borrow_mut();
            chip.push_stack();
            chip.jump(address);

            Some(())
        }

        // Skip next instruction if Vx == kk
        pub fn _3(chip: Chip8InterpreterRef, args: usize) -> Option<()> {
            let mut chip = chip.borrow_mut();
            if chip.get_register(args >> 8) == (args & 0x0FF) as u8 {
                chip.increase_pc();
            }

            Some(())
        }

        // Skip next instruction if Vx != kk
        pub fn _4(chip: Chip8InterpreterRef, args: usize) -> Option<()> {
            let mut chip = chip.borrow_mut();
            if chip.get_register(args >> 8) != (args & 0x0FF) as u8 {
                chip.increase_pc();
            }

            Some(())
        }

        pub fn _5(chip: Chip8InterpreterRef, address: usize) -> Option<()> {
            Some(())
        }

        pub fn _6(chip: Chip8InterpreterRef, address: usize) -> Option<()> {
            Some(())
        }

        pub fn _7(chip: Chip8InterpreterRef, address: usize) -> Option<()> {
            Some(())
        }
    }

    mod digit_0 {
        use super::*;

        // Ignore, deprecated
        pub fn _0(chip: Chip8InterpreterRef, address: usize) -> Option<()> {
            Some(())
        }

        // Clear screen
        pub fn _e0(chip: Chip8InterpreterRef, _: usize) -> Option<()> {
            Some(())
        }

        // Return from function
        pub fn _ee(chip: Chip8InterpreterRef, _: usize) -> Option<()> {
            chip.borrow_mut().pop_stack();

            Some(())
        }
    }

    mod digit_8 {}

    mod digit_e {}

    mod digit_f {}
}
