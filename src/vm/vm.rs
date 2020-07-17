use super::super::class::attribute::Attribute;
use super::super::class::class_file;
use super::super::gc::gc;
use super::super::stack::stack;

use std::vec::Vec;

pub struct VM {
    stack_machine: stack::StackMachine,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack_machine: stack::StackMachine::new(),
        }
    }
}

impl VM {
    pub fn run(&mut self, gc: gc::ClassHeap, class_name: &str) -> Option<()> {
        let class = &gc.get_class(class_name).unwrap();

        // println!("{:?}", class);
        // println!("{:?}", class.constant_pool);
        // let methods = &class.methods;
        // println!("methods : {:?}", methods);
        // println!("descriptor_index : {:?}", methods[0].descriptor_index);

        // let (code, attributes) = if let Some(Attribute::Code {
        //     code, attributes, ..
        // // }) = methods[idx as usize].get_code_attribute()
        // }) = methods[0 as usize].get_code_attribute()
        // {
        //     (code, attributes)
        // } else {
        //     panic!()
        // };
        // println!("{:?}", _code_length);
        // println!("code : {:?}", code);
        // println!("attributes : {:?}", attributes);

        // println!( "attributes[0].attribute_name_index: {:?}", attributes[0].attribute_name_index );
        // println!(
        //     "Method: {:?}:{:?}",
        //     class.constant_pool[attributes[0].attribute_name_index as usize].get_utf8()?,
        //     class.constant_pool[methods[0].descriptor_index as usize - 1].get_utf8()?
        // );

        // println!("return value : {:?}", read_ope_code(code));
        // println!("return value : {:?}", self.read_idx_code(class, 0))

        self.read_idx_code(class, 0);
        Some(())
    }

    pub fn read_idx_code(&mut self, class: &class_file::ClassFile, idx: u8) -> Option<()> {
        let code = match class.methods[idx as usize].get_code_attribute() {
            Some(Attribute::Code { code, .. }) => code,
            _ => panic!(),
        };
        println!("code : {:?}", code);
        self.read_ope_code(class, code);
        Some(())
    }

    pub fn read_ope_code(&mut self, class: &class_file::ClassFile, v: &Vec<u8>) -> Option<u8> {
        for mut _n in 0..v.len() {
            match v[_n] {
                Inst::iconst_m1..=Inst::iconst_5 => {
                    self.stack_machine.imm.push(v[_n] - 3);
                }
                Inst::bipush => {
                    _n += 1;
                    self.stack_machine.imm.push(v[_n]);
                }
                Inst::iload_1 => self.stack_machine.imm.push(self.stack_machine.i_st1 as u8),
                Inst::iload_2 => self.stack_machine.imm.push(self.stack_machine.i_st2 as u8),
                Inst::iload_3 => self.stack_machine.imm.push(self.stack_machine.i_st3 as u8),
                Inst::istore_1 => self.stack_machine.i_st1 = self.stack_machine.imm.pop()? as i8,
                Inst::istore_2 => self.stack_machine.i_st2 = self.stack_machine.imm.pop()? as i8,
                Inst::istore_3 => self.stack_machine.i_st3 = self.stack_machine.imm.pop()? as i8,
                Inst::iadd => {
                    let tmp = self.stack_machine.imm.pop()? + self.stack_machine.imm.pop()?;
                    self.stack_machine.imm.push(tmp);
                }
                Inst::ireturn => {
                    let ret_v = self.stack_machine.imm.pop();
                    println!("{:?}", self.stack_machine);
                    return ret_v;
                }
                Inst::_return => {
                    println!("{:?}", self.stack_machine);
                    return None;
                }
                Inst::invoke_virtual => {
                    let idx = search_invoke_virtual_index(class, (_n + 2) as u8).unwrap();
                    _n += 2;
                    self.read_idx_code(class, idx);
                }
                Inst::invoke_special => {
                    let idx = search_special_methods(class).unwrap();
                    println!("search_special_methods idx : {}", idx);
                    self.read_idx_code(class, idx);
                    _n += 2;
                }
                _ => {}
            }
        }
        None
    }
}

pub fn search_invoke_virtual_index(class: &class_file::ClassFile, idx: u8) -> Option<u8> {
    let name_and_type_index = class.constant_pool[idx as usize]
        .get_method_name_and_type_index()
        .unwrap();
    let name_index = class.constant_pool[name_and_type_index as usize]
        .get_name_and_type_name_index()
        .unwrap();

    let attribute_count = match class.methods[0 as usize].get_code_attribute() {
        Some(Attribute::Code {
            attributes_count, ..
        }) => attributes_count,
        _ => panic!(),
    };

    for n in 0..*attribute_count as u8 {
        if class.methods[n as usize].name_index == name_index {
            return Some(n + 1 as u8);
        }
    }

    None
}

pub fn search_special_methods(class: &class_file::ClassFile) -> Option<u8> {
    let (attribute_count, attributes) = match class.methods[0 as usize].get_code_attribute() {
        Some(Attribute::Code {
            attributes_count,
            attributes,
            ..
        }) => (attributes_count, attributes),
        _ => panic!(),
    };

    for n in 0..*attribute_count as u8 {
        if class.constant_pool[attributes[n as usize].attribute_name_index as usize].get_utf8()?
            == "main"
        {
            return Some(n + 1 as u8);
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
    pub const iload_0: u8 = 26;
    pub const iload_1: u8 = 27;
    pub const iload_2: u8 = 28;
    pub const iload_3: u8 = 29;
    pub const istore_0: u8 = 59;
    pub const istore_1: u8 = 60;
    pub const istore_2: u8 = 61;
    pub const istore_3: u8 = 62;
    pub const iadd: u8 = 96;
    pub const ireturn: u8 = 172;
    pub const _return: u8 = 177;
    pub const invoke_virtual: u8 = 182;
    pub const invoke_special: u8 = 183;
}
