use crate::chip8::Chip8Vm;

#[test]
fn test_jump() {
    let mut vm = init_vm();

    assert_ne!(vm.pc, 0xFF0, "{}", init_pc());

    vm.jump(0xFF0);

    assert_eq!(vm.pc, 0xFF0, "{}", ch_pc());
}

#[test]
fn test_jump_0() {
    let mut vm = init_vm();

    assert_ne!(vm.pc, 0x5FF);

    vm.jump_0(0x500);

    assert_eq!(vm.pc, 0x5FF);
}

#[test]
fn test_call() {
    let mut vm = init_vm();

    let prev_sp = vm.sp;

    assert_ne!(vm.pc, 0xFF0, "{}", init_pc());
    assert_ne!(vm.stack[1], 0x200, "{}", init_stack());

    vm.call(0xFF0);

    assert_eq!(vm.pc, 0xFF0, "{}", ch_pc());
    assert_eq!(vm.stack[1], 0x200, "{}", ch_stack());
    assert_eq!(vm.stack[0], 0xFFF, "{}", ch_stack());
    assert_eq!(prev_sp + 1, vm.sp, "{}", ch_sp());
}

#[test]
fn test_return() {
    let mut vm = init_vm();

    assert_ne!(vm.pc, 0xFFF, "{}", init_pc());
    assert_ne!(vm.sp, 0, "{}", init_sp());
    assert_ne!(vm.stack[0], 0, "{}", init_stack());

    vm.ret(0);

    assert_eq!(vm.pc, 0xFFF, "{}", ch_pc());
    assert_eq!(vm.sp, 0, "{}", ch_sp());
    assert_eq!(vm.stack[0], 0, "{}", ch_stack());
}

#[test]
fn test_b_eq() {
    let mut vm = init_vm();

    vm.skip_b_eq(0x0EF);

    assert_eq!(vm.pc, 0x200, "{}", init_pc());

    let prev_val = vm.pc;
    vm.skip_b_eq(0x0FF);

    assert_eq!(vm.pc, 0x202, "{}", ch_pc());
    assert_eq!(vm.pc, prev_val + 2);
}

#[test]
fn test_b_ne() {
    let mut vm = init_vm();

    vm.skip_b_ne(0x0FF);

    assert_eq!(vm.pc, 0x200, "{}", init_pc());

    let prev_val = vm.pc;
    vm.skip_b_ne(0x0EF);

    assert_eq!(vm.pc, 0x202, "{}", ch_pc());
    assert_eq!(vm.pc, prev_val + 2);
}

#[test]
fn test_y_eq() {
    let mut vm = init_vm();

    vm.skip_y_eq(0x020);

    assert_eq!(vm.pc, 0x200, "{}", init_pc());

    let prev_val = vm.pc;
    vm.skip_y_eq(0x0E0);

    assert_eq!(vm.pc, 0x202, "{}", ch_pc());
    assert_eq!(vm.pc, prev_val + 2);
}

#[test]
fn test_y_ne() {
    let mut vm = init_vm();

    vm.skip_y_ne(0x0E0);

    assert_eq!(vm.pc, 0x200, "{}", init_pc());

    let prev_val = vm.pc;
    vm.skip_y_ne(0x020);

    assert_eq!(vm.pc, 0x202, "{}", ch_pc());
    assert_eq!(vm.pc, prev_val + 2);
}

#[test]
fn test_put_x_b() {
    let mut vm = init_vm();

    assert_ne!(vm.v[0xE].0, 0x20);

    vm.put_x_b(0xE20);

    assert_eq!(vm.v[0xE].0, 0x20);
}

#[test]
fn test_put_x_y() {
    let mut vm = init_vm();

    assert_ne!(vm.v[0].0, 0x20);

    vm.put_x_y(0x010);

    assert_eq!(vm.v[0].0, 0x20);
    assert_eq!(vm.v[0].0, vm.v[1].0);
}

#[test]
fn test_add_x_b() {
    let mut vm = init_vm();

    assert_ne!(vm.v[1].0, 0x2E);

    let prev_val = vm.v[1].0;
    vm.add_x_b(0x10E);

    assert_eq!(vm.v[1].0, 0x2E);
    assert_eq!(prev_val, vm.v[1].0 - 0xE);
}

#[test]
fn test_add_x_y() {
    let mut vm = init_vm();

    assert_ne!(vm.v[1].0, 0x2E);

    let prev_val = vm.v[1];
    vm.add_x_y(0x124);

    assert_eq!(vm.v[0xF].0, 0);
    assert_eq!(vm.v[1].0, 0x2E);
    assert_eq!(prev_val, vm.v[1] - vm.v[2]);

    let mut vm = init_vm();

    assert_ne!(vm.v[0].0, 0xE);

    let prev_val = vm.v[0];
    vm.add_x_y(0x024);

    assert_eq!(vm.v[0xF].0, 1);
    assert_eq!(vm.v[0].0, 0xD);
    assert_eq!(vm.v[0].0, (prev_val + vm.v[2]).0);
}

