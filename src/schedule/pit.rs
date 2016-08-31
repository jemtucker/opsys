use schedule::timer::Timer;

pub struct Pit {
    tick_count: usize,
}

impl Pit {
    pub const fn new() -> Pit {
        Pit {
            tick_count: 0
        }
    }
}

impl Timer for Pit {
    fn tick(&mut self) {
        self.tick_count += 1;
    }

    fn get_ticks(&self) -> usize {
        self.tick_count
    }
}