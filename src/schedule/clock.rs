pub struct Clock {
    milliseconds: usize,
}

impl Clock {
    /// Creates a new clock with a starting time of 0 milliseconds.
    pub fn new() -> Clock {
        Clock { milliseconds: 0 }
    }

    /// Increments the time by one unit.
    /// Returns the new time.
    pub fn tick(&mut self) -> usize {
        self.milliseconds += 1;
        self.milliseconds
    }

    /// Returns the number of milliseconds on this clock
    pub fn now(&self) -> usize {
        self.milliseconds
    }
}
