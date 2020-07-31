use super::super::api::llvm;
use super::super::class::attribute::Attribute;
use super::super::class::{class_file, class_parser};
use super::super::gc::gc;
use super::super::stack::stack;
use std::collections::HashMap;
use std::vec::Vec;

pub struct VM {
    stack_machine: stack::StackMachine,
    gc: gc::ClassHeap,
    topic_class: class_file::ClassFile,
    variables: Vec<i64>,
    hashes: HashMap<u8, Vec<u8>>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack_machine: stack::StackMachine::new(),
            gc: gc::ClassHeap::new(),
            topic_class: class_file::ClassFile::new(),
            variables: vec![0; 255],
            hashes: HashMap::default(),
        }
    }
}

impl VM {
    pub fn run(&mut self, class_name: &str) -> () {
        let file_path = format!("java/{}.class", class_name);
        println!("========================================");
        println!("reading .. {}.", file_path);
        println!("========================================");

        let mut reader = match class_parser::ClassFileReader::new(&file_path) {
            Some(reader) => reader,
            _ => {
                eprintln!("{}: file not found.", file_path);
                panic!();
            }
        };

        self.gc.insert_class(class_name, reader.read().unwrap());
        self.topic_class = self.gc.get_class(class_name).unwrap().clone();
        self.hashes.insert(255, vec![0]);
        self.read_idx_code(0);
        self.drop_machine();
        Some(());
    }

    pub fn read_idx_code(&mut self, idx: u8) -> Option<()> {
        let code = match self.topic_class.methods[idx as usize].get_code_attribute() {
            Some(Attribute::Code { code, .. }) => code.clone(),
            _ => panic!(),
        };
        println!("code : {:?}", code);
        self.read_ope_code(&code);
        Some(())
    }