#[test]
fn test_or() {
    let mut vm = init_vm();

    assert_ne!(vm.v[1].0, 0x2E);

    let prev_val = vm.v[1].0;
    vm.or(0x120);

    assert_eq!(vm.v[1].0, 0x2E);
    assert_eq!(vm.v[1].0, prev_val | vm.v[2].0);
}

#[test]
fn test_and() {
    let mut vm = init_vm();

    assert_ne!(vm.v[1].0, 0x2E);

    let prev_val = vm.v[1].0;
    vm.and(0x120);

    assert_eq!(vm.v[1].0, 0);
    assert_eq!(vm.v[1].0, prev_val & vm.v[2].0);
}

#[test]
fn test_xor() {
    let mut vm = init_vm();

    assert_ne!(vm.v[1].0, 0x2E);

    let prev_val = vm.v[1].0;
    vm.xor(0x120);

    assert_eq!(vm.v[1].0, 0x2E);
    assert_eq!(vm.v[1].0, prev_val ^ vm.v[2].0);
}

#[test]
fn test_sub() {
    let mut vm = init_vm();

    let prev_val = vm.v[0];
    vm.sub(0x015);

    assert_eq!(vm.v[0xF].0, 1);
    assert_eq!(vm.v[0].0, 0xDF);
    assert_eq!(vm.v[0], prev_val - vm.v[1]);

    let mut vm = init_vm();

    assert_ne!(vm.v[2].0, 0xEE);

    let prev_val = vm.v[2];
    vm.sub(0x215);

    assert_eq!(vm.v[0xF].0, 0);
    assert_eq!(vm.v[2].0, 0xEE);
    assert_eq!(vm.v[2], prev_val - vm.v[1]);
}

#[test]
fn test_subn() {
    let mut vm = init_vm();

    let prev_val = vm.v[1];
    vm.subn(0x105);

    assert_eq!(vm.v[0xF].0, 1);
    assert_eq!(vm.v[1].0, 0xDF);
    assert_eq!(vm.v[1], vm.v[0] - prev_val);

    let mut vm = init_vm();

    assert_ne!(vm.v[2].0, 0xEE);

    let prev_val = vm.v[1];
    vm.subn(0x125);

    assert_eq!(vm.v[0xF].0, 0);
    assert_eq!(vm.v[1].0, 0xEE);
    assert_eq!(vm.v[1], vm.v[2] - prev_val);
}

#[test]
fn test_shr() {
    let mut vm = init_vm();

    let prev_val = vm.v[2];
    vm.shr(0x206);

    assert_eq!(vm.v[0xF].0, 0);
    assert_eq!(vm.v[2].0, 0x7);
    assert_eq!(vm.v[2], prev_val >> 1);

    let mut vm = init_vm();

    let prev_val = vm.v[0];

    vm.shr(0x006);

    assert_eq!(vm.v[0xF].0, 1);
    assert_eq!(vm.v[0].0, 0x7F);
    assert_eq!(vm.v[0], prev_val >> 1);
}

#[test]
fn test_shl() {
    let mut vm = init_vm();

    let prev_val = vm.v[2];
    vm.shl(0x20E);

    assert_eq!(vm.v[0xF].0, 0);
    assert_eq!(vm.v[2].0, 0x1C);
    assert_eq!(vm.v[2], prev_val << 1);

    let mut vm = init_vm();

    let prev_val = vm.v[0];

    vm.shl(0x00E);

    assert_eq!(vm.v[0xF].0, 1);
    assert_eq!(vm.v[0].0, 0xFE);
    assert_eq!(vm.v[0], prev_val << 1);
}

#[test]
fn test_put_i_adrr() {
    let mut vm = init_vm();

    assert_ne!(vm.i, 0x200);

    vm.put_i_addr(0x200);

    assert_eq!(vm.i, 0x200);
}

fn init_vm() -> Chip8Vm {
    let mut vm = Chip8Vm::new();
    vm.pc = 0x200;
    vm.stack[0] = 0xFFF;
    vm.sp = 1;
    use std::num::Wrapping;
    vm.v[0] = Wrapping(0xFF);
    vm.v[1] = Wrapping(0x20);
    vm.v[2] = Wrapping(0xE);
    vm.v[0xE] = Wrapping(0xFF);
    vm
}

fn init_pc() -> &'static str {
    "Initial program counter."
}

fn init_sp() -> &'static str {
    "Initial stack pointer."
}

fn init_stack() -> &'static str {
    "Initial stack."
}

fn ch_pc() -> &'static str {
    "Testing change in program counter."
}

fn ch_sp() -> &'static str {
    "Testing change in stack pointer."
}

fn ch_stack() -> &'static str {
    "Testing change in stack."
}
