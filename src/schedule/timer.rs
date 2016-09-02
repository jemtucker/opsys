pub trait Timer {
    // Increment this timer by one tick
    fn tick(&mut self);

    // Get the current tick count.
    fn run_in(&mut self, interval: usize, func: fn());
}