    pub fn read_ope_code(&mut self, v: &Vec<u8>) -> Option<()> {
        let mut n = 0;
        // println!("v.len() : {}", v.len());
        while n < v.len() {
            // println!("v[{}] : {}", n, v[n]);
            match v[n] {
                Inst::iconst_m1..=Inst::iconst_5 => {
                    self.stack_machine.imm.push(v[n] as i64 - 3);
                    // println!("self.stack_machine.imm : {:?}", self.stack_machine.imm);
                }
                Inst::bipush => {
                    n += 1;
                    self.stack_machine.imm.push(v[n] as i64);
                }
                Inst::pipush => {
                    println!("{:?}", self.stack_machine);
                    self.stack_machine
                        .imm
                        .push(((v[n + 1] * 255) as i64) + (v[n + 2] as i64 + 1));
                    n += 2;
                }
                Inst::ldc => {
                    let string_index = self.topic_class.constant_pool[v[n + 1] as usize - 1]
                        .get_string_index()
                        .unwrap();
                    let output = self.topic_class.constant_pool[string_index as usize - 1]
                        .get_utf8()
                        .unwrap();
                    self.stack_machine.output_stream_st.push(output.clone());
                    // println!("{:?}", self.stack_machine);
                    n += 1;
                }
                Inst::iload => {
                    self.stack_machine
                        .imm
                        .push(self.variables[v[n + 1] as usize] as i64);
                    n += 1;
                }
                Inst::iload_0 => self.stack_machine.imm.push(self.stack_machine.i_st0 as i64),
                Inst::iload_1 => self.stack_machine.imm.push(self.stack_machine.i_st1 as i64),
                Inst::iload_2 => self.stack_machine.imm.push(self.stack_machine.i_st2 as i64),
                Inst::iload_3 => self.stack_machine.imm.push(self.stack_machine.i_st3 as i64),
                Inst::aload_0 => {}
                Inst::istore => {
                    self.variables[v[n + 1] as usize] =
                        self.stack_machine.imm.pop().unwrap() as i64;
                    println!(
                        "self.variables[v[n + 1] as usize] : {}",
                        self.variables[v[n + 1] as usize]
                    );
                    n += 1;
                }
                Inst::istore_1 => self.stack_machine.i_st1 = self.stack_machine.imm.pop()? as i32,
                Inst::istore_2 => self.stack_machine.i_st2 = self.stack_machine.imm.pop()? as i32,
                Inst::istore_3 => self.stack_machine.i_st3 = self.stack_machine.imm.pop()? as i32,
                Inst::astore_1..=Inst::astore_3 => {
                    let store_idx = v[n] - 75;
                    match v[n + 2] {
                        Inst::iconst_m1..=Inst::iconst_5 => {
                            let arr_idx = v[n + 3];
                            let insert_num = v[n + 4] - 3;
                            self.astore(store_idx, arr_idx, insert_num);
                            n += 4;
                        }
                        _ => {
                            let arr_idx = v[n + 3];
                            let insert_num = v[n + 4] - 3;
                            self.astore(store_idx, arr_idx, insert_num);
                            n += 5;
                        }
                    }
                    println!("{:?}", self.hashes);
                }
                Inst::pop => self
                    .stack_machine
                    .imm
                    .push(self.stack_machine.op.pop()? as i64),
                Inst::new => n += 2,
                Inst::iadd => {
                    let ri = self.stack_machine.imm.pop()?;
                    let li = self.stack_machine.imm.pop()?;
                    let res = ri + li;
                    self.stack_machine.imm.push(res);
                }
                Inst::irem => {
                    let ri = self.stack_machine.imm.pop()?;
                    let li = self.stack_machine.imm.pop()?;
                    let res = li - (li / ri) * ri;
                    self.stack_machine.imm.push(res);
                }
                Inst::iinc => {
                    self.increment_i(v[n + 1], v[n + 2]);
                    // println!(
                    //     "self.stack_machine after read iinc : {:?}",
                    //     self.stack_machine
                    // );
                    n += 2;
                }
                Inst::ifeq => {
                    let t = self.stack_machine.imm.pop()?;
                    if t == 0 {
                        n += (v[n as usize + 1] as usize >> 8) + v[n as usize + 2] as usize - 1;
                    } else {
                        n += 2;
                    }
                }
                Inst::ifne => {
                    let t = self.stack_machine.imm.pop()?;
                    if t != 0 {
                        n += (v[n as usize + 1] as usize >> 8) + v[n as usize + 2] as usize - 1;
                    } else {
                        n += 2;
                    }
                }
                Inst::ifge => {
                    let t = self.stack_machine.imm.pop()?;
                    if t >= 0 {
                        n += (v[n as usize + 1] as usize >> 8) + v[n as usize + 2] as usize - 1;
                    } else {
                        n += 2;
                    }
                }
                Inst::if_cmpge => {
                    self.stack_machine.imp_i = check_loop_base(n as i32, v).unwrap() as i32;
                    if self.stack_machine.imm.pop() <= self.stack_machine.imm.pop() {
                        n = v[n + (v[n as usize + 1] as usize >> 8) + v[n as usize + 2] as usize]
                            as usize;
                    } else {
                        n += 2;
                    }
                }
                Inst::if_cmpgt => {
                    self.stack_machine.imp_i = check_loop_base(n as i32, v).unwrap();
                    if self.stack_machine.imm.pop() < self.stack_machine.imm.pop() {
                        n = v[n + (v[n as usize + 1] as usize >> 8) + v[n as usize + 2] as usize]
                            as usize;
                    } else {
                        n += 2;
                    }
                }
                Inst::goto => match v[n as usize + 1] {
                    255 => n = self.stack_machine.imp_i as usize - 1,
                    _ => n += (v[n as usize + 1] as usize >> 8) + v[n as usize + 2] as usize - 1,
                },
                Inst::lookupswitch => {
                    n += 1;
                    while v[n] == 0 {
                        n += 1;
                    }

                    let mut idx_hs: HashMap<u8, u8> = HashMap::default();
                    idx_hs.insert(255, v[n]);
                    n += 4;
                    let loop_c = v[n];
                    n += 1;
                    for _ in 0..loop_c {
                        idx_hs.insert(v[n + 3], v[n + 7]);
                        n += 8;
                    }

                    let imm = self.stack_machine.imm.pop().unwrap();

                    let goto_idx: u8 = match idx_hs.get_mut(&(imm as u8)) {
                        Some(n) => *n,
                        _ => *idx_hs.get_mut(&255).unwrap(),
                    };
                    n = goto_idx as usize;
                }
                Inst::ireturn => {
                    let ret_i = self.stack_machine.imm.pop().unwrap();
                    self.stack_machine.op.push(ret_i);
                    println!(
                        "self.stack_machine after read ireturn : {:?}",
                        self.stack_machine
                    );
                    // println!("self.variables after read ireturn : {:?}", self.variables);
                    return Some(());
                }
                Inst::_return => {
                    println!(
                        "self.stack_machine after read _return : {:?}",
                        self.stack_machine
                    );
                    // println!("self.variables after read _return : {:?}", self.variables);
                    return Some(());
                }
                Inst::getstatic => {
                    let field_ref_info_index = (v[n + 1] as usize >> 8) | v[n + 2] as usize;
                    self.get_field_info(field_ref_info_index);
                    n += 2;
                }
                Inst::invoke_virtual => {
                    // println!("{:?}", self.topic_class.constant_pool);

                    let (method_class_index, method_name_and_type_index) = self
                        .topic_class
                        .constant_pool[(v[n + 1] as usize >> 8) | v[n + 2] as usize - 1]
                        .get_method_indexes()
                        .unwrap();

                    let class_class_index = self.topic_class.constant_pool
                        [method_class_index as usize - 1]
                        .get_class_class_index()
                        .unwrap();
                    let field_summary = self.topic_class.constant_pool
                        [class_class_index as usize - 1]
                        .get_utf8()
                        .unwrap();

                    let (name_index, descriptor_index) = self.topic_class.constant_pool
                        [method_name_and_type_index as usize - 1]
                        .get_name_and_type_indexes()
                        .unwrap();
                    let detail_of_constructor = self.topic_class.constant_pool
                        [name_index as usize - 1]
                        .get_utf8()
                        .unwrap()
                        .clone();
                    let types_info = self.topic_class.constant_pool[descriptor_index as usize - 1]
                        .get_utf8()
                        .unwrap()
                        .clone();

                    println!("{}.{}:{}", field_summary, detail_of_constructor, types_info);

                    self.stack_machine
                        .class_stream_st
                        .push(field_summary.to_owned() + ":" + &detail_of_constructor);

                    self.invoke_virtual(types_info);
                    n += 2;
                }
                Inst::invoke_special => {
                    let idx = self.search_special_methods_index().unwrap();
                    n += 2;
                    self.read_idx_code(idx);
                }
                Inst::invoke_static => {
                    let idx = self.search_invoke_static_index((v[n + 1]) as u8).unwrap();

                    self.parse_args_and_return_value(v[n + 1]);
                    n += 2;
                    self.read_idx_code(idx).unwrap();
                }
                Inst::dup => n += 2,
                Inst::newarray => {
                    let arr_count = self.hashes.get_mut(&255).unwrap()[0];
                    self.new_array(arr_count);
                    println!("stackmachine read after newarray {:?}", self.stack_machine);
                    println!("hashes read after newarray{:?}", self.hashes);
                    n += 1;
                }
                e => unimplemented!("{}", e),
            }
            n += 1;
        }
        None
    }

