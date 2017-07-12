use alloc::linked_list::LinkedList;

use super::task::Task;
use super::task::TaskStatus;
use super::task::TaskContext;

use kernel::kget;
use memory::MemoryManager;

pub struct Scheduler {
    inactive_tasks: LinkedList<Task>,
    active_task: Option<Task>,
    task_count: u32,
    last_resched: usize,
}

/// Scheduler for the kernel. Manages scheduling of tasks and timers
impl Scheduler {
    /// Creates a new scheduler with empty lists of timers and tasks.
    pub fn new() -> Scheduler {
        Scheduler {
            inactive_tasks: LinkedList::new(),
            active_task: Some(Task::default()),
            task_count: 1,
            last_resched: 0,
        }
    }

    /// Create a new task to be scheduled.
    pub fn new_task(&mut self, memory_manager: &mut MemoryManager, func: fn()) {
        let stack = memory_manager.allocate_stack();

        self.inactive_tasks.push_front(
            Task::new(self.task_count, stack, func),
        );

        self.task_count += 1;
    }

    /// Schedule the next task.
    ///
    /// Choses the next task with status != `TaskStatus::COMPLETED` and switches its context with
    /// that of the currently active task.
    pub fn schedule(&mut self, active_ctx: &mut TaskContext) {
        if self.inactive_tasks.len() == 0 {
            return;
        }

        // Choose the next task and remove from list.
        // TODO List is definitely not the best structure to use here, pop_back is O(n). Research
        // alternative rust collections...
        let new_task = self.inactive_tasks.pop_back().unwrap();
        let mut old_task = self.active_task.take().unwrap();

        // Swap the contexts
        // Copy the active context to save it
        old_task.set_context(active_ctx);
        *active_ctx = *new_task.get_context();

        // Update the schedulers internal references and store the initial task back into the
        // inactive_tasks list if it is not yet finished.
        self.active_task = Some(new_task);
        if old_task.get_status() != TaskStatus::COMPLETED {
            self.inactive_tasks.push_front(old_task);
        }

        // TODO some sort of task cleanup

        // Update the last_resched time
        self.update_last_resched();
    }

    /// Get a mutable reference to the current active task.
    pub fn get_active_task_mut(&mut self) -> Option<&mut Task> {
        self.active_task.as_mut()
    }

    /// Returns true if a reschedule is needed
    ///
    /// Returns true if the last reschedule was over 10 milliseconds ago.
    pub fn need_resched(&self) -> bool {
        let clock = unsafe { &mut *kget().clock.get() };
        let now = clock.now();
        (now - self.last_resched) > 10
    }

    /// Update `last_resched` to now.
    fn update_last_resched(&mut self) {
        let clock = unsafe { &mut *kget().clock.get() };
        self.last_resched = clock.now();
    }
}
