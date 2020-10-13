mod opcodes {
    use crate::Chip8InterpreterRef;

    type Instruction = fn(Chip8InterpreterRef, u16) -> Option<()>;

    const FIRST_DIGIT_OPCODE_COUNT: usize = 12;
    const DIGIT_0_OPCODE_COUNT: usize = 3;
    const DIGIT_8_OPCODE_COUNT: usize = 9;
    const DIGIT_E_OPCODE_COUNT: usize = 2;
    const DIGIT_F_OPCODE_COUNT: usize = 9;

    use first_digit as d;

    const FIRST_DIGIT_OPCODES: [u16; 7] = [0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7];
    const FIRST_DIGIT_INSTRUCTIONS: [Instruction; 7] =
        [d::_1, d::_2, d::_3, d::_4, d::_5, d::_6, d::_7];

    use digit_0 as d0;

    const DIGIT_0_OPCODES: [u16; DIGIT_0_OPCODE_COUNT] = [0x0, 0xE0, 0xEE];
    const DIGIT_0_INSTRUCTIONS: [Instruction; DIGIT_0_OPCODE_COUNT] = [d0::_0, d0::_e0, d0::_ee];

    use digit_8 as d8;

    //const DIGIT_8_OPCODES: [u16; DIGIT_8_OPCODE_COUNT] = [];
    //const DIGIT_8_INSTRUCTIONS: [Instruction; DIGIT_8_OPCODE_COUNT] = [];

    use digit_e as de;

    //const DIGIT_E_OPCODES: [u16; DIGIT_E_OPCODE_COUNT] = [];
    //const DIGIT_E_INSTRUCTIONS: [Instruction; DIGIT_E_OPCODE_COUNT] = [];

    use digit_f as df;

    //const DIGIT_F_OPCODES: [u16; DIGIT_F_OPCODE_COUNT] = [];
    //const DIGIT_F_INSTRUCTIONS: [Instruction; DIGIT_F_OPCODE_COUNT] = [];

    use std::collections::HashMap;

    pub struct Chip8InstructionSet {
        first_digit: HashMap<u16, Instruction>,
        digit_0: HashMap<u16, Instruction>,
        digit_8: HashMap<u16, Instruction>,
        digit_e: HashMap<u16, Instruction>,
        digit_f: HashMap<u16, Instruction>,
    }

    impl Chip8InstructionSet {
        pub fn new() -> Self {
            let mut first_digit = HashMap::with_capacity(FIRST_DIGIT_OPCODE_COUNT);
            let mut digit_0 = HashMap::with_capacity(DIGIT_0_OPCODE_COUNT);
            for (op, func) in DIGIT_0_OPCODES
                .iter()
                .map(|o| *o as u16)
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

    mod hex {
        // Last significant hex
        pub fn lsh(value: u16, lsh_place: u16) -> u16 {
            assert!(lsh_place > 0);
            let lsh_place = lsh_place - 1;
            (value >> (4 * lsh_place)) & 0xF << (lsh_place * 4)
        }

        pub fn opcode_lsh(value: u16, lsh_place: u16) -> u16 {
            assert!(lsh_place > 0 && lsh_place < 4);
            if lsh_place == 3 {
                (value >> (4 * lsh_place))
            } else {
                lsh(value, lsh_place)
            }
        }

        pub fn l2sh(value: u16) -> u8 {
            (value & 0xFF) as u8
        }

        pub fn lsh1(value: u16) -> u16 {
            opcode_lsh(value, 1)
        }

        pub fn lsh2(value: u16) -> u16 {
            opcode_lsh(value, 2)
        }

        pub fn lsh3(value: u16) -> u16 {
            opcode_lsh(value, 3)
        }
    }

    mod first_digit {
        use super::*;

        // Jump to address
        pub fn _1(chip: Chip8InterpreterRef, address: u16) -> Option<()> {
            chip.borrow_mut().jump(address);

            Some(())
        }

        // Call subroutine at address
        pub fn _2(chip: Chip8InterpreterRef, address: u16) -> Option<()> {
            let mut chip = chip.borrow_mut();
            chip.push_stack();
            chip.jump(address);

            Some(())
        }

        // Skip next instruction if Vx == kk
        pub fn _3(chip: Chip8InterpreterRef, args: u16) -> Option<()> {
            let mut chip = chip.borrow_mut();
            if chip.get_register(hex::lsh3(args)) == hex::l2sh(args) {
                chip.increase_pc();
            }

            Some(())
        }

        // Skip next instruction if Vx != kk
        pub fn _4(chip: Chip8InterpreterRef, args: u16) -> Option<()> {
            let mut chip = chip.borrow_mut();
            if chip.get_register(hex::lsh3(args)) != hex::l2sh(args) {
                chip.increase_pc();
            }

            Some(())
        }

        // Skip next instruction if Vx == Vy
        pub fn _5(chip: Chip8InterpreterRef, args: u16) -> Option<()> {
            let mut chip = chip.borrow_mut();
            if chip.get_register(hex::lsh3(args)) == chip.get_register(hex::lsh2(args)) {
                chip.increase_pc();
            }

            Some(())
        }

        // Set Vx = kk
        pub fn _6(chip: Chip8InterpreterRef, args: u16) -> Option<()> {
            chip.borrow_mut()
                .set_register(hex::lsh3(args), hex::l2sh(args));

            Some(())
        }

        // Set Vx = Vx + kk
        pub fn _7(chip: Chip8InterpreterRef, args: u16) -> Option<()> {
            *chip.borrow_mut().get_register_mut(hex::lsh3(args)) += hex::l2sh(args);

            Some(())
        }
    }

    mod digit_0 {
        use super::*;

        // Ignore, deprecated
        pub fn _0(chip: Chip8InterpreterRef, address: u16) -> Option<()> {
            Some(())
        }

        // Clear screen
        pub fn _e0(chip: Chip8InterpreterRef, _: u16) -> Option<()> {
            Some(())
        }

        // Return from function
        pub fn _ee(chip: Chip8InterpreterRef, _: u16) -> Option<()> {
            chip.borrow_mut().pop_stack();

            Some(())
        }
    }

    mod digit_8 {
        use super::*;

        // Set Vx = Vy
        pub fn _0(chip: Chip8InterpreterRef, args: u16) -> Option<()> {
            let mut chip = chip.borrow_mut();
            let vy = chip.get_register(hex::lsh2(args));
            chip.set_register(hex::lsh3(args), vy);

            Some(())
        }

        // Sets Vx = Vx | Vy
        pub fn _1(chip: Chip8InterpreterRef, args: u16) -> Option<()> {
            let mut chip = chip.borrow_mut();
            let vy = chip.get_register(hex::lsh2(args));
            *chip.get_register_mut(hex::lsh3(args)) |= vy;

            Some(())
        }

        // Sets Vx = Vx & Vy
        pub fn _2(chip: Chip8InterpreterRef, args: u16) -> Option<()> {
            let mut chip = chip.borrow_mut();
            let vy = chip.get_register(hex::lsh2(args));
            *chip.get_register_mut(hex::lsh3(args)) &= vy;

            Some(())
        }

        // Sets Vx = Vx ^ Vy
        pub fn _3(chip: Chip8InterpreterRef, _: u16) -> Option<()> {
            Some(())
        }

        pub fn _4(chip: Chip8InterpreterRef, _: u16) -> Option<()> {
            Some(())
        }

        pub fn _5(chip: Chip8InterpreterRef, _: u16) -> Option<()> {
            Some(())
        }

        pub fn _6(chip: Chip8InterpreterRef, _: u16) -> Option<()> {
            Some(())
        }

        pub fn _7(chip: Chip8InterpreterRef, _: u16) -> Option<()> {
            Some(())
        }

        pub fn _E(chip: Chip8InterpreterRef, _: u16) -> Option<()> {
            Some(())
        }
    }

    mod digit_e {}

    mod digit_f {}
}
