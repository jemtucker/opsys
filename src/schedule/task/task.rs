use super::*;

use memory::Stack;

/// Status of a kernel task
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// Task is ready to run or running
    READY,
    /// Task is waiting and should not yet run
    WAITING,
    /// Task is completed and ready to be destroyed
    COMPLETED,
}

/// Priority of a kernel task
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TaskPriority {
    /// Highest priority, meant for interrupt handlers
    IRQ,
    /// Normal priority, meant for all other kernel tasks
    NORMAL,
}

/// A task to be run by the kernel.
///
/// Currently this represents only a kernel thread.
#[derive(Debug)]
pub struct Task {
    id: u32,
    context: TaskContext,
    status: TaskStatus,
    priority: TaskPriority,
    stack: Stack,
}

impl Task {
    /// Create a new Task with an id 'id'.
    ///
    /// This task will be initialized with status `READY` and priority `NORMAL`.
    pub fn default(id: u32) -> Task {
        Task {
            id: id,
            context: TaskContext::new(),
            status: TaskStatus::READY,
            priority: TaskPriority::NORMAL,
            stack: Stack {
                start_address: 0,
                size: 0,
            },
        }
    }

    /// Create a new task with a stack and function to run
    ///
    /// The CS and RFLAGS registers are hardcoded with working values. RIP is set to the address of
    /// the `execute` function with the first argument (RDI) holding the adress of `fun`.
    pub fn new(
        id: u32,
        stack: Stack,
        fun: fn(),
        priority: TaskPriority,
        status: TaskStatus,
    ) -> Task {
        let mut context = TaskContext::new();
        context.cs = 8;
        context.rflags = 582;
        context.rsp = stack.top() as u64;

        // Assign the entry point of the task to the execute function, passing the 'fun' function
        // as an argument in the rdi register.
        context.rip = (execute as *const ()) as u64;
        context.rdi = (fun as *const ()) as u64;

        // Create the task
        Task {
            id: id,
            context: context,
            status: status,
            priority: priority,
            stack: stack,
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

    /// Change the status of this Task
    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }

    /// Return the current Task status
    pub fn get_status(&self) -> TaskStatus {
        self.status
    }

    /// Return the current Task priority
    pub fn get_priority(&self) -> TaskPriority {
        self.priority
    }

    /// Return the id of the Task
    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for Task {
    fn drop(&mut self) {
        kprintln!("Task::Drop");

        use kernel::kget;
        let mm = unsafe { &mut *kget().memory_manager.get() };

        // TODO is this safe? I have a feeling that the stack will still be used after this drop?
        mm.deallocate_stack(&self.stack);
    }
}

/// Wraps execution of a function with safe thread termination
fn execute(fun: fn()) {
    use kernel::kget;

    // Execute the function
    fun();

    // Get the 'active' task
    let scheduler = unsafe { &mut *kget().scheduler.get() };
    let task = scheduler.get_active_task_mut().unwrap();

    // Set it to not active (COMPLETED)
    task.set_status(TaskStatus::COMPLETED);

    // TODO set scheduler to reschedule and halt! instead of hang!. Consider unreachable! after
    // the halt
    hang!();
}
