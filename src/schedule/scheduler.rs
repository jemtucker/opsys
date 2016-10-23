use collections::linked_list::LinkedList;

use super::clock::Clock;
use super::timer::Timer;
use super::task::Task;
use super::task::TaskContext;


pub struct Scheduler {
    timers: LinkedList<Timer>,
    inactive_tasks: LinkedList<Task>,
    active_task: Option<Task>,
    task_count: u32,
    clock: Clock,
}

impl Scheduler {
    /// Creates a new scheduler with empty lists of timers and tasks.
    pub fn new() -> Scheduler {
        // TODO Push the current "task" here in the constructor. This will ensure we always
        // have an active task.
        Scheduler {
            timers: LinkedList::new(),
            inactive_tasks: LinkedList::new(),
            active_task: Some(Task::new(0)),
            task_count: 1,
            clock: Clock::new(),
        }
    }

    /// Create a new task to be scheduled.
    pub fn new_task(&mut self, func: fn()) {
        self.task_count += 1;
        self.inactive_tasks.push_front(Task::new(self.task_count));
    }

    /// Schedule an event to be fired at a future time
    pub fn new_timer(&mut self, what: fn(), when: usize) {
        self.timers.push_front(Timer::new(what, when));
    }

    /// Increments the timer on all scheduled events.
    pub fn tick(&mut self, active_ctx: &mut TaskContext) {
        let time = self.clock.tick();
        self.handle_timers(time);

        if self.inactive_tasks.len() == 0 {
            return;
        }

        // Choose the next task and remove from list.
        // TODO List is definitely not the best structure to use here, pop_back is O(n). Research
        // alternative rust collections...
        let mut new_task = self.inactive_tasks.pop_back().unwrap();
        let mut old_task = self.active_task.take().unwrap();

        // Swap the contexts
        // Copy the active context to save it
        old_task.set_context(active_ctx);
        *active_ctx = *new_task.get_context();

        // Update the schedulers internal references and store the initial
        // task back into the inactive_tasks list
        self.active_task = Some(new_task);
        self.inactive_tasks.push_front(old_task);
    }

    /// Tick all the timers and prune any expired ones.
    fn handle_timers(&mut self, _: usize) {
        // TODO Rework the timers to all use the centeral time. A task should run
        // if its when <= now/
        let mut new_timers = LinkedList::new();

        for _ in 0..self.timers.len() {
            let mut timer = self.timers.pop_front().unwrap();
            if !timer.tick() {
                new_timers.push_front(timer);
            }
        }

        self.timers = new_timers;
    }
}
