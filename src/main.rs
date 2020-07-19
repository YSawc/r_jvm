use r_jvm::vm::vm;
fn main() {
    println!("{}", 0xff >> 8);
}

#[test]
fn just_add_int() {
    let mut vm = vm::VM::new();
    vm.run("JustAddInt");
    vm.run("CallAdd");
    vm.run("Fibonacci");
}
