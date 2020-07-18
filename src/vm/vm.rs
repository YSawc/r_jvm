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

    pub fn read_ope_code(&mut self, class: &class_file::ClassFile, v: &Vec<u8>) -> Option<()> {
        // println!("{}", v.len());

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
                    let idx = search_special_methods_index(class).unwrap();
                    n += 2;
                    self.read_idx_code(class, idx);
                }
                Inst::invoke_static => {
                    let idx = search_invoke_static_index(class, (v[n + 1]) as u8).unwrap();

                    self.parse_args_and_return_value(class, v[n + 1]);
                    n += 2;
                    self.read_idx_code(class, idx).unwrap();
                }
                _ => unimplemented!(),
            }
            n += 1;
        }
        None
    }

    pub fn parse_args_and_return_value(
        &mut self,
        class: &class_file::ClassFile,
        idx: u8,
    ) -> Option<()> {
        let method_name_and_type_index = class.constant_pool[idx as usize]
            .get_method_name_and_type_index()
            .unwrap();
        let descriptor_index = class.constant_pool[method_name_and_type_index as usize]
            .get_name_and_type_name_index()
            .unwrap();
        let type_info = class.constant_pool[descriptor_index as usize]
            .get_utf8()
            .unwrap();
        println!("{:?}", type_info);
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
}

pub fn search_special_methods_index(class: &class_file::ClassFile) -> Option<u8> {
    for n in 0..class.methods_count as u8 {
        if class.constant_pool[class.methods[n as usize].name_index as usize - 1].get_utf8()?
            == "main"
        {
            return Some(n as u8);
        }
    }

    None
}

pub fn search_invoke_static_index(class: &class_file::ClassFile, idx: u8) -> Option<u8> {
    let name_and_type_index = class.constant_pool[idx as usize]
        .get_method_name_and_type_index()
        .unwrap();
    let name_index = class.constant_pool[name_and_type_index as usize]
        .get_name_and_type_name_index()
        .unwrap();

    for n in 0..=class.attributes_count as u8 {
        if class.methods[n as usize].name_index == name_index {
            return Some(n as u8);
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
    pub const aload_0: u8 = 42;
    pub const istore_0: u8 = 59;
    pub const istore_1: u8 = 60;
    pub const istore_2: u8 = 61;
    pub const istore_3: u8 = 62;
    pub const pop: u8 = 87;
    pub const iadd: u8 = 96;
    pub const ireturn: u8 = 172;
    pub const _return: u8 = 177;
    pub const invoke_virtual: u8 = 182;
    pub const invoke_special: u8 = 183;
    pub const invoke_static: u8 = 184;
}
