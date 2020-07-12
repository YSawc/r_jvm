use r_jvm::class;
use r_jvm::class::attribute::Attribute;
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

    let methods = &gc.get_class("0").unwrap().methods;
    let (_code_length, code) = if let Some(Attribute::Code {
        code_length, code, ..
    }) = methods[1].get_code_attribute()
    {
        (code_length, code)
    } else {
        panic!()
    };
    // println!("{:?}", _code_length);
    println!("{:?}", code);

    let ret_v = vm::read_ope_code(code);
    println!("return value : {:?}", ret_v);
}
