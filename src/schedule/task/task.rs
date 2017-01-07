use super::*;

#[derive(Debug)]
pub enum TaskStatus {
    READY,
    RUNNING,
    COMPLETED,
}

#[derive(Debug)]
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
            context: TaskContext::new(),
            status: TaskStatus::READY,
        }
    }

    /// Create a new task with a stack and function to run
    pub fn new(id: u32, stack: usize, fun: fn()) -> Task {
        // TODO The CS and RFLAGS registers are hardcoded with working values. We should work them
        // out properly instead.
        let mut context = TaskContext::new();
        context.cs = 8;
        context.rflags = 582;
        context.rsp = stack as u64 - (4096 * 2);

        // Assign the entry point of the task to the execute function, passing the 'fun' function
        // as an argument in the rdi register.
        context.rip = (execute as *const ()) as u64;
        context.rdi = (fun as *const ()) as u64;

        // Create the task
        Task {
            id: id,
            context: context,
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
}

/// Wraps execution of a function with safe thread termination
fn execute(fun: fn ()) {
    // Execute the function
    fun();

    // Get the 'active' task

    // Set it to not active (COMPLETED)

    // Halt?
}
