use super::*;

pub enum TaskStatus {
    RUNNING,
    WAITING,
    READY,
}

pub struct Task {
    id: u32,
    context: TaskContext,
    status: TaskStatus,
}

impl Task {
    /// Create a new Task with an id 'id'. This task will be initialized with status READY.
    pub fn default() -> Task {
        Task {
            id: 0,
            context: TaskContext::new(0, 0),
            status: TaskStatus::READY,
        }
    }

    pub fn new(id: u32, stack: usize, func: fn()) -> Task {
        let rip = (func as *const ()) as u64;
        Task {
            id: id,
            context: TaskContext::new(stack as u64 + (4096 * 2), rip),
            status: TaskStatus::READY,
        }
    }

    /// Update this tasks context
    pub fn set_context(&mut self, context: &TaskContext) {
        self.context = *context;
    }

    /// Return an immutable reference to this tasks context
    pub fn get_context(&self) -> &TaskContext {
        &self.context
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}
