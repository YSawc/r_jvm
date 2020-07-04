use r_jvm::class;

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

    println!("{:?}", reader.read());
}
