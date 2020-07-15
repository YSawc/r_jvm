use r_jvm::class;
use r_jvm::gc::gc;
use r_jvm::vm::vm;

fn main() {}

#[test]
fn just_add_int() {
    let file_path: &str = "java/JustAddInt.class";
    println!("read java/JustAddInt.class ..");

    let mut reader = match class::class_parser::ClassFileReader::new(file_path) {
        Some(reader) => reader,
        None => {
            eprintln!("{}: file not found.", file_path);
            return;
        }
    };

    let class_name = "JustAddInt";
    let mut gc = gc::ClassHeap::new();
    gc.insert_class(class_name, reader.read().unwrap());

    let mut vm = vm::VM::new();
    vm.run(gc, class_name);
}
