#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TaskContext {
    // GP Registers, pushed manually
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rbp: u64,

    // Rest of trap frame pushed by CPU interrupt
    /// Instruction pointer
    pub rip: u64,

    /// Code segment
    pub cs: u64,

    /// Flags register
    pub rflags: u64,

    /// Stack pointer
    pub rsp: u64,
}

impl TaskContext {
    pub fn new(rsp: u64, rip: u64) -> TaskContext {
        TaskContext {
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rdi: 0,
            rsi: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rbp: 0,
            rip: rip,
            cs: 8, // TODO calculate this properly
            rflags: 582, // TODO work out the proper flags to start with
            rsp: rsp,
        }
    }
}
