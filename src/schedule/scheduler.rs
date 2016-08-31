use super::timer::Timer;
use super::pit::Pit;

pub struct Scheduler {
    timer: Pit
}

impl Scheduler {
    pub const fn new() -> Scheduler {
        Scheduler {
            timer: Pit::new()
        }
    }

    pub fn get_timer_mut(&mut self) -> &mut Timer {
        &mut self.timer
    }
}