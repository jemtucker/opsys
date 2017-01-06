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
    pub error_code: u64,
    pub exception: Exception,
}

bitflags! {
    pub flags PageFaultErrorCode: u64 {
        const PROTECTION_VIOLATION = 1 << 0,
        const CAUSED_BY_WRITE = 1 << 1,
        const USER_MODE = 1 << 2,
        const MALFORMED_TABLE = 1 << 3,
        const INSTRUCTION_FETCH = 1 << 4,
    }
}