    pub fn parse_args_and_return_value(&mut self, idx: u8) -> Option<()> {
        let topic_class = self.topic_class.clone();
        let method_name_and_type_index = topic_class.constant_pool[idx as usize]
            .get_method_name_and_type_index()
            .unwrap();
        let descriptor_index = topic_class.constant_pool[method_name_and_type_index as usize]
            .get_name_and_type_name_index()
            .unwrap();
        let type_info = topic_class.constant_pool[descriptor_index as usize]
            .get_utf8()
            .unwrap();
        println!("type_info : {}", type_info);

        self.parse_args(type_info);
        Some(())
    }

    pub fn parse_args(&mut self, str: &String) -> Option<()> {
        for n in 0..str.len() {
            match str.as_bytes()[n] as char {
                '(' => {}
                'I' => self.push_to_i_st(n as u8 - 1).unwrap(),
                ')' => return Some(()),
                e => unimplemented!("{}", e),
            }
        }
        Some(())
    }

    pub fn push_to_i_st(&mut self, idx: u8) -> Option<()> {
        match idx {
            0 => self.stack_machine.i_st0 = self.stack_machine.imm.pop()? as i32,
            1 => self.stack_machine.i_st1 = self.stack_machine.imm.pop()? as i32,
            2 => self.stack_machine.i_st2 = self.stack_machine.imm.pop()? as i32,
            3 => self.stack_machine.i_st3 = self.stack_machine.imm.pop()? as i32,
            _ => panic!(),
        }
        Some(())
    }
    pub fn search_special_methods_index(&mut self) -> Option<u8> {
        for n in 0..self.topic_class.methods_count as u8 {
            if self.topic_class.constant_pool
                [self.topic_class.methods[n as usize].name_index as usize - 1]
                .get_utf8()?
                == "main"
            {
                return Some(n as u8);
            }
        }

        None
    }

