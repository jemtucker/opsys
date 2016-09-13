use super::timer::Timer;
use collections::vec::Vec;

pub struct Scheduler {
    timers: Vec<Timer>
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            timers: Vec::new()
        }
    }

    pub fn tick(&mut self) {
        let mut new_timers: Vec<Timer> = Vec::new();

        for mut timer in self.timers.iter_mut() {
            if !timer.tick() {
                new_timers.push(timer.clone());
            }
        }

        self.timers = new_timers;
    }

    pub fn schedule(&mut self, what: fn(), when: usize) {
        self.timers.push(Timer::new(what, when));
    }
}