use r_jvm::class_parser;
use r_jvm::util;

fn main() {}

#[test]
fn read_classfile() {
    let filename: &str = "java/JustAddInt.class";
    let mut reader = match class_parser::ClassFileReader::new(filename) {
        Some(reader) => reader,
        None => {
            eprintln!("{}: file not found.", filename);
            return;
        }
    };

    reader.read();
}
