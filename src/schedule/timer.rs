#[derive(Copy, Clone)]
pub struct Timer {
    counter: usize,
    function: fn()
}

impl Timer {
    pub fn new(f: fn(), when: usize) -> Timer {
        Timer {
            counter: when,
            function: f
        }
    }

    // Increment this timer by one tick. Returns true if the timer reached 0
    // and runs its internal function
    pub fn tick(&mut self) -> bool {
        self.counter -= 1;

        let finished = self.counter < 1;

        if finished {
            (self.function)();
        }

        finished
    }
}