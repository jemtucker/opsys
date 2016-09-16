#[derive(Copy, Clone)]
pub struct Timer {
    counter: usize,
    function: fn()
}

impl Timer {
    pub fn new(what: fn(), when: usize) -> Timer {
        Timer {
            counter: when,
            function: what
        }
    }

    // Increment this timer by one tick. Returns true if the timer reached 0
    // and runs its internal function
    pub fn tick(&mut self) -> bool {
        if self.counter != 0 {
            //self.counter -= 1; // This line causes a page fault at the moment?

            let finished = self.counter < 1;
            if finished {
                (self.function)();
            }

            finished
        } else {
            false
        }
    }
}