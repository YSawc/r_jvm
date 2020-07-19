use super::super::class::attribute::Attribute;
use super::super::class::{class_file, class_parser};
use super::super::gc::gc;
use super::super::stack::stack;

use std::vec::Vec;

pub struct VM {
    stack_machine: stack::StackMachine,
    gc: gc::ClassHeap,
    topic_class: class_file::ClassFile,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack_machine: stack::StackMachine::new(),
            gc: gc::ClassHeap::new(),
            topic_class: class_file::ClassFile::new(),
        }
    }
}

impl VM {
    pub fn run(&mut self, class_name: &str) -> () {
        let file_path = vec![
            "java/".to_string(),
            class_name.to_string(),
            ".class".to_string(),
        ]
        .join("");
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
        self.read_idx_code(0);
        self.drop_stack_machine();
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
        while n < v.len() {
            println!("n : {}, v[n] : {}", n, v[n]);
            match v[n] {
                Inst::iconst_m1..=Inst::iconst_5 => {
                    self.stack_machine.imm.push(v[n] - 3);
                    println!("self.stack_machine.imm : {:?}", self.stack_machine.imm);
                }
                Inst::bipush => {
                    n += 1;
                    self.stack_machine.imm.push(v[n]);
                }
                Inst::iload_0 => self.stack_machine.imm.push(self.stack_machine.i_st0 as u8),
                Inst::iload_1 => self.stack_machine.imm.push(self.stack_machine.i_st1 as u8),
                Inst::iload_2 => self.stack_machine.imm.push(self.stack_machine.i_st2 as u8),
                Inst::iload_3 => self.stack_machine.imm.push(self.stack_machine.i_st3 as u8),
                Inst::aload_0 => {}
                Inst::istore_1 => self.stack_machine.i_st1 = self.stack_machine.imm.pop()? as i8,
                Inst::istore_2 => self.stack_machine.i_st2 = self.stack_machine.imm.pop()? as i8,
                Inst::istore_3 => self.stack_machine.i_st3 = self.stack_machine.imm.pop()? as i8,
                Inst::pop => self.stack_machine.imm.push(self.stack_machine.op.pop()?),
                Inst::iadd => {
                    let ri = self.stack_machine.imm.pop()?;
                    let li = self.stack_machine.imm.pop()?;
                    let res = ri + li;
                    self.stack_machine.imm.push(res);
                }
                Inst::iinc => {
                    self.increment_i(v[n + 1], v[n + 2]);
                    println!("self.stack_machine : {:?}", self.stack_machine);
                    n += 2;
                }
                Inst::ifge => {
                    if self.stack_machine.imm.pop()? >= 0 {
                        let skip_count = index_to_next_to_goto(n as u8, v).unwrap();
                        n += skip_count as usize;
                    }
                    n += 2;
                }
                Inst::if_cmpge => {
                    if self.stack_machine.imm.pop() <= self.stack_machine.imm.pop() {
                        let skip_count = index_to_next_to_goto(n as u8, v).unwrap();
                        n += skip_count as usize;
                        n += 2;
                    } else {
                        n = v[n as usize + 2] as usize;
                    }
                }
                Inst::goto => n -= 7,
                Inst::ireturn => {
                    let ret_i = self.stack_machine.imm.pop().unwrap();
                    self.stack_machine.op.push(ret_i);
                    println!("{:?}", self.stack_machine);
                    return Some(());
                }
                Inst::_return => {
                    println!("{:?}", self.stack_machine);
                    return Some(());
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
                _ => unimplemented!(),
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
                _ => unimplemented!(),
            }
        }
        Some(())
    }

    pub fn push_to_i_st(&mut self, idx: u8) -> Option<()> {
        match idx {
            0 => self.stack_machine.i_st0 = self.stack_machine.imm.pop()? as i8,
            1 => self.stack_machine.i_st1 = self.stack_machine.imm.pop()? as i8,
            2 => self.stack_machine.i_st2 = self.stack_machine.imm.pop()? as i8,
            3 => self.stack_machine.i_st3 = self.stack_machine.imm.pop()? as i8,
            _ => unimplemented!(),
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

    pub fn drop_stack_machine(&mut self) -> () {
        self.stack_machine = stack::StackMachine::new();
    }

    pub fn increment_i(&mut self, idx: u8, c: u8) -> Option<()> {
        match idx {
            0 => self.stack_machine.i_st0 += c as i8,
            1 => self.stack_machine.i_st1 += c as i8,
            2 => self.stack_machine.i_st2 += c as i8,
            3 => self.stack_machine.i_st3 += c as i8,
            _ => unimplemented!(),
        }
        Some(())
    }
}

pub fn index_to_next_to_goto(c_idx: u8, v: &Vec<u8>) -> Option<u8> {
    let mut i = c_idx + 1;
    let segf_c = 128;
    while v[i as usize] != 167 {
        i += 1;
        if i >= segf_c {
            panic!();
        }
    }
    return Some(i - c_idx);
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
    pub const iload_0: u8 = 26;
    pub const iload_1: u8 = 27;
    pub const iload_2: u8 = 28;
    pub const iload_3: u8 = 29;
    pub const aload_0: u8 = 42;
    pub const istore_0: u8 = 59;
    pub const istore_1: u8 = 60;
    pub const istore_2: u8 = 61;
    pub const istore_3: u8 = 62;
    pub const pop: u8 = 87;
    pub const iadd: u8 = 96;
    pub const iinc: u8 = 132;
    pub const ifge: u8 = 156;
    pub const if_cmpge: u8 = 162;
    pub const goto: u8 = 167;
    pub const ireturn: u8 = 172;
    pub const _return: u8 = 177;
    pub const invoke_virtual: u8 = 182;
    pub const invoke_special: u8 = 183;
    pub const invoke_static: u8 = 184;
}
