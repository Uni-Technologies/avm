use crate::vmcodes::VMCode;
#[allow(unused)]
pub enum VMTypes {
    BCS1,
    ECS1
}

pub struct Memory {
    pub itself: [i32; 0xFFFF]
}

impl Memory {
    pub fn get_by(&mut self, index: usize) -> i32{
        return self.itself[index];
    }
    pub fn set_by(&mut self, index: usize, value: i32) -> VMCode {
        if index < 0xFFFF {
            self.itself[index] = value;
            return VMCode::Success;
        }
        else {
            return VMCode::WrongMemoryIndex;
        }
    }
}

// SIGNATURE OF EVERY INSTRUCTION: fn(&mut Memory, Vec<i32>, &mut i32) -> VMCode