use super::*;

pub struct Task {
    id: u32,
    regs: TaskContext,
    status: TaskStatus,
}
