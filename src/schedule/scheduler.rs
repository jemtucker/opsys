use alloc::linked_list::LinkedList;

use super::task::{Task, TaskStatus, TaskContext, TaskPriority};

use kernel::kget;
use memory::MemoryManager;
use interrupts::bottom_half;

/// Scheduler for the kernel. Manages scheduling of tasks and timers
pub struct Scheduler {
    inactive_tasks: LinkedList<Task>,
    active_task: Option<Task>,
    task_count: u32,
    last_resched: usize,
}

impl Scheduler {
    /// Creates a new scheduler
    ///
    /// The currently active task is created along with a single, currently `WAITING`, task of
    /// priority `IRQ`.
    pub fn new(memory_manager: &mut MemoryManager) -> Scheduler {
        let mut inactive_tasks = LinkedList::new();

        // Create the kernel bottom_half IRQ processing thread
        let stack = memory_manager.allocate_stack();
        inactive_tasks.push_front(Task::new(
            1,
            stack,
            bottom_half::execute,
            TaskPriority::IRQ,
            TaskStatus::WAITING,
        ));

        Scheduler {
            inactive_tasks: inactive_tasks,
            active_task: Some(Task::default()),
            task_count: 2,
            last_resched: 0,
        }
    }

    /// Create a new task to be scheduled.
    pub fn new_task(&mut self, memory_manager: &mut MemoryManager, func: fn()) {
        let stack = memory_manager.allocate_stack();

        self.inactive_tasks.push_front(Task::new(
            self.task_count,
            stack,
            func,
            TaskPriority::NORMAL,
            TaskStatus::READY,
        ));

        self.task_count += 1;
    }

    /// Schedule the next task.
    ///
    /// Choses the next task with status != `TaskStatus::COMPLETED` and switches its context with
    /// that of the currently active task.
    pub fn schedule(&mut self, active_ctx: &mut TaskContext) {

        // Optimization - return early if nothing to do
        if self.inactive_tasks.len() == 0 {
            return;
        }

        // First look for active high priority tasks first, if none of these exist then look for
        // normal priority tasks.
        let new_task = match self.next_task(TaskPriority::IRQ) {
            Some(t) => t,
            None => self.next_task(TaskPriority::NORMAL).unwrap(),
        };

        let mut old_task = self.active_task.take().unwrap();

        // Swap the contexts
        // Copy the active context to save it
        old_task.set_context(active_ctx);
        *active_ctx = *new_task.get_context();

        // Update the schedulers internal references and store the initial task back into the
        // inactive_tasks list if it is not yet finished. By not restoring COMPLETED tasks here
        // we force cleanup of COMPLETED tasks.
        self.active_task = Some(new_task);
        if old_task.get_status() != TaskStatus::COMPLETED {
            self.inactive_tasks.push_back(old_task);
        }

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

    /// Find the next task with priority matching `priority`
    fn next_task(&mut self, priority: TaskPriority) -> Option<Task> {
        let mut i = 0;
        let mut found = false;

        for ref t in self.inactive_tasks.iter() {
            if t.get_priority() != priority || t.get_status() != TaskStatus::READY {
                // On to the next, this is not suitable
                i += 1;
            } else {
                found = true;
                break;
            }
        }

        if found {
            // Split inactive_tasks, remove the task we found, then re-merge the two lists
            let mut remainder = self.inactive_tasks.split_off(i);
            let next_task = remainder.pop_front();

            // Merge the lists
            loop {
                match remainder.pop_front() {
                    Some(t) => self.inactive_tasks.push_back(t),
                    None => break,
                }
            }

            next_task
        } else {
            None
        }
    }
}
