#[derive(Debug)]
#[repr(C)]
pub struct Exception {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}

#[derive(Debug)]
#[repr(C)]
pub struct ExceptionWithError {
    pub error_code: u32,
    pub exception: Exception,
}