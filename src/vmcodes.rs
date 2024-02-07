#[repr(i32)]
#[doc = "It can be used as exit code"]
pub enum VMCode {
    Success = 0,
    WrongMemoryIndex = 1,
    //SomeError(i32) = 2,  // USE IT LATER
    UnknownLogicalOperation = 3
}