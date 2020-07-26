#[allow(unused_imports)]
use r_jvm::vm::vm;
fn main() {
    println!("{}", 0xff >> 8);
}

#[test]
fn test() {
    let mut vm = vm::VM::new();
    vm.run("JustAddInt");
    vm.run("CallAdd");
    vm.run("Fibonacci");
    vm.run("FizzBuzz");
    vm.run("CreateBuffer");
    vm.run("Switch");
    vm.run("HelloWorld");
}
