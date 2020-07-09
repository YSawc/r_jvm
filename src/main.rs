use r_jvm::class;
use r_jvm::class::attribute::Attribute;
use r_jvm::code::code;

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

    let class_file = reader.read();

    let methods = class_file.unwrap().methods;
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

    let ret_v = code::read_ope_code(code);
    println!("return value : {:?}", ret_v);
}
