use r_jvm::class;
use r_jvm::gc::gc;
use r_jvm::vm::vm;

fn main() {}

#[test]
fn just_add_int() {
    let mut vm = vm::VM::new();
    vm.run("JustAddInt");
    vm.run("CallAdd");
}
