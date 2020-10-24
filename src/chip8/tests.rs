use crate::chip8::Chip8Vm;

#[test]
fn test_return() {
    let mut vm = Chip8Vm::new();
    vm.pc = 0x200;
    vm.stack[0] = 0xFFFF;
    vm.sp = 1;

    assert_ne!(vm.pc, 0xFFFF);
    assert_ne!(vm.sp, 0);
    assert_ne!(vm.stack[0], 0);

    vm.ret(0);

    assert_eq!(vm.pc, 0xFFFF);
    assert_eq!(vm.sp, 0);
    assert_eq!(vm.stack[0], 0);
}
