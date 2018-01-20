use alloc::linked_list::LinkedList;
use alloc::boxed::Box;
use kernel::kget;

use schedule::task::TID_BOTTOMHALFD;
use schedule::task::TaskStatus;

use spin::Mutex;

/// Main bottomhalfd task
///
/// Loops forever, iterating over all queued `BottomHalf` tasks and executing them in series.
pub fn execute() {
    loop {
        kprintln!("Enter loop BH::execute");
        let scheduler = unsafe { &mut *kget().scheduler.get() };
        let bh_manager = scheduler.bh_manager();

        // Execute all waiting bottom halves
        bh_manager.execute_all();

        kprintln!("executed all BH");

        // Set the current task status to WAITING and set the scheduler to reschedule
        // at its next opportunity.
        scheduler.set_task_status(TID_BOTTOMHALFD, TaskStatus::WAITING);
        scheduler.set_need_resched();

        kprintln!("Halting");

        // Sleep until the next interrupt
        halt!();

        kprintln!("Awakened");
    }
}

/// A task to execute as the bottom half of an interrupt handler
pub trait BottomHalf {
    /// Exectute the work for this bottom half
    fn execute(&mut self);
}

/// A first in first out queue of `BottomHalf` tasks
struct BottomHalfQueue {
    queue: LinkedList<Box<BottomHalf>>,
}

impl BottomHalfQueue {
    /// Construct a new `BottomHalfQueue`
    pub fn new() -> BottomHalfQueue {
        BottomHalfQueue { queue: LinkedList::new() }
    }

    /// Returns the length of the queue
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Push a `BottomHalf` onto the back of the queue
    pub fn push(&mut self, bh: Box<BottomHalf>) {
        self.queue.push_back(bh);
    }

    /// Pop a `BottomHalf` from the front of the queue
    ///
    /// If the queue is empty this will return `None`
    pub fn pop(&mut self) -> Option<Box<BottomHalf>> {
        self.queue.pop_front()
    }
}

/// Manager of `BottomHalf` queues.
///
/// Provides thread safety to the `BottomHalf` processing.
pub struct BottomHalfManager {
    queue: Mutex<BottomHalfQueue>,
}

impl BottomHalfManager {
    /// Construct a new `BottomHalfManager`
    pub fn new() -> BottomHalfManager {
        BottomHalfManager { queue: Mutex::new(BottomHalfQueue::new()) }
    }

    /// Push a `BottomHalf` onto the back of the queue
    pub fn add_bh(&self, bh: Box<BottomHalf>) {
        let len = {
            let mut q = self.queue.lock();
            q.push(bh);
            q.len()
        };

        // If this is the first task added then tell the scheduler to schedule the bottom half
        // thread next time it runs
        if len == 1 {
            let scheduler = unsafe { &mut *kget().scheduler.get() };
            scheduler.set_task_status(TID_BOTTOMHALFD, TaskStatus::READY);
            scheduler.set_need_resched();
        }
    }

    /// Execute all currently queued `BottomHalf` tasks
    pub fn execute_all(&self) {
        loop {
            match self.queue.lock().pop() {
                Some(mut bh) => bh.execute(),
                None => break,
            }
        }
    }
}
