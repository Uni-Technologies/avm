use crate::avm_bcs1::{add, add_from, set, copy_to, out, out_range, goto1, goto2, if_op, read1, read_range, string_writer, multiply, sleep1, sleep2 };
use crate::avm_ecs1::{division, dwr1, dwr2, shutdown, out_itself};
use crate::vmcodes::VMCode;
use crate::base::Memory;
use std::io::Write;


pub fn change_bg1(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    let mut num = _args[0];
    print!("\x1b[48;5;{}m", num);
    std::io::stdout().flush().unwrap();
    return VMCode::Success;
}

pub fn change_bg2(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    let mut num = memory.get_by(_args[0] as usize);
    print!("\x1b[48;5;{}m", num);
    std::io::stdout().flush().unwrap();
    return VMCode::Success;
}

pub fn change_fg1(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    let mut num = _args[0];
    print!("\x1b[38;5;{}m", num);
    std::io::stdout().flush().unwrap();
    return VMCode::Success;
}

pub fn change_fg2(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    let mut num = memory.get_by(_args[0] as usize);
    print!("\x1b[38;5;{}m", num);
    std::io::stdout().flush().unwrap();
    return VMCode::Success;
}

pub fn clear_scrn(_memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    print!("\x1b[2J");
    std::io::stdout().flush().unwrap();
    return VMCode::Success;
}

pub fn move_cursor(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    let direction = match memory.get_by(_args[0] as usize) {
        0x0 => "A",
        0x1 => "B",
        0x2 => "C",
        0x3 => "D",
        _ => "A"
    };
    let num = memory.get_by(_args[1] as usize);
    print!("\x1b[{0}{1}", num, direction);
    std::io::stdout().flush().unwrap();
    return VMCode::Success;
}

pub fn set_cursor(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    let a = memory.get_by(_args[0] as usize);
    let b = memory.get_by(_args[1] as usize);
    print!("\x1b[{};{}H", a, b);
    std::io::stdout().flush().unwrap();
    return VMCode::Success;
}

pub fn toggle_cursor(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    let enabled = match memory.get_by(_args[0] as usize) {
        0x0 => "?25l",
        0x1 => "?25h",
        _ => "?25h"
    };
    print!("\x1b[{}", enabled);
    std::io::stdout().flush().unwrap();
    return VMCode::Success;
}

pub fn get_instructions_for_uacs1() ->Vec<(i32, fn(&mut Memory, Vec<i32>,&mut i32) -> VMCode)> {
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
        (0x14, change_bg1),
        (0x15, change_bg2),
        (0x16, change_fg1),
        (0x17, change_fg2),
        (0x18, clear_scrn),
        (0x19, move_cursor),
        (0x1A, toggle_cursor),
        (0x1B, set_cursor)
    ];
}