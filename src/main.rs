use r_jvm::class;
// use r_jvm::code::code::read_ope_code;

fn main() {}

#[test]
fn read_classfile() {
    let filename: &str = "java/JustAddInt.class";
    let mut reader = match class::class_parser::ClassFileReader::new(filename) {
        Some(reader) => reader,
        None => {
            eprintln!("{}: file not found.", filename);
            return;
        }
    };

    let class_file = reader.read();
    println!("{:?}", class_file);
    // read_ope_code(class_file)
}
