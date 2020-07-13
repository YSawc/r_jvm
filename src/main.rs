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

    let mut gc = gc::ClassHeap::new();
    gc.insert_class(reader.read().unwrap());

    vm::run(gc, "0", 1);
}