    pub fn search_invoke_static_index(&mut self, idx: u8) -> Option<u8> {
        let name_and_type_index = self.topic_class.constant_pool[idx as usize]
            .get_method_name_and_type_index()
            .unwrap();
        let name_index = self.topic_class.constant_pool[name_and_type_index as usize]
            .get_name_and_type_name_index()
            .unwrap();

        for n in 0..=self.topic_class.attributes_count as u8 {
            if self.topic_class.methods[n as usize].name_index == name_index {
                return Some(n as u8);
            }
        }

        None
    }

    pub fn drop_machine(&mut self) -> () {
        self.drop_stack_machine();
        self.drop_variables();
        self.drop_hashes();
    }

    pub fn drop_stack_machine(&mut self) -> () {
        self.stack_machine = stack::StackMachine::new();
    }

    pub fn drop_variables(&mut self) -> () {
        self.variables = vec![0; 255];
    }

    pub fn drop_hashes(&mut self) -> () {
        self.hashes = HashMap::default();
    }

    pub fn increment_i(&mut self, idx: u8, c: u8) -> Option<()> {
        match idx {
            0 => self.stack_machine.i_st0 += c as i32,
            1 => self.stack_machine.i_st1 += c as i32,
            2 => self.stack_machine.i_st2 += c as i32,
            3 => self.stack_machine.i_st3 += c as i32,
            _ => self.variables[idx as usize] += c as i64,
        }
        // println!("{:?}", self.variables);
        Some(())
    }

    pub fn new_array(&mut self, arr_count: u8) -> Option<()> {
        self.hashes.get_mut(&255).unwrap()[0] += 1;
        match arr_count {
            0 => self.stack_machine.a_st1 = vec![0; self.stack_machine.imm.pop().unwrap() as usize],
            1 => self.stack_machine.a_st2 = vec![0; self.stack_machine.imm.pop().unwrap() as usize],
            2 => self.stack_machine.a_st3 = vec![0; self.stack_machine.imm.pop().unwrap() as usize],
            _ => {
                self.hashes.insert(
                    arr_count,
                    vec![0; self.stack_machine.imm.pop().unwrap() as usize],
                );
                ()
            }
        }
        Some(())
    }

    pub fn astore(&mut self, store_idx: u8, arr_idx: u8, insert_num: u8) -> Option<()> {
        // println!("store_idx : {}", store_idx);
        // println!("arr_idx : {}", arr_idx);
        // println!("insert_num : {}", insert_num);
        match store_idx {
            1 => self.stack_machine.a_st1[arr_idx as usize] = insert_num as i32,
            2 => self.stack_machine.a_st2[arr_idx as usize] = insert_num as i32,
            3 => self.stack_machine.a_st3[arr_idx as usize] = insert_num as i32,
            _ => self.hashes.get_mut(&store_idx).unwrap()[arr_idx as usize] = insert_num,
        }

        Some(())
    }

