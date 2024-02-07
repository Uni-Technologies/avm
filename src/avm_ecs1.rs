use std::io::Write;

use crate::avm_bcs1::{add, add_from, set, copy_to, out, out_range, goto1, goto2, if_op, read1, read_range, string_writer, multiply, sleep1, sleep2 };
use crate::vmcodes::VMCode;
use crate::base::Memory;

pub fn division(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    let a = memory.get_by(_args[0] as usize);
    let b = memory.get_by(_args[1] as usize);
    memory.set_by(_args[0] as usize, a/b);
    return VMCode::Success;
}

// Signature: 0x0 0x1 0x2 0x3
pub fn dwr1(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode 
{
    let a = memory.get_by(_args[0] as usize);
    let b = memory.get_by(_args[1] as usize);
    let c = a % b;
    memory.set_by(_args[2] as usize, c);
    return VMCode::Success;
}

pub fn dwr2(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode 
{
    let a = memory.get_by(_args[0] as usize);
    let b = _args[1];
    let c = a % b;
    memory.set_by(_args[2] as usize, c);
    return VMCode::Success;
}

pub fn out_itself(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    print!("{}", memory.get_by(_args[0] as usize));
    std::io::stdout().flush().unwrap();
    return VMCode::Success;
}

pub fn shutdown(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    std::process::exit(VMCode::Success as i32);
    return VMCode::Success;
}

pub fn get_instructions_for_ecs1() ->Vec<(i32, fn(&mut Memory, Vec<i32>,&mut i32) -> VMCode)> {
    return vec![
        (0x00, add_from),
        (0x01, set),
        (0x02, copy_to),
        (0x03, out),
        (0x04, out_range),
        (0x05, goto1),
        (0x06, goto2),
        (0x07, if_op),
        (0x08, add),
        (0x09, read1),
        (0x0A, read_range),
        (0x0B, string_writer),
        (0x0C, multiply),
        (0x0D, sleep1),
        (0x0E, sleep2),
        (0x0F, division),
        (0x10, dwr1),
        (0x11, dwr2),
        (0x12, out_itself),
        (0x13, shutdown),
    ];
}