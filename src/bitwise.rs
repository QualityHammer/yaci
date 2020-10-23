pub fn get_address(opcode: u16) -> u16 {
    opcode & 0xFFF
}

pub fn get_nibble(opcode: u16) -> u16 {
    opcode & 0xF
}

pub fn get_x(opcode: u16) -> usize {
    ((opcode & 0xF00) >> 8) as usize
}

pub fn get_y(opcode: u16) -> usize {
    ((opcode & 0xF0) >> 4) as usize
}

pub fn get_byte(opcode: u16) -> u8 {
    (opcode & 0xFF) as u8
}
