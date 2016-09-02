use super::timer::Timer;
use super::pit::Pit;

pub struct Scheduler {
    timer: Pit
}

impl Scheduler {
    pub fn new() -> Scheduler {
        let mut s = Scheduler {
            timer: Pit::new()
        };

        s.timer.run_in(10, Scheduler::schedule);

        s
    }

    pub fn get_timer_mut(&mut self) -> &mut Timer {
        &mut self.timer
    }

    fn schedule() {
        kprintln!("BOOM");
    }
}