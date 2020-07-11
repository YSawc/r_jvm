// use super::super::class::class_file;
use super::super::stack::stack;

use std::vec::Vec;

pub fn read_ope_code(v: &Vec<u8>) -> Option<u8> {
    let mut stack_machine = stack::StackMachine::new();

    for mut n in 0..v.len() {
        match v[n] {
            Inst::iconst_m1..=Inst::iconst_5 => {
                stack_machine.imm.push(v[n] - 3);
            }
            Inst::bipush => {
                n += 1;
                stack_machine.imm.push(v[n]);
            }
            Inst::iload_1 => stack_machine.imm.push(stack_machine.i_st1 as u8),
            Inst::iload_2 => stack_machine.imm.push(stack_machine.i_st2 as u8),
            Inst::iload_3 => stack_machine.imm.push(stack_machine.i_st3 as u8),
            Inst::istore_1 => stack_machine.i_st1 = stack_machine.imm.pop()? as i8,
            Inst::istore_2 => stack_machine.i_st2 = stack_machine.imm.pop()? as i8,
            Inst::istore_3 => stack_machine.i_st3 = stack_machine.imm.pop()? as i8,
            Inst::iadd => {
                let tmp = stack_machine.imm.pop()? + stack_machine.imm.pop()?;
                stack_machine.imm.push(tmp);
            }
            Inst::ireturn => {
                let ret_v = stack_machine.imm.pop();
                println!("stack_matchine : {:?}", stack_machine);
                return ret_v;
            }
            Inst::_return => {
                println!("stack_matchine : {:?}", stack_machine);
                return None;
            }
            _ => {}
        }
    }

    Some(0)
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
}
