use super::timer::Timer;
use collections::linked_list::LinkedList;

pub struct Scheduler {
    timers: LinkedList<Timer>,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler { timers: LinkedList::new() }
    }

    pub fn tick(&mut self) {
        let mut new_timers = LinkedList::new();

        for _ in 0..self.timers.len() {
            let mut timer = self.timers.pop_front().unwrap();
            if !timer.tick() {
                new_timers.push_front(timer);
            }
        }

        self.timers = new_timers;
    }

    pub fn schedule(&mut self, what: fn(), when: usize) {
        self.timers.push_front(Timer::new(what, when));
    }
}
