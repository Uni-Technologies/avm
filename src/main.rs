#![allow(unused)]

// Base
mod base;
mod vmcodes;

// Architectures
mod avm_bcs1;
mod avm_ecs1;
mod avm_uacs1;

// Uses
use base::Memory;
use base::VMTypes;
use vmcodes::VMCode;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::vec;


// Main function
fn main_main() {
    // Create Memory
    let mut base_mem = Memory {
        itself: [0; 0xFFFF]
    };

    let args: Vec<String> = std::env::args().collect();

    let mut type_of_vm: VMTypes = VMTypes::BCS1;
    let mut path: String;
    if args.len() > 2
    {
        if args.contains(&String::from("--ecs1")) { type_of_vm = VMTypes::ECS1; }
        if args.contains(&String::from("--uacs1")) { type_of_vm = VMTypes::UACS1; }
        path = args[1].to_string();
    }
    else {
        path = args[1].to_string();
    }

    let operations = load_from_file(path);

    start_up(&mut base_mem, operations, type_of_vm);
}

fn test_main() {
    let mut base_mem = Memory {
        itself: [0; 0xFFFF]
    };
    let operations: Vec<Vec<i32>> = vec![
        vec![0x0B, 0x0, 0x21, 0x21, 0x21, 0x21,], //1
        vec![0x0A, 0x4, 0x7], // 2
        vec![0x7, 0x0, 0x0, 0x4, 0x5], // 3
        vec![0x13], // 4
        vec![0x03, 0x0], // 5
        vec![0x03, 0x0], // 6
        ];
    start_up(&mut base_mem, operations, VMTypes::ECS1);
}

fn main() {
<<<<<<< Updated upstream
    main_main();
=======
    main_main(); 
>>>>>>> Stashed changes
}

fn load_from_file(path: String) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];

    let mut file = File::open(path).expect("Error while opening file");
    let mut whole_file = String::new();
    file.read_to_string(&mut whole_file).expect("Error reading file");

    let fixed = whole_file.replace("\r", "");

    for i in fixed.split("\n") {
        let fixed_line = i.split("//").collect::<Vec<&str>>()[0];
        
        let mut new_opline: Vec<i32> = vec![];

        for word in fixed_line.split(" ") {
            if word != "" && word != "\0" {
                let res = <i32 as FromStr>::from_str(word);
                if res.is_ok() {
                    new_opline.push(res.ok().expect("IDK"));
                }
                else {
                    println!("Can't parse int from str");
                    panic!();
                }
            }
        }
        result.push(new_opline.clone());
    }


    return result;
}


fn start_up(memory: &mut Memory, operations: Vec<Vec<i32>>, _vm: VMTypes) -> VMCode {
    let instructions: Vec<(i32, fn(&mut Memory, Vec<i32>,&mut i32) -> VMCode)> =
    match _vm {
        VMTypes::BCS1 => avm_bcs1::get_instructions_for_bcs1(),
        VMTypes::ECS1 => avm_ecs1::get_instructions_for_ecs1(),
        VMTypes::UACS1=> avm_uacs1::get_instructions_for_uacs1()
    };

    let mut i: i32 = 0;
    while i < operations.len() as i32 {
        
        let line = &operations[i as usize];
        for b in 0..instructions.len() {
            if instructions[b].0.eq(&line[0]) {
                let c: Vec<i32> = line[1..].to_vec();
                instructions[b].1(memory, c, &mut i);
                break;
            }
        }
        i+=1;
    }
    print!("\x1b[0m");

    return VMCode::Success;
}