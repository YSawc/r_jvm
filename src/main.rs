use r_jvm::vm::vm;
fn main() {}

#[test]
fn just_add_int() {
    let mut vm = vm::VM::new();
    // vm.run("JustAddInt");
    // vm.run("CallAdd");
    vm.run("Fibonacci");
}
