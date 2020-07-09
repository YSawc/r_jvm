// use super::super::class::class_file;
use super::super::stack::stack;

use std::vec::Vec;

pub fn read_ope_code(v: &Vec<u8>) -> Option<u8> {
    let mut stack_machine = stack::StackMachine::new();

    for mut n in 0..v.len() {
        match v[n] {
            2..=8 => {
                stack_machine.imm.push(v[n] - 3);
            }
            16 => {
                n += 1;
                stack_machine.imm.push(v[n]);
            }
            27 => stack_machine.imm.push(stack_machine.i_st1 as u8),
            28 => stack_machine.imm.push(stack_machine.i_st2 as u8),
            60 => stack_machine.i_st1 = stack_machine.imm.pop()? as i8,
            61 => stack_machine.i_st2 = stack_machine.imm.pop()? as i8,
            62 => stack_machine.i_st3 = stack_machine.imm.pop()? as i8,
            96 => {
                let tmp = stack_machine.imm.pop()? + stack_machine.imm.pop()?;
                stack_machine.imm.push(tmp);
            }
            177 => {
                println!("stack_matchine : {:?}", stack_machine);
                return None;
            }
            _ => {}
        }
    }

    Some(0)
}
