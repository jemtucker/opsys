pub mod scheduler;
pub mod timer;

mod pit;

use self::scheduler::Scheduler;
use spin::Mutex;


lazy_static! {
    pub static ref SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());
}