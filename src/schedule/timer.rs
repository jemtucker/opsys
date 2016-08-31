pub trait Timer {
    // Increment this timer by one tick
    fn tick(&mut self);

    // Get the current tick count.
    fn get_ticks(&self) -> usize;
}