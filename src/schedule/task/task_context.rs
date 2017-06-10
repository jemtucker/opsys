use core::fmt;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TaskContext {
    /// GP Registers, pushed manually
    // pub rbp: u64,
    // pub r15: u64,
    // pub r14: u64,
    // pub r13: u64,
    // pub r12: u64,
    // pub r11: u64,
    // pub r10: u64,
    // pub r9: u64,
    // pub r8: u64,
    // pub rsi: u64,
    // pub rdi: u64,
    // pub rdx: u64,
    // pub rcx: u64,
    // pub rbx: u64,
    // pub rax: u64,

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

    /// Rest of trap frame pushed by CPU interrupt
    /// Instruction pointer
    pub rip: u64,

    /// Code segment
    pub cs: u64,

    /// Flags register
    pub rflags: u64,

    /// Stack pointer
    pub rsp: u64,

    /// Stack segment
    pub ss: u64,
}

impl TaskContext {
    pub fn new() -> TaskContext {
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
            rip: 0,
            cs: 0,
            rflags: 0,
            rsp: 0,
            ss: 0,
        }
    }
}

impl fmt::Debug for TaskContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Hex(u64);
        impl fmt::Debug for Hex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:#x}", self.0)
            }
        }

        let mut s = f.debug_struct("TaskContext");
        s.field("rax", &Hex(self.rax));
        s.field("rbx", &Hex(self.rbx));
        s.field("rcx", &Hex(self.rcx));
        s.field("rdx", &Hex(self.rdx));
        s.field("rdi", &Hex(self.rdi));
        s.field("rsi", &Hex(self.rsi));
        s.field("r8", &Hex(self.r8));
        s.field("r9", &Hex(self.r9));
        s.field("r10", &Hex(self.r10));
        s.field("r11", &Hex(self.r11));
        s.field("r12", &Hex(self.r12));
        s.field("r13", &Hex(self.r13));
        s.field("r14", &Hex(self.r14));
        s.field("r15", &Hex(self.r15));
        s.field("rbp", &Hex(self.rbp));
        s.field("rip", &Hex(self.rip));
        s.field("cs", &self.cs);
        s.field("rflags", &Hex(self.rflags));
        s.field("rsp", &Hex(self.rsp));
        s.field("ss", &self.ss);
        s.finish()
    }
}