    pub fn invoke_virtual(&mut self, str: String) {
        println!("{:?}", self.stack_machine);
        match self.stack_machine.class_stream_st.pop().unwrap().as_str() {
            "java/io/PrintStream:println" => match &*str {
                "(Ljava/lang/String;)V" => {
                    llvm::inkwell::println_str(self.stack_machine.output_stream_st.pop().unwrap())
                }
                "(I)V" => {
                    let v = self.stack_machine.imm.pop().unwrap();
                    let mut a: Vec<i64> = vec![];
                    let mut c: i64 = 1;
                    while v / c > 0 {
                        a.push(v / c);
                        c *= 10;
                    }
                    a.reverse();
                    llvm::inkwell::println_int(a);
                }
                _ => unimplemented!(),
            },
            "java/lang/StringBuilder:append" => {}
            e => unimplemented!("{}", e),
        }
    }

    pub fn get_field_info(&mut self, u: usize) {
        let (class_index, name_and_type_index) = self.topic_class.constant_pool[u as usize - 1]
            .get_field_ref_indexes()
            .unwrap();

        let class_index2 = self.topic_class.constant_pool[class_index as usize - 1]
            .get_class_class_index()
            .unwrap();
        let base_class = self.topic_class.constant_pool[class_index2 as usize - 1]
            .get_utf8()
            .unwrap();

        let (name_index, name_and_type_name_index) = self.topic_class.constant_pool
            [name_and_type_index as usize - 1]
            .get_name_and_type_indexes()
            .unwrap();
        let stream = self.topic_class.constant_pool[name_index as usize - 1]
            .get_utf8()
            .unwrap();
        let field_class = self.topic_class.constant_pool[name_and_type_name_index as usize - 1]
            .get_utf8()
            .unwrap();

        println!("{}.{}:{}", base_class, stream, field_class);
    }
}

fn check_loop_base(idx: i32, v: &Vec<u8>) -> Option<i32> {
    for i in 0..(v.len() as i32 - idx) {
        match v[idx as usize - i as usize] {
            21 | 27 | 28 | 29 => return Some(idx - i),
            _ => {}
        }
    }
    None
}

#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod Inst {
    pub type Code = u8;
    pub const iconst_m1: u8 = 2;
    pub const iconst_0: u8 = 3;
    pub const iconst_1: u8 = 4;
    pub const iconst_2: u8 = 5;
    pub const iconst_3: u8 = 6;
    pub const iconst_4: u8 = 7;
    pub const iconst_5: u8 = 8;
    pub const bipush: u8 = 16;
    pub const pipush: u8 = 17;
    pub const ldc: u8 = 18;
    pub const iload: u8 = 21;
    pub const iload_0: u8 = 26;
    pub const iload_1: u8 = 27;
    pub const iload_2: u8 = 28;
    pub const iload_3: u8 = 29;
    pub const aload_0: u8 = 42;
    pub const istore: u8 = 54;
    pub const istore_0: u8 = 59;
    pub const istore_1: u8 = 60;
    pub const istore_2: u8 = 61;
    pub const pop: u8 = 87;
    pub const dup: u8 = 89;
    pub const istore_3: u8 = 62;
    pub const astore_1: u8 = 76;
    pub const astore_2: u8 = 77;
    pub const astore_3: u8 = 78;
    pub const iadd: u8 = 96;
    pub const irem: u8 = 112;
    pub const iinc: u8 = 132;
    pub const ifeq: u8 = 153;
    pub const ifne: u8 = 154;
    pub const ifge: u8 = 156;
    pub const if_cmpge: u8 = 162;
    pub const if_cmpgt: u8 = 163;
    pub const goto: u8 = 167;
    pub const lookupswitch: u8 = 171;
    pub const ireturn: u8 = 172;
    pub const _return: u8 = 177;
    pub const getstatic: u8 = 178;
    pub const invoke_virtual: u8 = 182;
    pub const invoke_special: u8 = 183;
    pub const invoke_static: u8 = 184;
    pub const new: u8 = 187;
    pub const newarray: u8 = 188;
}
