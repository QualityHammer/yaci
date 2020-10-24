use crate::chip8::Chip8Vm;

#[test]
fn test_jump() {
    let mut vm = init_vm();

    assert_ne!(vm.pc, 0xFF0, "{}", init_pc());

    vm.jump(0xFF0);

    assert_eq!(vm.pc, 0xFF0, "{}", ch_pc());
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

fn init_vm() -> Chip8Vm {
    let mut vm = Chip8Vm::new();
    vm.pc = 0x200;
    vm.stack[0] = 0xFFF;
    vm.sp = 1;
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
