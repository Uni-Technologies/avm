use std::{io::Write, thread, time::Duration};

#[allow(unused_mut)]
#[allow(unused_variables)]
#[allow(unused)]
use crate::{base::Memory, vmcodes::VMCode};

pub fn add_from(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    let a = memory.get_by(_args[0] as usize);
    let b = memory.get_by(_args[1] as usize);
    memory.set_by(_args[0] as usize, a + b);
    return VMCode::Success;
}

pub fn add(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    let a = memory.get_by(_args[0] as usize);
    memory.set_by(_args[0] as usize, a + _args[1]);
    return VMCode::Success;
}

pub fn set(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    memory.set_by(_args[0] as usize, _args[1]);
    return VMCode::Success;
}

pub fn copy_to(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    let a = memory.get_by(_args[0] as usize);
    memory.set_by(_args[1] as usize, a);
    return VMCode::Success;
}

pub fn out(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    let ch = char::from_u32(memory.get_by(_args[0] as usize) as u32).expect("out error");
    print!("{}", ch);
    std::io::stdout().flush().unwrap();
    return VMCode::Success;
}

pub fn out_range(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    
    for i in _args[0] as usize.._args[1]  as usize{
        
        let a = memory.get_by(i) ;
        if a >= 32
        {
            let ch = char::from_u32(a as u32).expect("out range error");
            eprint!("{}", ch);
        }
        else {
            
            return VMCode::Success;
        }
    }
    
    return VMCode::Success;
}

pub fn goto1(_memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    *_curr_op = _args[0]-1;
    return VMCode::Success;
}

pub fn goto2(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    *_curr_op = memory.get_by(_args[0] as usize)-1;
    return VMCode::Success;
}

pub fn if_op(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    let should: bool;
    let a = memory.get_by(_args[0] as usize);
    let b = memory.get_by(_args[2] as usize);
    match _args[1] {
        0x0 => 
        {
            should = a == b;
        }
        0x1 => 
        {
            should = a != b;
        }
        0x2 => 
        {
            should = a > b;
        }
        0x3 => 
        {
            should = a < b;
        }
        0x4 => 
        {
            should = a >= b;
        }
        0x5 => 
        {
            should = a <= b;
        }
        _ => return VMCode::UnknownLogicalOperation
    }

    if should {
        *_curr_op = _args[3]-1;
    }

    return VMCode::Success;
}

pub fn read1(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    let mut input = String::new();
    let _str = std::io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    let byte = input.bytes().nth(0).expect("no byte read");

    memory.set_by(_args[0] as usize, byte as i32);
    return VMCode::Success;
}

pub fn read_range(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    let mut input = String::new();
    let _str: usize = std::io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    let _byte = input.as_bytes();

    let mut curr_index = _args[0];
    for i in 0.._args[1]-_args[0] {
        if i < _byte.len() as i32
        {
            let a = _byte[i as usize];
            memory.set_by(curr_index as usize, a as i32);
            curr_index += 1;
        }
        else {
            return VMCode::Success;
        }
    }

    return VMCode::Success;
}

pub fn string_writer(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    let mut start_ind = _args[0];
    for i in 1.._args.len() {
        memory.set_by(start_ind as usize, _args[i]);
        start_ind+=1;
    }
    return VMCode::Success;
}

pub fn multiply(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    let a = memory.get_by(_args[0] as usize);
    memory.set_by(_args[0] as usize, a * _args[1]);
    return VMCode::Success;
}

pub fn sleep1(_memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    let duration: Duration = Duration::from_millis(_args[0] as u64);
    thread::sleep(duration);
    return VMCode::Success;
}

pub fn sleep2(memory: &mut Memory, _args: Vec<i32>, _curr_op: &mut i32) -> VMCode {
    let duration: Duration = Duration::from_millis(memory.get_by(_args[0] as usize) as u64);
    thread::sleep(duration);
    return VMCode::Success;
}

pub fn get_instructions_for_bcs1() ->Vec<(i32, fn(&mut Memory, Vec<i32>,&mut i32) -> VMCode)> {
    return vec![
        (0x0, add_from),
        (0x1, set),
        (0x2, copy_to),
        (0x3, out),
        (0x4, out_range),
        (0x5, goto1),
        (0x6, goto2),
        (0x7, if_op),
        (0x8, add),
        (0x9, read1),
        (0xA, read_range),
        (0xB, string_writer),
        (0xC, multiply),
        (0xD, sleep1),
        (0xE, sleep2)
    ];
}