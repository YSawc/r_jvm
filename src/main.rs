use r_jvm::class;
use r_jvm::class::attribute::Attribute;

fn main() {}

#[test]
fn read_classfile() {
    let file_path: &str = "java/JustAddInt.class";
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
    }) = methods[0].get_code_attribute()
    {
        (code_length, code)
    } else {
        panic!()
    };
    println!("{:?}", code);
}
