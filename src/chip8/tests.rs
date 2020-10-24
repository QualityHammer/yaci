use crate::chip8::Chip8Vm;

fn init_vm() -> Chip8Vm {
    let mut vm = Chip8Vm::new();
    vm.pc = 0x200;
    vm.stack[0] = 0xFFF;
    vm.sp = 1;
    vm
}

#[test]
fn test_return() {
    let mut vm = init_vm();

    assert_ne!(vm.pc, 0xFFF);
    assert_ne!(vm.sp, 0);
    assert_ne!(vm.stack[0], 0);

    vm.ret(0);

    assert_eq!(vm.pc, 0xFFF);
    assert_eq!(vm.sp, 0);
    assert_eq!(vm.stack[0], 0);
}

#[test]
fn test_jump() {
    let mut vm = init_vm();

    assert_ne!(vm.pc, 0xFF0);

    vm.jump(0xFF0);

    assert_eq!(vm.pc, 0xFF0);
}
