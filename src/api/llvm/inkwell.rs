use inkwell::context::Context;
use std::process::Command;

pub fn println(str: String) {
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let i32_type = context.i32_type();

    let putchar_type = i32_type.fn_type(&[i32_type.into()], false);
    module.add_function("putchar", putchar_type, None);

    let main_type = i32_type.fn_type(&[], false);
    let function = module.add_function("main", main_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);

    let fun = module.get_function("putchar");

    let test: &str = &str;
    let bytes: &[u8] = test.as_bytes();

    for n in 0..bytes.len() {
        builder.build_call(
            fun.unwrap(),
            &[i32_type.const_int(bytes[n] as u64, false).into()],
            "putchar",
        );
    }

    builder.build_return(Some(&i32_type.const_int(0, false)));

    module.print_to_file("a.ll").unwrap();

    let output = Command::new("lli-6.0").arg("a.ll").output().unwrap();

    Command::new("rm").args(&["-rf", "a.ll"]).output().unwrap();

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
