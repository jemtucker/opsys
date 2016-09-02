use schedule::timer::Timer;
use collections::btree_map::BTreeMap;

pub struct Pit {
    tick_count: usize,
    events: BTreeMap<usize, fn()>, // May be faster with ordered list?
}

impl Pit {
    pub fn new() -> Pit {
        Pit {
            tick_count: 0,
            events: BTreeMap::new(),
        }
    }
}

impl Timer for Pit {
    fn tick(&mut self) {
        self.tick_count += 1;

        let now = self.tick_count;

        match self.events.remove(&now) {
            Some(func) => func(),
            None => {}
        }
    }

    fn run_in(&mut self, interval: usize, func: fn()) {
        self.events.insert(self.tick_count + interval, func);
    }
}