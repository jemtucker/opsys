#[derive(Copy, Clone)]
pub struct Timer {
    counter: usize,
    function: fn(),
}

impl Timer {
    pub fn new(what: fn(), when: usize) -> Timer {
        Timer {
            counter: when,
            function: what,
        }
    }

    // Increment this timer by one tick. Returns true if the timer reached 0
    // and runs its internal function
    pub fn tick(&mut self) -> bool {
        if self.counter % 100 == 0 {
            unsafe {
                ::vga_buffer::print_error(format_args!("Counter: {}", self.counter));
            };
        }

        if self.counter != 0 {

            self.counter -= 1;

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
