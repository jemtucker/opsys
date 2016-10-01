#[derive(Debug)]
#[repr(C)]
pub struct TaskContext {
    // Registers
    // TODO swap order?
    pub rsp: u64,
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub r8:  u64,
    pub r9:  u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rbp: u64,

    // TODO heap: usize?
    //      stack: usize?
}

pub fn switch_context(old: &TaskContext, new: &TaskContext) {
    // Store the old TaskContext somewhere then replace it with
    // the new one. The interrupt should then automatically return
    // into the new context (fingers crossed)
}
