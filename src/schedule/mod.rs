pub mod scheduler;
pub mod timer;

mod pit;

use self::scheduler::Scheduler;
use spin::Mutex;

pub static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());